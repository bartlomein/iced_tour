use iced::keyboard;
use iced::widget::canvas::{self, Frame, Geometry, Path};
use iced::widget::{canvas as canvas_widget, stack, Space};
use iced::{mouse, Color, Element, Event, Length, Point, Rectangle, Theme};

use crate::card::tour_card;
use crate::manager::{TourManager, TourManagerMessage};
use crate::state::{TourMessage, TourState};
use crate::step::CardPosition;
use crate::theme::TourTheme;

/// Call this in your `view()` function. Add the returned Element to your
/// existing `stack![]`. Returns invisible `Space` when the tour is inactive,
/// so it's safe to always include.
///
/// Keyboard navigation is built-in when the tour is active:
/// - **Escape** — skip the tour
/// - **Right arrow** / **Enter** — next step
/// - **Left arrow** — previous step
///
/// ```ignore
/// // In your view() function:
/// stack![
///     self.view_editor(),
///     tour_overlay(&self.tour_state, &TourTheme::dark(), Message::Tour),
/// ]
/// ```
pub fn tour_overlay<'a, Message: Clone + 'a>(
    state: &'a TourState,
    theme: &'a TourTheme,
    on_message: impl Fn(TourMessage) -> Message + 'a,
) -> Element<'a, Message> {
    if !state.is_active() {
        return Space::new().width(0).height(0).into();
    }

    let step = match state.current_step() {
        Some(s) => s,
        None => return Space::new().width(0).height(0).into(),
    };

    let backdrop_color = theme.backdrop_color;
    let cutout_padding = theme.cutout_padding;
    let cutout_border_radius = theme.cutout_border_radius;
    let target = state.animated_target(std::time::Instant::now());

    // Canvas handles drawing (backdrop + cutout) and keyboard/mouse input.
    // It produces TourMessage directly.
    let allow_escape = theme.allow_escape;

    let backdrop: Element<'_, TourMessage> = canvas_widget(BackdropProgram {
        backdrop_color,
        cutout_padding,
        cutout_border_radius,
        target,
        allow_escape,
    })
    .width(Length::Fill)
    .height(Length::Fill)
    .into();

    // Build card with TourMessage as the message type (using identity mapper)
    let card = tour_card(state, theme, &std::convert::identity);
    let card_positioned: Element<'_, TourMessage> =
        position_card(card, target, step.position(), cutout_padding);

    // Assemble as TourMessage stack, then map everything to app's Message type
    let inner: Element<'_, TourMessage> = stack![backdrop, card_positioned]
        .width(Length::Fill)
        .height(Length::Fill)
        .into();

    inner.map(on_message)
}

/// Overlay for a `TourManager`. Renders the currently active tour (if any).
///
/// This is the multi-tour equivalent of `tour_overlay()`. It delegates to
/// the active tour's state and maps messages through `TourManagerMessage`.
///
/// ```ignore
/// stack![
///     self.view_editor(),
///     tour_manager_overlay(&self.tours, &self.tour_theme, Message::Tour),
/// ]
/// ```
pub fn tour_manager_overlay<'a, Message: Clone + 'a>(
    manager: &'a TourManager,
    theme: &'a TourTheme,
    on_message: impl Fn(TourManagerMessage) -> Message + 'a,
) -> Element<'a, Message> {
    match manager.active_state() {
        Some(state) => tour_overlay(state, theme, move |msg| on_message(TourManagerMessage(msg))),
        None => Space::new().width(0).height(0).into(),
    }
}

/// Position the card relative to the target rectangle.
fn position_card<'a, Message: 'a>(
    card: Element<'a, Message>,
    target: Option<Rectangle>,
    position: CardPosition,
    cutout_padding: f32,
) -> Element<'a, Message> {
    use iced::widget::container;
    use iced::Alignment;

    match target {
        None => {
            // No target: center the card on screen
            container(card)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .into()
        }
        Some(rect) => {
            // Position card adjacent to the target
            let gap = 12.0;
            let padded_rect = Rectangle {
                x: rect.x - cutout_padding,
                y: rect.y - cutout_padding,
                width: rect.width + cutout_padding * 2.0,
                height: rect.height + cutout_padding * 2.0,
            };

            let resolved = match position {
                CardPosition::Bottom => CardPosition::Bottom,
                CardPosition::Top => CardPosition::Top,
                CardPosition::Left => CardPosition::Left,
                CardPosition::Right => CardPosition::Right,
                CardPosition::Auto => {
                    // Without knowing viewport size at layout time, we default
                    // to Bottom which works well for most UI layouts. For precise
                    // control, use an explicit CardPosition on each step.
                    CardPosition::Bottom
                }
            };

            match resolved {
                CardPosition::Bottom => {
                    let top = padded_rect.y + padded_rect.height + gap;
                    let left = padded_rect.x + padded_rect.width / 2.0 - 175.0;
                    positioned_container(card, left.max(8.0), top)
                }
                CardPosition::Top => {
                    // We estimate card height at ~200px. Position above.
                    let top = (padded_rect.y - gap - 200.0).max(8.0);
                    let left = padded_rect.x + padded_rect.width / 2.0 - 175.0;
                    positioned_container(card, left.max(8.0), top)
                }
                CardPosition::Right => {
                    let left = padded_rect.x + padded_rect.width + gap;
                    let top = padded_rect.y + padded_rect.height / 2.0 - 100.0;
                    positioned_container(card, left, top.max(8.0))
                }
                CardPosition::Left => {
                    let left = (padded_rect.x - gap - 350.0).max(8.0);
                    let top = padded_rect.y + padded_rect.height / 2.0 - 100.0;
                    positioned_container(card, left, top.max(8.0))
                }
                CardPosition::Auto => unreachable!(),
            }
        }
    }
}

fn positioned_container<'a, Message: 'a>(
    content: Element<'a, Message>,
    left: f32,
    top: f32,
) -> Element<'a, Message> {
    use iced::widget::{column, row, Space};

    // Use spacer rows/columns for absolute positioning within a fill container
    column![
        Space::new().height(Length::Fixed(top)),
        row![Space::new().width(Length::Fixed(left)), content,]
    ]
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

// --- Canvas Program for the backdrop ---

struct BackdropProgram {
    backdrop_color: Color,
    cutout_padding: f32,
    cutout_border_radius: f32,
    target: Option<Rectangle>,
    allow_escape: bool,
}

impl canvas::Program<TourMessage, Theme> for BackdropProgram {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        event: &Event,
        _bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<TourMessage>> {
        match event {
            // Keyboard: Escape, arrows, Enter
            Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => {
                use keyboard::key::Named;
                use keyboard::Key;
                match key {
                    Key::Named(Named::Escape) if self.allow_escape => {
                        Some(canvas::Action::publish(TourMessage::Skip).and_capture())
                    }
                    Key::Named(Named::ArrowRight) | Key::Named(Named::Enter) => {
                        Some(canvas::Action::publish(TourMessage::Next).and_capture())
                    }
                    Key::Named(Named::ArrowLeft) => {
                        Some(canvas::Action::publish(TourMessage::Back).and_capture())
                    }
                    _ => None,
                }
            }
            // Mouse click on backdrop (not on card — card handles its own clicks)
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if cursor.is_over(_bounds) {
                    Some(canvas::Action::publish(TourMessage::BackdropClicked).and_capture())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        match self.target {
            None => {
                // Full backdrop, no cutout
                let path = Path::rectangle(Point::ORIGIN, bounds.size());
                frame.fill(&path, self.backdrop_color);
            }
            Some(rect) => {
                // Cutout using EvenOdd fill rule
                let pad = self.cutout_padding;
                let inner_x = rect.x - pad;
                let inner_y = rect.y - pad;
                let inner_w = rect.width + pad * 2.0;
                let inner_h = rect.height + pad * 2.0;
                let _r = self.cutout_border_radius;

                let cutout_path = Path::new(|p| {
                    // Outer rectangle (clockwise)
                    p.move_to(Point::new(0.0, 0.0));
                    p.line_to(Point::new(bounds.width, 0.0));
                    p.line_to(Point::new(bounds.width, bounds.height));
                    p.line_to(Point::new(0.0, bounds.height));
                    p.close();

                    // Inner rectangle (counter-clockwise for cutout)
                    // Using rounded corners via arc_to for border radius
                    if _r > 0.0 {
                        let r = _r.min(inner_w / 2.0).min(inner_h / 2.0);
                        p.move_to(Point::new(inner_x + r, inner_y));
                        // Top-right corner
                        p.line_to(Point::new(inner_x + inner_w - r, inner_y));
                        p.arc_to(
                            Point::new(inner_x + inner_w, inner_y),
                            Point::new(inner_x + inner_w, inner_y + r),
                            r,
                        );
                        // Bottom-right corner
                        p.line_to(Point::new(inner_x + inner_w, inner_y + inner_h - r));
                        p.arc_to(
                            Point::new(inner_x + inner_w, inner_y + inner_h),
                            Point::new(inner_x + inner_w - r, inner_y + inner_h),
                            r,
                        );
                        // Bottom-left corner
                        p.line_to(Point::new(inner_x + r, inner_y + inner_h));
                        p.arc_to(
                            Point::new(inner_x, inner_y + inner_h),
                            Point::new(inner_x, inner_y + inner_h - r),
                            r,
                        );
                        // Top-left corner
                        p.line_to(Point::new(inner_x, inner_y + r));
                        p.arc_to(
                            Point::new(inner_x, inner_y),
                            Point::new(inner_x + r, inner_y),
                            r,
                        );
                        p.close();
                    } else {
                        p.move_to(Point::new(inner_x, inner_y));
                        p.line_to(Point::new(inner_x, inner_y + inner_h));
                        p.line_to(Point::new(inner_x + inner_w, inner_y + inner_h));
                        p.line_to(Point::new(inner_x + inner_w, inner_y));
                        p.close();
                    }
                });

                use iced::widget::canvas::fill::Rule;
                use iced::widget::canvas::Fill;
                use iced::widget::canvas::Style;
                frame.fill(
                    &cutout_path,
                    Fill {
                        style: Style::Solid(self.backdrop_color),
                        rule: Rule::EvenOdd,
                    },
                );
            }
        }

        vec![frame.into_geometry()]
    }
}
