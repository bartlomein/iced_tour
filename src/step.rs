use iced::Rectangle;

/// How a tour step identifies its spotlight target.
///
/// - `None` — No target. Shows a centered card with full dark backdrop.
/// - `Manual(Rectangle)` — Fixed rectangle. Use when you know exact pixel coordinates
///   (e.g., panels with dimensions stored in app state).
/// - `WidgetId(String)` — Reference a `container().id()` by name. Bounds are resolved
///   at runtime via `container::visible_bounds()` equivalent. Use for dynamically
///   positioned widgets (buttons, cards, responsive layouts).
///
/// # Examples
///
/// ```
/// use iced_tour::{TourStep, TourTarget, CardPosition};
/// use iced::{Rectangle, Point, Size};
///
/// // No target (centered card)
/// let intro = TourStep::new("Welcome", "Let's get started");
///
/// // Manual rectangle (known panel dimensions)
/// let panel = TourStep::new("Sidebar", "Your files are here")
///     .target(Rectangle::new(Point::new(0.0, 48.0), Size::new(300.0, 600.0)));
///
/// // Widget ID (resolved at runtime)
/// let button = TourStep::new("Open Video", "Click here to import")
///     .target_id("open_video");
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Default)]
pub enum TourTarget {
    /// No target — centered card with full dark backdrop.
    #[default]
    None,
    /// Fixed rectangle target (spotlight cutout at exact coordinates).
    Manual(Rectangle),
    /// Widget ID target — bounds resolved at runtime via iced container ID.
    WidgetId(String),
}

/// A single step in the guided tour.
///
/// Create steps with `TourStep::new(title, description)` and optionally chain
/// `.target(rect)` or `.target_id(id)` to highlight a specific UI area, or
/// `.card_position(pos)` to control where the tooltip card appears.
///
/// Steps without a target show a centered card with a full dark backdrop
/// (no cutout). This is the LLM-friendly default — works without knowing
/// pixel coordinates.
///
/// # Examples
///
/// ```
/// use iced_tour::{TourStep, CardPosition};
/// use iced::{Rectangle, Point, Size};
///
/// // Centered card (no target — works without knowing layout)
/// let welcome = TourStep::new("Welcome", "Let's take a quick tour of the app");
///
/// // Spotlight on a specific area (manual coordinates)
/// let preview = TourStep::new("Video Preview", "Drop a cycling video here")
///     .target(Rectangle::new(Point::new(200.0, 0.0), Size::new(800.0, 500.0)))
///     .card_position(CardPosition::Bottom);
///
/// // Spotlight by widget ID (resolved at runtime)
/// let button = TourStep::new("Open Video", "Click to import footage")
///     .target_id("open_video")
///     .card_position(CardPosition::Right);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct TourStep {
    title: String,
    description: String,
    target: TourTarget,
    card_position: CardPosition,
}

impl TourStep {
    /// Create a new tour step. The card will be centered on screen with a full
    /// dark backdrop (no cutout). Call `.target(rect)` or `.target_id(id)` to
    /// highlight a specific area.
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            target: TourTarget::None,
            card_position: CardPosition::Auto,
        }
    }

    /// Set the target rectangle to highlight with a spotlight cutout.
    /// When set, the backdrop will have a transparent hole around this area.
    pub fn target(mut self, rect: Rectangle) -> Self {
        self.target = TourTarget::Manual(rect);
        self
    }

    /// Set the target by widget ID. The widget must be wrapped in
    /// `container(widget).id(container::Id::new("my_id"))` in your view.
    /// Bounds are resolved at runtime after layout.
    pub fn target_id(mut self, id: impl Into<String>) -> Self {
        self.target = TourTarget::WidgetId(id.into());
        self
    }

    /// Control where the tooltip card appears relative to the target.
    /// Defaults to `CardPosition::Auto` which picks the best position
    /// based on available space.
    pub fn card_position(mut self, position: CardPosition) -> Self {
        self.card_position = position;
        self
    }

    /// Returns the step title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the step description.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the target type for this step.
    pub fn target_type(&self) -> &TourTarget {
        &self.target
    }

    /// Returns the target rectangle for manual targets.
    /// For widget ID targets, returns `None` — use `effective_rect()` with resolved bounds.
    pub fn target_rect(&self) -> Option<Rectangle> {
        match &self.target {
            TourTarget::Manual(rect) => Some(*rect),
            _ => None,
        }
    }

    /// Returns the card position setting.
    pub fn position(&self) -> CardPosition {
        self.card_position
    }

    /// Returns true if this step has no target (centered card mode).
    pub fn is_centered(&self) -> bool {
        matches!(self.target, TourTarget::None)
    }

    /// Returns true if this step uses a widget ID target that needs runtime resolution.
    pub fn needs_bounds_resolution(&self) -> bool {
        matches!(self.target, TourTarget::WidgetId(_))
    }

    /// Returns the widget ID for this step, if it uses widget ID targeting.
    pub fn widget_id(&self) -> Option<&str> {
        match &self.target {
            TourTarget::WidgetId(id) => Some(id.as_str()),
            _ => None,
        }
    }
}

/// Controls where the tooltip card appears relative to the spotlight target.
///
/// Use `Auto` (the default) to let the crate pick the best position based on
/// available viewport space. Use a specific direction to override.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CardPosition {
    /// Position the card above the target.
    Top,
    /// Position the card below the target.
    Bottom,
    /// Position the card to the left of the target.
    Left,
    /// Position the card to the right of the target.
    Right,
    /// The crate picks the best position based on available space.
    #[default]
    Auto,
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::{Point, Size};

    #[test]
    fn new_step_has_no_target() {
        let step = TourStep::new("Title", "Description");
        assert!(step.is_centered());
        assert!(!step.needs_bounds_resolution());
        assert_eq!(step.position(), CardPosition::Auto);
    }

    #[test]
    fn builder_sets_manual_target() {
        let rect = Rectangle::new(Point::new(10.0, 20.0), Size::new(100.0, 50.0));
        let step = TourStep::new("Title", "Desc").target(rect);
        assert_eq!(step.target_rect(), Some(rect));
        assert!(!step.is_centered());
        assert!(!step.needs_bounds_resolution());
    }

    #[test]
    fn builder_sets_widget_id_target() {
        let step = TourStep::new("Title", "Desc").target_id("my_button");
        assert!(step.needs_bounds_resolution());
        assert!(!step.is_centered());
        assert_eq!(step.widget_id(), Some("my_button"));
        assert_eq!(step.target_rect(), None);
    }

    #[test]
    fn builder_sets_card_position() {
        let step = TourStep::new("Title", "Desc").card_position(CardPosition::Bottom);
        assert_eq!(step.position(), CardPosition::Bottom);
    }

    #[test]
    fn builder_chains_manual() {
        let rect = Rectangle::new(Point::new(0.0, 0.0), Size::new(200.0, 100.0));
        let step = TourStep::new("Panel", "Info")
            .target(rect)
            .card_position(CardPosition::Left);
        assert_eq!(step.target_rect(), Some(rect));
        assert_eq!(step.position(), CardPosition::Left);
        assert_eq!(step.title(), "Panel");
        assert_eq!(step.description(), "Info");
    }

    #[test]
    fn builder_chains_widget_id() {
        let step = TourStep::new("Button", "Click here")
            .target_id("open_video")
            .card_position(CardPosition::Right);
        assert_eq!(step.widget_id(), Some("open_video"));
        assert_eq!(step.position(), CardPosition::Right);
    }

    #[test]
    fn target_overrides_target_id() {
        let rect = Rectangle::new(Point::new(0.0, 0.0), Size::new(100.0, 50.0));
        let step = TourStep::new("A", "B").target_id("foo").target(rect);
        assert!(!step.needs_bounds_resolution());
        assert_eq!(step.target_rect(), Some(rect));
    }

    #[test]
    fn target_id_overrides_target() {
        let rect = Rectangle::new(Point::new(0.0, 0.0), Size::new(100.0, 50.0));
        let step = TourStep::new("A", "B").target(rect).target_id("foo");
        assert!(step.needs_bounds_resolution());
        assert_eq!(step.widget_id(), Some("foo"));
    }
}
