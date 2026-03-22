#![warn(missing_docs)]
#![deny(unsafe_code)]

//! # iced_tour
//!
//! Guided tour / onboarding overlay for iced 0.14 apps.
//!
//! There are **zero** existing Rust crates for in-app onboarding/guided tours.
//! This is the first.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use iced_tour::{TourState, TourStep, TourMessage, TourTheme, CardPosition, tour_steps};
//!
//! // Define steps (no target = centered card, works without knowing layout)
//! let steps = tour_steps![
//!     "Welcome" => "Let's take a quick tour of the app",
//!     "Editor" => "This is where you edit your content",
//!     "Timeline" => "Arrange your clips here",
//! ];
//!
//! let state = TourState::new(steps);
//! ```
//!
//! See the [README](https://github.com/bartlomein/iced_tour) for full integration instructions.

pub mod animation;
pub mod bounds;
mod card;
mod manager;
/// Tour overlay rendering (backdrop, spotlight cutout, tooltip card).
pub mod overlay;
mod state;
mod step;
mod theme;

pub use animation::TourAnimation;
pub use bounds::visible_bounds;
pub use manager::{TourManager, TourManagerEvent, TourManagerMessage};
pub use overlay::{tour_manager_overlay, tour_overlay};
pub use state::{TourEvent, TourMessage, TourState};
pub use step::{CardPosition, TourStep, TourTarget};
pub use theme::{ThemeMode, TourTheme};

/// Convenience macro for quick step definitions.
///
/// Creates a `Vec<TourStep>` with no targets (all centered cards).
/// Targets and positions can be set afterwards with `.target()` and `.card_position()`.
///
/// # Examples
///
/// ```
/// use iced_tour::tour_steps;
///
/// let steps = tour_steps![
///     "Video Preview" => "Drop a cycling video here to get started",
///     "Timeline" => "Arrange your clips and telemetry tracks here",
///     "Inspector" => "Customize overlay styles, fonts, and colors",
/// ];
/// assert_eq!(steps.len(), 3);
/// assert_eq!(steps[0].title(), "Video Preview");
/// ```
#[macro_export]
macro_rules! tour_steps {
    ($($title:expr => $desc:expr),* $(,)?) => {
        vec![
            $(
                $crate::TourStep::new($title, $desc),
            )*
        ]
    };
}

/// Verify your tour integration. Only available in debug builds.
///
/// Prints a checklist of what's configured and what's missing to stdout.
///
/// # Example output
///
/// ```text
/// [iced_tour] Integration checklist:
///   + TourState created with 5 steps
///   - 3 steps have no target rectangle (will show centered)
///   + Theme: Dark mode
///   + Custom title font: Inter
///   Reminder: Make sure tour_overlay() is in your view's stack![]
/// ```
#[cfg(debug_assertions)]
pub fn integration_checklist(state: &TourState, theme: &TourTheme) {
    let total = state.steps().len();
    let no_target = state.steps().iter().filter(|s| s.is_centered()).count();
    let has_target = total - no_target;

    println!("[iced_tour] Integration checklist:");

    if total > 0 {
        println!("  + TourState created with {total} steps");
    } else {
        println!("  - TourState has no steps (add steps with TourState::new(vec![...]))");
    }

    if has_target > 0 {
        println!("  + {has_target} steps have target rectangles (spotlight cutout)");
    }
    if no_target > 0 {
        println!(
            "  {} {} steps have no target rectangle (will show centered)",
            if no_target == total { "-" } else { "~" },
            no_target
        );
    }

    println!("  + Theme: {:?} mode", theme.mode);

    if theme.title_font != iced::Font::DEFAULT {
        println!("  + Custom title font set");
    } else {
        println!("  ~ Using default title font (set with .with_title_font())");
    }

    if theme.description_font != iced::Font::DEFAULT {
        println!("  + Custom description font set");
    }

    if state.is_active() {
        println!(
            "  + Tour is currently active (step {})",
            state.step_index().0 + 1
        );
    } else if state.is_finished() {
        println!("  + Tour has been completed/skipped");
    } else {
        println!("  ~ Tour is inactive (call .start() to begin)");
    }

    println!("  Reminder: Make sure tour_overlay() is in your view's stack![]");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tour_steps_macro_creates_steps() {
        let steps = tour_steps![
            "Step 1" => "First",
            "Step 2" => "Second",
            "Step 3" => "Third",
        ];
        assert_eq!(steps.len(), 3);
        assert_eq!(steps[0].title(), "Step 1");
        assert_eq!(steps[0].description(), "First");
        assert_eq!(steps[2].title(), "Step 3");
        assert!(steps.iter().all(|s| s.is_centered()));
    }

    #[test]
    fn tour_steps_macro_empty() {
        let steps: Vec<TourStep> = tour_steps![];
        assert!(steps.is_empty());
    }

    #[test]
    fn tour_steps_macro_single() {
        let steps = tour_steps![
            "Only" => "One step",
        ];
        assert_eq!(steps.len(), 1);
    }

    #[test]
    fn tour_steps_macro_no_trailing_comma() {
        let steps = tour_steps![
            "A" => "First",
            "B" => "Second"
        ];
        assert_eq!(steps.len(), 2);
    }

    #[test]
    fn integration_checklist_runs_without_panic() {
        let state = TourState::new(tour_steps![
            "A" => "First",
            "B" => "Second",
        ]);
        let theme = TourTheme::dark();
        // Just verify it doesn't panic
        integration_checklist(&state, &theme);
    }
}
