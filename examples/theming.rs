use iced::widget::{button, column, container, row, stack, text, Space};
use iced::{Alignment, Color, Element, Length, Size, Task, Theme};
use iced_tour::{tour_overlay, tour_steps, TourMessage, TourState, TourTheme};

fn main() -> iced::Result {
    iced::application(boot, update, view)
        .title("iced_tour — Theming Example")
        .window_size(Size::new(600.0, 400.0))
        .theme(theme)
        .run()
}

fn theme(app: &App) -> Theme {
    if app.light_mode {
        Theme::Light
    } else {
        Theme::Dark
    }
}

struct App {
    tour_state: TourState,
    tour_theme: TourTheme,
    light_mode: bool,
}

#[derive(Debug, Clone)]
enum Message {
    StartDark,
    StartLight,
    StartCustom,
    Tour(TourMessage),
}

fn steps() -> Vec<iced_tour::TourStep> {
    tour_steps![
        "Theme Demo" => "This tour uses a custom theme. Check out the colors!",
        "Customizable" => "Use TourTheme::dark(), TourTheme::light(), or build your own.",
        "Builder API" => "Chain .with_backdrop_opacity(), .with_fonts(), .button_color, etc.",
    ]
}

fn boot() -> (App, Task<Message>) {
    let app = App {
        tour_state: TourState::new(steps()),
        tour_theme: TourTheme::dark(),
        light_mode: false,
    };
    (app, Task::none())
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::StartDark => {
            app.light_mode = false;
            app.tour_theme = TourTheme::dark();
            app.tour_state = TourState::new(steps());
            app.tour_state.start();
        }
        Message::StartLight => {
            app.light_mode = true;
            app.tour_theme = TourTheme::light();
            app.tour_state = TourState::new(steps());
            app.tour_state.start();
        }
        Message::StartCustom => {
            app.light_mode = false;
            let mut custom = TourTheme::dark().with_backdrop_opacity(0.85);
            custom.card_background = Color::from_rgb(0.1, 0.05, 0.2);
            custom.title_color = Color::from_rgb(0.7, 0.5, 1.0);
            custom.button_color = Color::from_rgb(0.6, 0.3, 0.9);
            app.tour_theme = custom;
            app.tour_state = TourState::new(steps());
            app.tour_state.start();
        }
        Message::Tour(msg) => {
            app.tour_state.update(msg);
        }
    }
    Task::none()
}

fn view(app: &App) -> Element<Message> {
    let content = container(
        column![
            text("Theme Showcase").size(24),
            Space::new().height(16),
            row![
                button(text("Dark Theme")).on_press(Message::StartDark),
                button(text("Light Theme")).on_press(Message::StartLight),
                button(text("Custom Theme")).on_press(Message::StartCustom),
            ]
            .spacing(12),
        ]
        .spacing(8)
        .align_x(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .align_x(Alignment::Center)
    .align_y(Alignment::Center);

    stack![
        content,
        tour_overlay(&app.tour_state, &app.tour_theme, Message::Tour),
    ]
    .into()
}
