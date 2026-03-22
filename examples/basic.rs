use iced::widget::{button, column, container, row, stack, text, Space};
use iced::{
    Alignment, Background, Border, Color, Element, Length, Point, Rectangle, Size, Task, Theme,
};
use iced_tour::{tour_overlay, CardPosition, TourMessage, TourState, TourStep, TourTheme};

// Panel layout constants
const SIDEBAR_WIDTH: f32 = 200.0;
const BOTTOM_HEIGHT: f32 = 150.0;

fn main() -> iced::Result {
    iced::application(boot, update, view)
        .title("iced_tour — Basic Example")
        .window_size(Size::new(900.0, 600.0))
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
}

fn boot() -> (App, Task<Message>) {
    let steps = vec![
        TourStep::new(
            "Navigation",
            "Browse your projects and files in the sidebar.",
        )
        .target(Rectangle::new(
            Point::new(0.0, 0.0),
            Size::new(SIDEBAR_WIDTH, 600.0 - BOTTOM_HEIGHT),
        ))
        .card_position(CardPosition::Right),
        TourStep::new(
            "Canvas",
            "Your main workspace for editing and previewing content.",
        )
        .target(Rectangle::new(
            Point::new(SIDEBAR_WIDTH, 0.0),
            Size::new(900.0 - SIDEBAR_WIDTH, 600.0 - BOTTOM_HEIGHT),
        ))
        .card_position(CardPosition::Bottom),
        TourStep::new(
            "Timeline",
            "Arrange your clips and tracks in the timeline below.",
        )
        .target(Rectangle::new(
            Point::new(0.0, 600.0 - BOTTOM_HEIGHT),
            Size::new(900.0, BOTTOM_HEIGHT),
        ))
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
        }
        Message::Tour(msg) => {
            app.tour_state.update(msg);
        }
    }
    Task::none()
}

fn view(app: &App) -> Element<Message> {
    // Sidebar panel
    let sidebar = container(
        column![
            text("Sidebar").size(16).color(Color::WHITE),
            Space::new().height(16),
            button(text("Start Tour").size(14))
                .on_press(Message::StartTour)
                .padding([8, 16]),
        ]
        .padding(16),
    )
    .width(Length::Fixed(SIDEBAR_WIDTH))
    .height(Length::Fill)
    .style(|_| container::Style {
        background: Some(Background::Color(Color::from_rgb(0.15, 0.16, 0.18))),
        border: Border {
            color: Color::from_rgb(0.25, 0.26, 0.28),
            width: 1.0,
            ..Default::default()
        },
        snap: false,
        ..Default::default()
    });

    // Main area panel
    let main_area = container(
        column![
            text("Main Canvas").size(20).color(Color::WHITE),
            Space::new().height(8),
            text("This is your workspace.")
                .size(14)
                .color(Color::from_rgb(0.6, 0.6, 0.65)),
        ]
        .padding(24)
        .align_x(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .align_x(Alignment::Center)
    .align_y(Alignment::Center)
    .style(|_| container::Style {
        background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.12))),
        snap: false,
        ..Default::default()
    });

    // Bottom panel
    let bottom_panel = container(
        text("Timeline")
            .size(16)
            .color(Color::from_rgb(0.7, 0.7, 0.75)),
    )
    .width(Length::Fill)
    .height(Length::Fixed(BOTTOM_HEIGHT))
    .padding(16)
    .align_x(Alignment::Center)
    .align_y(Alignment::Center)
    .style(|_| container::Style {
        background: Some(Background::Color(Color::from_rgb(0.12, 0.12, 0.14))),
        border: Border {
            color: Color::from_rgb(0.25, 0.26, 0.28),
            width: 1.0,
            ..Default::default()
        },
        snap: false,
        ..Default::default()
    });

    // Layout: sidebar + main on top, timeline on bottom
    let top_row = row![sidebar, main_area];
    let layout = column![top_row, bottom_panel];

    // Stack the tour overlay on top
    stack![
        layout,
        tour_overlay(&app.tour_state, &app.tour_theme, Message::Tour),
    ]
    .into()
}
