use crate::animation::TourAnimation;
use iced::{Color, Font};

/// Visual theme for the tour overlay, tooltip card, and spotlight cutout.
///
/// Use `TourTheme::dark()` or `TourTheme::light()` for presets, or create
/// a custom theme by modifying individual fields.
///
/// The crate never loads fonts. Your app loads fonts via
/// `iced::application(...).font(...)` and you pass `Font::with_name("Inter")`
/// (or whatever) into the theme. The crate renders with whatever `Font` it receives.
///
/// # Examples
///
/// ```
/// use iced_tour::TourTheme;
/// use iced::Font;
///
/// // Use a preset
/// let theme = TourTheme::dark();
///
/// // Customize fonts
/// let theme = TourTheme::dark()
///     .with_title_font(Font::with_name("Inter"))
///     .with_description_font(Font::with_name("Inter"));
/// ```
#[derive(Debug, Clone)]
pub struct TourTheme {
    /// Whether this is a dark or light theme.
    pub mode: ThemeMode,
    /// Color of the semi-transparent backdrop overlay.
    pub backdrop_color: Color,
    /// Background color of the tooltip card.
    pub card_background: Color,
    /// Border radius of the tooltip card in pixels.
    pub card_border_radius: f32,
    /// Color of the title text in the tooltip card.
    pub title_color: Color,
    /// Color of the description text in the tooltip card.
    pub description_color: Color,
    /// Font for the title text.
    pub title_font: Font,
    /// Font for the description text.
    pub description_font: Font,
    /// Size of the title text in pixels.
    pub title_size: f32,
    /// Size of the description text in pixels.
    pub description_size: f32,
    /// Accent color for navigation buttons.
    pub button_color: Color,
    /// Color of the active dot indicator.
    pub dot_active_color: Color,
    /// Color of inactive dot indicators.
    pub dot_inactive_color: Color,
    /// Border radius of the spotlight cutout in pixels.
    pub cutout_border_radius: f32,
    /// Padding around the spotlight cutout in pixels.
    pub cutout_padding: f32,
    /// Whether Escape key dismisses the tour. Default: `true`.
    pub allow_escape: bool,
    /// Animation configuration for step transitions.
    pub animation: TourAnimation,
}

impl TourTheme {
    /// Dark theme preset — dark card with light text.
    /// Good for apps with dark backgrounds.
    pub fn dark() -> Self {
        Self {
            mode: ThemeMode::Dark,
            backdrop_color: Color::from_rgba(0.0, 0.0, 0.0, 0.7),
            card_background: Color::from_rgb(0.15, 0.15, 0.18),
            card_border_radius: 12.0,
            title_color: Color::WHITE,
            description_color: Color::from_rgb(0.78, 0.78, 0.82),
            title_font: Font::DEFAULT,
            description_font: Font::DEFAULT,
            title_size: 18.0,
            description_size: 14.0,
            button_color: Color::from_rgb(0.35, 0.55, 1.0),
            dot_active_color: Color::WHITE,
            dot_inactive_color: Color::from_rgba(1.0, 1.0, 1.0, 0.3),
            cutout_border_radius: 8.0,
            cutout_padding: 8.0,
            allow_escape: true,
            animation: TourAnimation::default(),
        }
    }

    /// Light theme preset — light card with dark text.
    /// Good for apps with light backgrounds.
    pub fn light() -> Self {
        Self {
            mode: ThemeMode::Light,
            backdrop_color: Color::from_rgba(0.0, 0.0, 0.0, 0.5),
            card_background: Color::WHITE,
            card_border_radius: 12.0,
            title_color: Color::from_rgb(0.1, 0.1, 0.12),
            description_color: Color::from_rgb(0.35, 0.35, 0.4),
            title_font: Font::DEFAULT,
            description_font: Font::DEFAULT,
            title_size: 18.0,
            description_size: 14.0,
            button_color: Color::from_rgb(0.2, 0.45, 0.95),
            dot_active_color: Color::from_rgb(0.2, 0.2, 0.25),
            dot_inactive_color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
            cutout_border_radius: 8.0,
            cutout_padding: 8.0,
            allow_escape: true,
            animation: TourAnimation::default(),
        }
    }

    /// Set the animation configuration for step transitions.
    pub fn with_animation(mut self, animation: TourAnimation) -> Self {
        self.animation = animation;
        self
    }

    /// Set a custom title font.
    pub fn with_title_font(mut self, font: Font) -> Self {
        self.title_font = font;
        self
    }

    /// Set a custom description font.
    pub fn with_description_font(mut self, font: Font) -> Self {
        self.description_font = font;
        self
    }

    /// Set both title and description fonts at once.
    pub fn with_fonts(mut self, font: Font) -> Self {
        self.title_font = font;
        self.description_font = font;
        self
    }

    /// Set the backdrop opacity (0.0 = transparent, 1.0 = fully opaque).
    pub fn with_backdrop_opacity(mut self, opacity: f32) -> Self {
        self.backdrop_color = Color::from_rgba(
            self.backdrop_color.r,
            self.backdrop_color.g,
            self.backdrop_color.b,
            opacity,
        );
        self
    }

    /// Set the title text size in pixels.
    pub fn with_title_size(mut self, size: f32) -> Self {
        self.title_size = size;
        self
    }

    /// Set the description text size in pixels.
    pub fn with_description_size(mut self, size: f32) -> Self {
        self.description_size = size;
        self
    }

    /// Set the card border radius in pixels.
    pub fn with_card_border_radius(mut self, radius: f32) -> Self {
        self.card_border_radius = radius;
        self
    }

    /// Set the spotlight cutout border radius in pixels.
    pub fn with_cutout_border_radius(mut self, radius: f32) -> Self {
        self.cutout_border_radius = radius;
        self
    }

    /// Set the padding around the spotlight cutout in pixels.
    pub fn with_cutout_padding(mut self, padding: f32) -> Self {
        self.cutout_padding = padding;
        self
    }

    /// Set the accent color for buttons.
    pub fn with_button_color(mut self, color: Color) -> Self {
        self.button_color = color;
        self
    }

    /// Set whether the Escape key dismisses the tour.
    pub fn with_allow_escape(mut self, allow: bool) -> Self {
        self.allow_escape = allow;
        self
    }
}

impl Default for TourTheme {
    fn default() -> Self {
        Self::dark()
    }
}

/// Whether the theme uses dark or light colors.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    /// Dark card with light text — for apps with dark backgrounds.
    Dark,
    /// Light card with dark text — for apps with light backgrounds.
    Light,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dark_theme_defaults() {
        let theme = TourTheme::dark();
        assert_eq!(theme.mode, ThemeMode::Dark);
        assert_eq!(theme.card_border_radius, 12.0);
        assert_eq!(theme.title_size, 18.0);
        assert_eq!(theme.description_size, 14.0);
        assert_eq!(theme.cutout_border_radius, 8.0);
        assert_eq!(theme.cutout_padding, 8.0);
    }

    #[test]
    fn light_theme_defaults() {
        let theme = TourTheme::light();
        assert_eq!(theme.mode, ThemeMode::Light);
        assert_eq!(theme.card_border_radius, 12.0);
    }

    #[test]
    fn with_fonts_sets_both() {
        let font = Font::with_name("Inter");
        let theme = TourTheme::dark().with_fonts(font);
        assert_eq!(theme.title_font, font);
        assert_eq!(theme.description_font, font);
    }

    #[test]
    fn with_backdrop_opacity() {
        let theme = TourTheme::dark().with_backdrop_opacity(0.5);
        assert!((theme.backdrop_color.a - 0.5).abs() < f32::EPSILON);
    }
}
