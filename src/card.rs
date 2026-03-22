use iced::widget::canvas::{self, Frame, Geometry, Path};
use iced::widget::{button, column, container, row, text, Canvas, Space};
use iced::{mouse, Background, Border, Color, Element, Length, Point, Rectangle, Theme};

use crate::state::{TourMessage, TourState};
use crate::theme::TourTheme;

const CARD_MAX_WIDTH: f32 = 350.0;
const CARD_PADDING: f32 = 20.0;
const DOT_SIZE: f32 = 8.0;
const DOT_SPACING: f32 = 6.0;

/// Build the tooltip card Element with title, description, dots, and buttons.
pub fn tour_card<'a, Message: Clone + 'a>(
    state: &'a TourState,
    theme: &'a TourTheme,
    on_message: &(impl Fn(TourMessage) -> Message + 'a),
) -> Element<'a, Message> {
    let step = match state.current_step() {
        Some(s) => s,
        None => return Space::new().width(0).height(0).into(),
    };

    let (current, total) = state.step_index();

    // Title
    let title = text(step.title())
        .size(theme.title_size)
        .color(theme.title_color)
        .font(theme.title_font)
        .width(Length::Fill);

    // Description
    let description = text(step.description())
        .size(theme.description_size)
        .color(theme.description_color)
        .font(theme.description_font)
        .width(Length::Fill);

    // Dot indicators
    let dots = dot_indicators(current, total, theme);

    // Navigation buttons
    let nav = navigation_buttons(state, theme, on_message);

    // Bottom row: dots on left, buttons on right
    let bottom_row = row![dots, Space::new().width(Length::Fill), nav]
        .spacing(8)
        .align_y(iced::Alignment::Center);

    // Card content
    let card_content = column![
        title,
        Space::new().height(8),
        description,
        Space::new().height(16),
        bottom_row,
    ]
    .width(Length::Fill);

    // Skip button below the card
    let skip_text = text("Skip tour").size(12.0).color(Color {
        a: 0.5,
        ..theme.description_color
    });

    let skip_btn = button(skip_text)
        .on_press(on_message(TourMessage::Skip))
        .style(|_, _| button::Style {
            background: None,
            text_color: Color::TRANSPARENT,
            border: Border::default(),
            shadow: iced::Shadow::default(),
            snap: false,
        })
        .padding(4);

    let card_with_skip = column![
        container(card_content)
            .width(Length::Fixed(CARD_MAX_WIDTH))
            .padding(CARD_PADDING)
            .style(move |_| card_style(theme)),
        container(skip_btn)
            .width(Length::Fixed(CARD_MAX_WIDTH))
            .align_x(iced::Alignment::End)
            .padding(4),
    ];

    card_with_skip.into()
}

fn card_style(theme: &TourTheme) -> container::Style {
    container::Style {
        background: Some(Background::Color(theme.card_background)),
        border: Border {
            radius: theme.card_border_radius.into(),
            width: 1.0,
            color: Color {
                a: 0.15,
                ..theme.title_color
            },
        },
        shadow: iced::Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
            offset: iced::Vector::new(0.0, 4.0),
            blur_radius: 16.0,
        },
        snap: false,
        ..Default::default()
    }
}

fn dot_indicators<'a, Message: 'a>(
    current: usize,
    total: usize,
    theme: &TourTheme,
) -> Element<'a, Message> {
    let active_color = theme.dot_active_color;
    let inactive_color = theme.dot_inactive_color;

    Canvas::new(DotProgram {
        current,
        total,
        active_color,
        inactive_color,
    })
    .width(Length::Fixed(
        (DOT_SIZE + DOT_SPACING) * total as f32 - DOT_SPACING,
    ))
    .height(Length::Fixed(DOT_SIZE))
    .into()
}

struct DotProgram {
    current: usize,
    total: usize,
    active_color: Color,
    inactive_color: Color,
}

impl<Message> canvas::Program<Message, Theme> for DotProgram {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let radius = DOT_SIZE / 2.0;

        for i in 0..self.total {
            let x = (DOT_SIZE + DOT_SPACING) * i as f32 + radius;
            let y = radius;
            let color = if i == self.current {
                self.active_color
            } else {
                self.inactive_color
            };
            let circle = Path::circle(Point::new(x, y), radius);
            frame.fill(&circle, color);
        }

        vec![frame.into_geometry()]
    }
}

fn navigation_buttons<'a, Message: Clone + 'a>(
    state: &'a TourState,
    theme: &'a TourTheme,
    on_message: &(impl Fn(TourMessage) -> Message + 'a),
) -> Element<'a, Message> {
    let mut buttons = row![].spacing(8).align_y(iced::Alignment::Center);

    // Back button (hidden on first step)
    if !state.is_first_step() {
        let back_text = text("Back").size(14.0).color(theme.description_color);

        let back_btn = button(back_text)
            .on_press(on_message(TourMessage::Back))
            .style(move |_, _| button::Style {
                background: None,
                text_color: theme.description_color,
                border: Border::default(),
                shadow: iced::Shadow::default(),
                snap: false,
            })
            .padding([6, 12]);

        buttons = buttons.push(back_btn);
    }

    // Next/Finish button
    let next_label = if state.is_last_step() {
        "Finish"
    } else {
        "Next"
    };
    let next_text = text(next_label).size(14.0).color(Color::WHITE);

    let button_color = theme.button_color;
    let next_msg = if state.is_last_step() {
        TourMessage::Finish
    } else {
        TourMessage::Next
    };

    let next_btn = button(next_text)
        .on_press(on_message(next_msg))
        .style(move |_, _| button::Style {
            background: Some(Background::Color(button_color)),
            text_color: Color::WHITE,
            border: Border {
                radius: 6.0.into(),
                ..Default::default()
            },
            shadow: iced::Shadow::default(),
            snap: false,
        })
        .padding([6, 16]);

    buttons = buttons.push(next_btn);

    buttons.into()
}
