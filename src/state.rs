use std::time::Instant;

use iced::{Animation, Rectangle};

use crate::animation::TourAnimation;
use crate::step::{TourStep, TourTarget};

/// The core state machine for a guided tour.
///
/// Add this as a field on your App struct.
/// Initialize with `TourState::new(steps)` in your app's `new()` function.
/// Call `state.start()` to begin the tour (e.g., on first launch).
///
/// # Examples
///
/// ```
/// use iced_tour::{TourState, TourStep};
///
/// let state = TourState::new(vec![
///     TourStep::new("Welcome", "Let's explore the app"),
///     TourStep::new("Editor", "This is where you edit"),
/// ]);
/// assert!(!state.is_active());
/// ```
#[derive(Debug, Clone)]
pub struct TourState {
    steps: Vec<TourStep>,
    current_step: usize,
    is_active: bool,
    is_finished: bool,
    /// Cached bounds for the current step's widget ID target (resolved at runtime).
    resolved_bounds: Option<Rectangle>,
    /// Previous step's resolved bounds — used as fallback during the one-frame
    /// gap while new bounds are being resolved, preventing a flash of no-cutout.
    previous_bounds: Option<Rectangle>,
    /// Spotlight transition animations (x, y, width, height).
    spotlight_anim: Option<SpotlightAnim>,
}

/// Animates the spotlight cutout rectangle between positions.
#[derive(Debug, Clone)]
struct SpotlightAnim {
    x: Animation<f32>,
    y: Animation<f32>,
    w: Animation<f32>,
    h: Animation<f32>,
}

impl TourState {
    /// Create a new tour with the given steps. The tour starts inactive.
    /// Call `.start()` to begin showing it (e.g., on first launch).
    pub fn new(steps: Vec<TourStep>) -> Self {
        Self {
            steps,
            current_step: 0,
            is_active: false,
            is_finished: false,
            resolved_bounds: None,
            previous_bounds: None,
            spotlight_anim: None,
        }
    }

    /// Activate the tour, showing from the first step.
    /// Call this when you want the tour to appear (e.g., first launch).
    pub fn start(&mut self) {
        if !self.steps.is_empty() {
            self.current_step = 0;
            self.is_active = true;
            self.is_finished = false;
            self.resolved_bounds = None;
            self.previous_bounds = None;
        }
    }

    /// Handle a tour navigation message. Returns lifecycle events that the app
    /// can use to trigger side effects (e.g., switching tabs, saving preferences,
    /// sending analytics).
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // In your update() function:
    /// Message::Tour(msg) => {
    ///     for event in self.tour_state.update(msg) {
    ///         match event {
    ///             TourEvent::StepEntered { index, .. } => { /* switch tab */ }
    ///             TourEvent::Completed => { /* save preferences */ }
    ///             TourEvent::Skipped { .. } => { /* analytics */ }
    ///             _ => {}
    ///         }
    ///     }
    /// }
    /// ```
    pub fn update(&mut self, message: TourMessage) -> Vec<TourEvent> {
        if !self.is_active {
            return vec![];
        }

        let prev_step = self.current_step;
        let mut events = Vec::new();

        match message {
            TourMessage::Next | TourMessage::BackdropClicked => {
                if self.current_step + 1 < self.steps.len() {
                    events.push(TourEvent::StepExited { index: prev_step });
                    self.current_step += 1;
                    self.previous_bounds = self.resolved_bounds.take();
                    events.push(TourEvent::StepEntered {
                        index: self.current_step,
                    });
                } else {
                    events.push(TourEvent::StepExited { index: prev_step });
                    self.finish();
                    events.push(TourEvent::Completed);
                }
            }
            TourMessage::Back => {
                if self.current_step > 0 {
                    events.push(TourEvent::StepExited { index: prev_step });
                    self.current_step -= 1;
                    self.previous_bounds = self.resolved_bounds.take();
                    events.push(TourEvent::StepEntered {
                        index: self.current_step,
                    });
                }
            }
            TourMessage::Finish => {
                events.push(TourEvent::StepExited { index: prev_step });
                self.finish();
                events.push(TourEvent::Completed);
            }
            TourMessage::Skip => {
                events.push(TourEvent::StepExited { index: prev_step });
                self.finish();
                events.push(TourEvent::Skipped { at_step: prev_step });
            }
        }

        events
    }

    /// Returns the current step, or `None` if the tour is inactive or empty.
    pub fn current_step(&self) -> Option<&TourStep> {
        if self.is_active {
            self.steps.get(self.current_step)
        } else {
            None
        }
    }

    /// Returns `(current_index, total_steps)` for rendering dot indicators.
    /// Returns `(0, 0)` if the tour is inactive.
    pub fn step_index(&self) -> (usize, usize) {
        if self.is_active {
            (self.current_step, self.steps.len())
        } else {
            (0, 0)
        }
    }

    /// Returns true if the tour is currently showing.
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    /// Returns true if the user completed or skipped the tour.
    pub fn is_finished(&self) -> bool {
        self.is_finished
    }

    /// Returns a slice of all steps.
    pub fn steps(&self) -> &[TourStep] {
        &self.steps
    }

    /// Returns true if the current step is the first step.
    pub fn is_first_step(&self) -> bool {
        self.current_step == 0
    }

    /// Returns true if the current step is the last step.
    pub fn is_last_step(&self) -> bool {
        self.current_step + 1 >= self.steps.len()
    }

    /// Set the resolved bounds for the current step's widget ID target.
    /// Call this after `container::visible_bounds()` returns the rectangle.
    pub fn set_resolved_bounds(&mut self, bounds: Rectangle) {
        self.resolved_bounds = Some(bounds);
        self.previous_bounds = None;
    }

    /// Set resolved bounds and start an animation from the previous target.
    /// Call this instead of `set_resolved_bounds` when you want smooth transitions.
    pub fn set_resolved_bounds_animated(&mut self, bounds: Rectangle, config: &TourAnimation) {
        let old_target = self.effective_target();
        self.resolved_bounds = Some(bounds);
        self.previous_bounds = None;

        if config.enabled {
            // Determine the "from" rectangle for the animation.
            // If previous step had a target, animate from there.
            // If previous step was centered (no target), animate from the
            // center of the new target (expand-from-center effect).
            let from = old_target.unwrap_or(Rectangle {
                x: bounds.x + bounds.width / 2.0,
                y: bounds.y + bounds.height / 2.0,
                width: 0.0,
                height: 0.0,
            });

            if from != bounds {
                let now = Instant::now();
                let dur = config.duration;
                let ease = config.easing;

                let mut x = Animation::new(from.x).duration(dur).easing(ease);
                let mut y = Animation::new(from.y).duration(dur).easing(ease);
                let mut w = Animation::new(from.width).duration(dur).easing(ease);
                let mut h = Animation::new(from.height).duration(dur).easing(ease);

                x.go_mut(bounds.x, now);
                y.go_mut(bounds.y, now);
                w.go_mut(bounds.width, now);
                h.go_mut(bounds.height, now);

                self.spotlight_anim = Some(SpotlightAnim { x, y, w, h });
                return;
            }
        }
        self.spotlight_anim = None;
    }

    /// Clear the resolved bounds (e.g., on window resize to trigger re-query).
    pub fn clear_resolved_bounds(&mut self) {
        self.resolved_bounds = None;
    }

    /// Returns the effective target rectangle for the current step.
    ///
    /// - `TourTarget::Manual(rect)` → returns `Some(rect)`
    /// - `TourTarget::WidgetId(_)` → returns `resolved_bounds` (may be `None` if not yet resolved)
    /// - `TourTarget::None` → returns `None`
    pub fn effective_target(&self) -> Option<Rectangle> {
        let step = self.current_step()?;
        match step.target_type() {
            TourTarget::None => None,
            TourTarget::Manual(rect) => Some(*rect),
            TourTarget::WidgetId(_) => self.resolved_bounds.or(self.previous_bounds),
        }
    }

    /// Returns the animated target rectangle for the current step.
    ///
    /// If a spotlight animation is in progress, returns the interpolated position.
    /// Otherwise falls back to `effective_target()`.
    pub fn animated_target(&self, now: Instant) -> Option<Rectangle> {
        if let Some(ref anim) = self.spotlight_anim {
            if anim.x.is_animating(now)
                || anim.y.is_animating(now)
                || anim.w.is_animating(now)
                || anim.h.is_animating(now)
            {
                return Some(Rectangle {
                    x: anim.x.interpolate_with(|v| v, now),
                    y: anim.y.interpolate_with(|v| v, now),
                    width: anim.w.interpolate_with(|v| v, now),
                    height: anim.h.interpolate_with(|v| v, now),
                });
            }
        }
        self.effective_target()
    }

    /// Returns true if any spotlight animation is currently in progress.
    pub fn is_animating(&self, now: Instant) -> bool {
        self.spotlight_anim.as_ref().is_some_and(|anim| {
            anim.x.is_animating(now)
                || anim.y.is_animating(now)
                || anim.w.is_animating(now)
                || anim.h.is_animating(now)
        })
    }

    /// Returns true if the current step needs bounds resolution (widget ID target
    /// without resolved bounds).
    pub fn needs_bounds_resolution(&self) -> bool {
        self.current_step()
            .map(|s| s.needs_bounds_resolution() && self.resolved_bounds.is_none())
            .unwrap_or(false)
    }

    fn finish(&mut self) {
        self.is_active = false;
        self.is_finished = true;
    }
}

/// Add `Tour(TourMessage)` as a variant in your app's Message enum.
/// In your `update()` function, match on it and call `tour_state.update(msg)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TourMessage {
    /// Advance to the next step. If on the last step, finishes the tour.
    Next,
    /// Go back to the previous step. Does nothing on the first step.
    Back,
    /// Skip the entire tour immediately.
    Skip,
    /// Finish the tour (same as Skip but semantically different — user completed it).
    Finish,
    /// The dark backdrop was clicked. Advances to next step, finishes on last.
    BackdropClicked,
}

/// Lifecycle events emitted by `TourState::update()`.
///
/// Use these to trigger app-specific side effects like switching UI tabs,
/// persisting completion state, or sending analytics events.
///
/// # Examples
///
/// ```ignore
/// for event in self.tour_state.update(msg) {
///     match event {
///         TourEvent::StepEntered { index } if index == 2 => {
///             self.sidebar_tab = Tab::Overlays;
///         }
///         TourEvent::Completed | TourEvent::Skipped { .. } => {
///             self.preferences.tour_completed = true;
///             save_preferences(&self.preferences);
///         }
///         _ => {}
///     }
/// }
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TourEvent {
    /// A step was entered (navigated to). Fires after the step index changes.
    StepEntered {
        /// The index of the step that was entered.
        index: usize,
    },
    /// A step was exited (navigated away from). Fires before the step index changes.
    StepExited {
        /// The index of the step that was exited.
        index: usize,
    },
    /// The tour was completed (user reached the last step and clicked Next/Finish).
    Completed,
    /// The tour was skipped (user clicked Skip before reaching the end).
    Skipped {
        /// The step index where the user skipped.
        at_step: usize,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_steps() -> Vec<TourStep> {
        vec![
            TourStep::new("Step 1", "First step"),
            TourStep::new("Step 2", "Second step"),
            TourStep::new("Step 3", "Third step"),
        ]
    }

    #[test]
    fn new_state_is_inactive() {
        let state = TourState::new(sample_steps());
        assert!(!state.is_active());
        assert!(!state.is_finished());
        assert!(state.current_step().is_none());
        assert_eq!(state.step_index(), (0, 0));
    }

    #[test]
    fn start_activates_tour() {
        let mut state = TourState::new(sample_steps());
        state.start();
        assert!(state.is_active());
        assert!(!state.is_finished());
        assert_eq!(state.current_step().unwrap().title(), "Step 1");
        assert_eq!(state.step_index(), (0, 3));
    }

    #[test]
    fn start_with_empty_steps_does_nothing() {
        let mut state = TourState::new(vec![]);
        state.start();
        assert!(!state.is_active());
    }

    #[test]
    fn next_advances_step() {
        let mut state = TourState::new(sample_steps());
        state.start();
        let events = state.update(TourMessage::Next);
        assert_eq!(state.step_index(), (1, 3));
        assert_eq!(state.current_step().unwrap().title(), "Step 2");
        assert_eq!(
            events,
            vec![
                TourEvent::StepExited { index: 0 },
                TourEvent::StepEntered { index: 1 },
            ]
        );
    }

    #[test]
    fn next_on_last_step_finishes() {
        let mut state = TourState::new(sample_steps());
        state.start();
        state.update(TourMessage::Next);
        state.update(TourMessage::Next);
        let events = state.update(TourMessage::Next);
        assert!(!state.is_active());
        assert!(state.is_finished());
        assert_eq!(
            events,
            vec![TourEvent::StepExited { index: 2 }, TourEvent::Completed,]
        );
    }

    #[test]
    fn back_goes_to_previous() {
        let mut state = TourState::new(sample_steps());
        state.start();
        state.update(TourMessage::Next);
        let events = state.update(TourMessage::Back);
        assert_eq!(state.step_index(), (0, 3));
        assert_eq!(
            events,
            vec![
                TourEvent::StepExited { index: 1 },
                TourEvent::StepEntered { index: 0 },
            ]
        );
    }

    #[test]
    fn back_on_first_step_does_nothing() {
        let mut state = TourState::new(sample_steps());
        state.start();
        let events = state.update(TourMessage::Back);
        assert_eq!(state.step_index(), (0, 3));
        assert!(events.is_empty());
    }

    #[test]
    fn skip_emits_skipped_event() {
        let mut state = TourState::new(sample_steps());
        state.start();
        state.update(TourMessage::Next); // step 1
        let events = state.update(TourMessage::Skip);
        assert!(!state.is_active());
        assert!(state.is_finished());
        assert_eq!(
            events,
            vec![
                TourEvent::StepExited { index: 1 },
                TourEvent::Skipped { at_step: 1 },
            ]
        );
    }

    #[test]
    fn finish_message_emits_completed() {
        let mut state = TourState::new(sample_steps());
        state.start();
        let events = state.update(TourMessage::Finish);
        assert!(!state.is_active());
        assert!(state.is_finished());
        assert_eq!(
            events,
            vec![TourEvent::StepExited { index: 0 }, TourEvent::Completed,]
        );
    }

    #[test]
    fn backdrop_click_advances() {
        let mut state = TourState::new(sample_steps());
        state.start();
        let events = state.update(TourMessage::BackdropClicked);
        assert_eq!(state.step_index(), (1, 3));
        assert_eq!(
            events,
            vec![
                TourEvent::StepExited { index: 0 },
                TourEvent::StepEntered { index: 1 },
            ]
        );
    }

    #[test]
    fn backdrop_click_on_last_finishes() {
        let mut state = TourState::new(sample_steps());
        state.start();
        state.update(TourMessage::Next);
        state.update(TourMessage::Next);
        let events = state.update(TourMessage::BackdropClicked);
        assert!(!state.is_active());
        assert!(state.is_finished());
        assert_eq!(
            events,
            vec![TourEvent::StepExited { index: 2 }, TourEvent::Completed,]
        );
    }

    #[test]
    fn messages_ignored_when_inactive() {
        let mut state = TourState::new(sample_steps());
        let events = state.update(TourMessage::Next);
        assert!(!state.is_active());
        assert!(events.is_empty());
    }

    #[test]
    fn is_first_and_last_step() {
        let mut state = TourState::new(sample_steps());
        state.start();
        assert!(state.is_first_step());
        assert!(!state.is_last_step());

        state.update(TourMessage::Next);
        state.update(TourMessage::Next);
        assert!(!state.is_first_step());
        assert!(state.is_last_step());
    }

    #[test]
    fn restart_after_finish() {
        let mut state = TourState::new(sample_steps());
        state.start();
        state.update(TourMessage::Skip);
        assert!(state.is_finished());

        state.start();
        assert!(state.is_active());
        assert!(!state.is_finished());
        assert_eq!(state.step_index(), (0, 3));
    }
}
