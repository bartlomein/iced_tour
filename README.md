# iced_tour

Guided tour / onboarding overlay for [iced](https://iced.rs) 0.14 apps.

There are **zero** existing Rust crates for in-app guided tours. This is the first.

[![Crates.io](https://img.shields.io/crates/v/iced_tour.svg)](https://crates.io/crates/iced_tour)
[![Documentation](https://docs.rs/iced_tour/badge.svg)](https://docs.rs/iced_tour)
[![License](https://img.shields.io/crates/l/iced_tour.svg)](LICENSE-MIT)
[![CI](https://github.com/bartlomein/iced_tour/actions/workflows/ci.yml/badge.svg)](https://github.com/bartlomein/iced_tour/actions/workflows/ci.yml)

[![MSRV](https://img.shields.io/badge/MSRV-1.88-blue.svg)]()

![iced_tour demo](tour.gif)

## Features

- Full-screen backdrop with spotlight cutout around target UI areas
- Tooltip card with title, description, dot indicators, and navigation buttons
- Builder pattern API for defining tour steps
- Dark and light theme presets with full customization
- `tour_steps![]` convenience macro for quick definitions
- Zero-cost when inactive (returns invisible `Space`)
- Works with iced's `stack![]` composition — no custom overlay system needed
- Designed to be LLM-friendly: self-documenting types and actionable doc comments

## Quick Start

```rust
use iced_tour::{TourState, TourStep, TourTheme, TourMessage, tour_overlay, tour_steps};

// 1. Define your steps
let steps = tour_steps![
    "Welcome" => "Let's take a quick tour of the app",
    "Editor" => "This is where you edit your content",
    "Timeline" => "Arrange your clips here",
];

// 2. Create state (inactive by default)
let mut tour_state = TourState::new(steps);

// 3. Start the tour when ready
tour_state.start();
```

## Integration

### Add to your App struct

```rust
use iced_tour::{TourState, TourTheme};

struct App {
    tour_state: TourState,
    tour_theme: TourTheme,
    // ... your other fields
}
```

### Add to your Message enum

```rust
use iced_tour::TourMessage;

enum Message {
    Tour(TourMessage),
    // ... your other messages
}
```

### Add to your view (stack composition)

```rust
use iced_tour::tour_overlay;

fn view(&self) -> Element<Message> {
    stack![
        self.view_main_content(),
        tour_overlay(&self.tour_state, &self.tour_theme, Message::Tour),
    ]
    .into()
}
```

### Handle messages in update

```rust
fn update(&mut self, message: Message) {
    match message {
        Message::Tour(msg) => {
            self.tour_state.update(msg);
            if self.tour_state.is_finished() {
                // Persist completion so it doesn't repeat
            }
        }
        // ...
    }
}
```

## Tour Steps

Steps can optionally highlight a specific UI area with a spotlight cutout:

```rust
use iced_tour::{TourStep, CardPosition};
use iced::{Rectangle, Point, Size};

// Centered card (no target — works without knowing layout positions)
TourStep::new("Welcome", "Let's explore the app");

// Spotlight on a specific area
TourStep::new("Sidebar", "Your files are here")
    .target(Rectangle::new(Point::new(0.0, 48.0), Size::new(280.0, 500.0)))
    .card_position(CardPosition::Right);
```

## Themes

```rust
use iced_tour::TourTheme;

// Presets
let dark = TourTheme::dark();
let light = TourTheme::light();

// Customize
let custom = TourTheme::dark()
    .with_fonts(iced::Font::with_name("Inter"))
    .with_backdrop_opacity(0.8);

// Or set individual fields
let mut theme = TourTheme::dark();
theme.button_color = iced::Color::from_rgb(1.0, 0.42, 0.21);
```

## API Reference

| Function / Type | Description |
|---|---|
| `TourState::new(steps)` | Create with steps, inactive by default |
| `TourState::start()` | Activate the tour |
| `TourState::update(msg)` | Handle Next/Back/Skip/Finish |
| `TourState::is_active()` | Check if tour is showing |
| `TourState::is_finished()` | Check if user completed/skipped |
| `TourStep::new(title, desc)` | Step with centered card (no cutout) |
| `.target(rect)` | Highlight a specific area |
| `.card_position(pos)` | Control card placement |
| `tour_overlay(state, theme, mapper)` | The overlay Element for your stack |
| `tour_steps![...]` | Convenience macro |
| `TourTheme::dark()` / `light()` | Theme presets |
| `integration_checklist(state, theme)` | Debug helper |

## Example

```bash
cargo run -p iced_tour --example basic
```

## Minimum Supported Rust Version

Rust 1.75 or later.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
