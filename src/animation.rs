//! Animation configuration for tour step transitions.
//!
//! Controls how the spotlight cutout and tooltip card move between steps.
//! Works out of the box with sensible defaults — no configuration required.
//!
//! # Examples
//!
//! ```
//! use iced_tour::TourAnimation;
//!
//! // Default: 300ms, EaseOutCubic, enabled
//! let default = TourAnimation::default();
//!
//! // Disabled (instant jumps)
//! let none = TourAnimation::none();
//! ```

use std::time::Duration;

/// Animation configuration for step transitions.
///
/// The spotlight cutout and tooltip card will smoothly slide between
/// positions using the configured duration and easing function.
///
/// # Examples
///
/// ```
/// use std::time::Duration;
/// use iced_tour::TourAnimation;
/// use iced::animation::Easing;
///
/// // Default: 300ms with EaseOutCubic
/// let anim = TourAnimation::default();
///
/// // Custom: 400ms with EaseInOut
/// let anim = TourAnimation::new(Duration::from_millis(400), Easing::EaseInOut);
///
/// // Disabled: instant position changes
/// let anim = TourAnimation::none();
/// ```
#[derive(Debug, Clone)]
pub struct TourAnimation {
    /// How long the spotlight transition takes. Default: 300ms.
    pub duration: Duration,
    /// The easing curve for the transition. Default: `EaseOutCubic`.
    pub easing: iced::animation::Easing,
    /// Whether animation is enabled. When `false`, positions change instantly. Default: `true`.
    pub enabled: bool,
}

impl TourAnimation {
    /// Create a custom animation with the given duration and easing.
    pub fn new(duration: Duration, easing: iced::animation::Easing) -> Self {
        Self {
            duration,
            easing,
            enabled: true,
        }
    }

    /// Disable animation — positions change instantly.
    pub fn none() -> Self {
        Self {
            duration: Duration::ZERO,
            easing: iced::animation::Easing::Linear,
            enabled: false,
        }
    }
}

impl Default for TourAnimation {
    /// 300ms with EaseOutCubic — a natural deceleration that feels polished.
    fn default() -> Self {
        Self {
            duration: Duration::from_millis(300),
            easing: iced::animation::Easing::EaseOutCubic,
            enabled: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_animation() {
        let anim = TourAnimation::default();
        assert!(anim.enabled);
        assert_eq!(anim.duration, Duration::from_millis(300));
    }

    #[test]
    fn none_animation() {
        let anim = TourAnimation::none();
        assert!(!anim.enabled);
        assert_eq!(anim.duration, Duration::ZERO);
    }

    #[test]
    fn custom_animation() {
        let anim = TourAnimation::new(
            Duration::from_millis(500),
            iced::animation::Easing::EaseInOut,
        );
        assert!(anim.enabled);
        assert_eq!(anim.duration, Duration::from_millis(500));
    }
}
