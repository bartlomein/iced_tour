use iced::widget::{button, column, container, stack, text};
use iced::{Element, Length, Size, Task, Theme};
use iced_tour::{tour_overlay, tour_steps, TourMessage, TourState, TourTheme};

fn main() -> iced::Result {
    iced::application(boot, update, view)
        .title("iced_tour — Minimal Example")
        .window_size(Size::new(600.0, 400.0))
        .theme(theme)
        .run()
}

struct App {
    tour_state: TourState,
    tour_theme: TourTheme,
}

fn theme(_app: &App) -> Theme {
    Theme::Dark
}

#[derive(Debug, Clone)]
enum Message {
    StartTour,
    Tour(TourMessage),
}

fn boot() -> (App, Task<Message>) {
    let app = App {
        tour_state: TourState::new(tour_steps![
            "Welcome" => "This is a guided tour overlay built with iced_tour.",
            "Navigation" => "Click Next/Back to move between steps, or Skip to dismiss.",
            "That's it!" => "You've seen the basics. Add .target() or .target_id() to highlight specific widgets.",
        ]),
        tour_theme: TourTheme::dark(),
    };
    (app, Task::none())
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::StartTour => app.tour_state.start(),
        Message::Tour(msg) => {
            app.tour_state.update(msg);
        }
    }
    Task::none()
}

fn view(app: &App) -> Element<Message> {
    let content = container(
        column![
            text("My App").size(24),
            button(text("Start Tour")).on_press(Message::StartTour),
        ]
        .spacing(16)
        .align_x(iced::Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .align_x(iced::Alignment::Center)
    .align_y(iced::Alignment::Center);

    stack![
        content,
        tour_overlay(&app.tour_state, &app.tour_theme, Message::Tour),
    ]
    .into()
}
