use iced::widget::{button, column, container, row, stack, text, Space};
use iced::{
    Alignment, Background, Border, Color, Element, Length, Rectangle, Size, Task, Theme,
};
use iced_tour::{tour_overlay, CardPosition, TourMessage, TourState, TourStep, TourTheme};

fn main() -> iced::Result {
    iced::application(boot, update, view)
        .title("iced_tour — Widget ID Targeting")
        .window_size(Size::new(800.0, 500.0))
        .theme(theme)
        .run()
}

fn theme(_app: &App) -> Theme {
    Theme::Dark
}

struct App {
    tour_state: TourState,
    tour_theme: TourTheme,
}

#[derive(Debug, Clone)]
enum Message {
    StartTour,
    Tour(TourMessage),
    BoundsResolved(Rectangle),
}

fn boot() -> (App, Task<Message>) {
    let steps = vec![
        TourStep::new("Welcome", "Let's explore the app. No hardcoded coordinates needed!"),
        TourStep::new("Toolbar", "Actions live up here. Resize the window — the spotlight follows.")
            .target_id("toolbar")
            .card_position(CardPosition::Bottom),
        TourStep::new("Content", "Your main workspace adapts to any window size.")
            .target_id("content")
            .card_position(CardPosition::Top),
        TourStep::new("Status Bar", "Status info stays anchored at the bottom.")
            .target_id("status_bar")
            .card_position(CardPosition::Top),
    ];

    let app = App {
        tour_state: TourState::new(steps),
        tour_theme: TourTheme::dark(),
    };
    (app, Task::none())
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::StartTour => {
            app.tour_state.start();
            resolve_bounds(app)
        }
        Message::Tour(msg) => {
            app.tour_state.update(msg);
            resolve_bounds(app)
        }
        Message::BoundsResolved(bounds) => {
            app.tour_state.set_resolved_bounds(bounds);
            Task::none()
        }
    }
}

/// If the current step targets a widget ID, resolve its bounds.
fn resolve_bounds(app: &App) -> Task<Message> {
    let step = match app.tour_state.current_step() {
        Some(s) => s,
        None => return Task::none(),
    };

    match step.widget_id() {
        Some(id) => {
            let widget_id = iced::widget::Id::from(id.to_string());
            iced_tour::visible_bounds(widget_id).map(|bounds| {
                Message::BoundsResolved(bounds.unwrap_or_default())
            })
        }
        None => Task::none(),
    }
}

fn view(app: &App) -> Element<Message> {
    // Toolbar
    let toolbar = container(
        row![
            text("File").size(14),
            text("Edit").size(14),
            text("View").size(14),
            Space::new().width(Length::Fill),
            button(text("Start Tour").size(13))
                .on_press(Message::StartTour)
                .padding([6, 12]),
        ]
        .spacing(16)
        .align_y(Alignment::Center)
        .padding([0, 12]),
    )
    .id(iced::widget::Id::new("toolbar"))
    .width(Length::Fill)
    .height(Length::Fixed(44.0))
    .align_y(Alignment::Center)
    .style(|_| container::Style {
        background: Some(Background::Color(Color::from_rgb(0.15, 0.15, 0.18))),
        border: Border {
            color: Color::from_rgb(0.25, 0.26, 0.28),
            width: 1.0,
            ..Default::default()
        },
        ..Default::default()
    });

    // Main content
    let content = container(
        column![
            text("Main Content Area").size(20).color(Color::WHITE),
            Space::new().height(8),
            text("Resize the window and start the tour again — the spotlight follows the widgets.")
                .size(14)
                .color(Color::from_rgb(0.6, 0.6, 0.65)),
        ]
        .align_x(Alignment::Center),
    )
    .id(iced::widget::Id::new("content"))
    .width(Length::Fill)
    .height(Length::Fill)
    .align_x(Alignment::Center)
    .align_y(Alignment::Center)
    .style(|_| container::Style {
        background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.12))),
        ..Default::default()
    });

    // Status bar
    let status_bar = container(
        row![
            text("Ready").size(12).color(Color::from_rgb(0.5, 0.8, 0.5)),
            Space::new().width(Length::Fill),
            text("v1.0.0").size(12).color(Color::from_rgb(0.5, 0.5, 0.55)),
        ]
        .padding([0, 12]),
    )
    .id(iced::widget::Id::new("status_bar"))
    .width(Length::Fill)
    .height(Length::Fixed(28.0))
    .align_y(Alignment::Center)
    .style(|_| container::Style {
        background: Some(Background::Color(Color::from_rgb(0.12, 0.12, 0.14))),
        border: Border {
            color: Color::from_rgb(0.25, 0.26, 0.28),
            width: 1.0,
            ..Default::default()
        },
        ..Default::default()
    });

    let layout = column![toolbar, content, status_bar];

    stack![
        layout,
        tour_overlay(&app.tour_state, &app.tour_theme, Message::Tour),
    ]
    .into()
}
