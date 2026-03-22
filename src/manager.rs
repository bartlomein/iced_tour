use std::collections::{HashMap, HashSet};

use crate::state::{TourEvent, TourMessage, TourState};
use crate::step::TourStep;

/// Manages multiple named tours with shared completion tracking.
///
/// Only one tour can be active at a time. Starting a new tour automatically
/// stops the current one (emitting `Skipped` events).
///
/// # Examples
///
/// ```
/// use iced_tour::{TourManager, TourStep, tour_steps};
///
/// let mut manager = TourManager::new()
///     .add_tour("welcome", tour_steps!["Hi" => "Welcome to the app"])
///     .add_tour("editor", tour_steps!["Editor" => "This is the editor"]);
///
/// manager.start("welcome");
/// assert!(manager.is_active());
/// assert_eq!(manager.active_tour(), Some("welcome"));
/// ```
#[derive(Debug, Clone)]
pub struct TourManager {
    tours: HashMap<String, TourState>,
    active_tour: Option<String>,
    completed: HashSet<String>,
}

/// A message routed to the active tour within a `TourManager`.
///
/// Use this in your app's Message enum:
/// ```ignore
/// enum Message {
///     Tour(TourManagerMessage),
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TourManagerMessage(pub TourMessage);

impl From<TourMessage> for TourManagerMessage {
    fn from(msg: TourMessage) -> Self {
        Self(msg)
    }
}

/// Events emitted by `TourManager::update()`, enriched with the tour name.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TourManagerEvent {
    /// A lifecycle event from a specific named tour.
    Tour {
        /// The name of the tour that emitted the event.
        name: String,
        /// The lifecycle event.
        event: TourEvent,
    },
    /// A tour was stopped because another tour was started.
    TourInterrupted {
        /// The name of the tour that was interrupted.
        name: String,
        /// The step index where the tour was interrupted.
        at_step: usize,
    },
}

impl TourManager {
    /// Create an empty tour manager.
    pub fn new() -> Self {
        Self {
            tours: HashMap::new(),
            active_tour: None,
            completed: HashSet::new(),
        }
    }

    /// Register a named tour. Builder-style, returns `self`.
    ///
    /// If a tour with the same name already exists, it is replaced.
    pub fn add_tour(mut self, name: impl Into<String>, steps: Vec<TourStep>) -> Self {
        self.tours.insert(name.into(), TourState::new(steps));
        self
    }

    /// Register a named tour on a mutable reference.
    pub fn insert_tour(&mut self, name: impl Into<String>, steps: Vec<TourStep>) {
        self.tours.insert(name.into(), TourState::new(steps));
    }

    /// Start a named tour. If another tour is active, it is stopped first
    /// (emitting a `TourInterrupted` event).
    ///
    /// Returns events from stopping the previous tour (if any).
    /// Returns an empty vec if the tour name doesn't exist.
    pub fn start(&mut self, name: &str) -> Vec<TourManagerEvent> {
        if !self.tours.contains_key(name) {
            return vec![];
        }

        let mut events = Vec::new();

        // Stop current tour if one is active
        if let Some(current_name) = self.active_tour.take() {
            if let Some(current_state) = self.tours.get_mut(&current_name) {
                if current_state.is_active() {
                    let step = current_state.step_index().0;
                    let tour_events = current_state.update(TourMessage::Skip);
                    self.completed.insert(current_name.clone());
                    for event in tour_events {
                        events.push(TourManagerEvent::Tour {
                            name: current_name.clone(),
                            event,
                        });
                    }
                    events.push(TourManagerEvent::TourInterrupted {
                        name: current_name,
                        at_step: step,
                    });
                }
            }
        }

        // Start the new tour
        if let Some(state) = self.tours.get_mut(name) {
            state.start();
            if state.is_active() {
                self.active_tour = Some(name.to_string());
            }
        }

        events
    }

    /// Stop the currently active tour (if any) by skipping it.
    pub fn stop(&mut self) -> Vec<TourManagerEvent> {
        let mut events = Vec::new();

        if let Some(name) = self.active_tour.take() {
            if let Some(state) = self.tours.get_mut(&name) {
                let tour_events = state.update(TourMessage::Skip);
                for event in &tour_events {
                    if matches!(event, TourEvent::Completed | TourEvent::Skipped { .. }) {
                        self.completed.insert(name.clone());
                    }
                }
                for event in tour_events {
                    events.push(TourManagerEvent::Tour {
                        name: name.clone(),
                        event,
                    });
                }
            }
        }

        events
    }

    /// Route a message to the currently active tour.
    ///
    /// Returns lifecycle events tagged with the tour name.
    /// Automatically tracks completion.
    pub fn update(&mut self, message: TourManagerMessage) -> Vec<TourManagerEvent> {
        let name = match &self.active_tour {
            Some(n) => n.clone(),
            None => return vec![],
        };

        let state = match self.tours.get_mut(&name) {
            Some(s) => s,
            None => return vec![],
        };

        let tour_events = state.update(message.0);

        let mut events = Vec::new();
        for event in &tour_events {
            if matches!(event, TourEvent::Completed | TourEvent::Skipped { .. }) {
                self.completed.insert(name.clone());
                self.active_tour = None;
            }
        }

        for event in tour_events {
            events.push(TourManagerEvent::Tour {
                name: name.clone(),
                event,
            });
        }

        events
    }

    /// Returns true if any tour is currently active.
    pub fn is_active(&self) -> bool {
        self.active_tour.is_some()
    }

    /// Returns the name of the currently active tour, if any.
    pub fn active_tour(&self) -> Option<&str> {
        self.active_tour.as_deref()
    }

    /// Returns a reference to the `TourState` of the currently active tour.
    pub fn active_state(&self) -> Option<&TourState> {
        self.active_tour
            .as_ref()
            .and_then(|name| self.tours.get(name))
    }

    /// Returns true if a specific named tour has been completed or skipped.
    pub fn is_completed(&self, name: &str) -> bool {
        self.completed.contains(name)
    }

    /// Returns the set of completed tour names.
    pub fn completed_tours(&self) -> &HashSet<String> {
        &self.completed
    }

    /// Mark a tour as completed without running it (e.g., from saved preferences).
    pub fn mark_completed(&mut self, name: &str) {
        self.completed.insert(name.to_string());
    }

    /// Reset completion status for a specific tour.
    pub fn reset_completion(&mut self, name: &str) {
        self.completed.remove(name);
    }

    /// Returns a reference to a named tour's state, if it exists.
    pub fn get(&self, name: &str) -> Option<&TourState> {
        self.tours.get(name)
    }

    /// Returns a mutable reference to a named tour's state.
    ///
    /// Useful for updating step targets dynamically (e.g., after layout).
    pub fn get_mut(&mut self, name: &str) -> Option<&mut TourState> {
        self.tours.get_mut(name)
    }

    /// Returns the number of registered tours.
    pub fn tour_count(&self) -> usize {
        self.tours.len()
    }

    /// Returns true if a tour with the given name is registered.
    pub fn has_tour(&self, name: &str) -> bool {
        self.tours.contains_key(name)
    }

    /// Returns a mutable reference to the active tour's state, if any.
    pub fn active_state_mut(&mut self) -> Option<&mut TourState> {
        self.active_tour
            .as_ref()
            .and_then(|name| self.tours.get_mut(name))
    }

    /// Set the resolved bounds for the active tour's current step.
    /// Call this after `visible_bounds()` returns the widget's rectangle.
    pub fn set_resolved_bounds(&mut self, bounds: iced::Rectangle) {
        if let Some(state) = self.active_state_mut() {
            state.set_resolved_bounds(bounds);
        }
    }

    /// Set resolved bounds with animation from previous position.
    pub fn set_resolved_bounds_animated(
        &mut self,
        bounds: iced::Rectangle,
        config: &crate::animation::TourAnimation,
    ) {
        if let Some(state) = self.active_state_mut() {
            state.set_resolved_bounds_animated(bounds, config);
        }
    }

    /// Returns true if any spotlight animation is in progress.
    pub fn is_animating(&self) -> bool {
        self.active_state()
            .map(|s| s.is_animating(std::time::Instant::now()))
            .unwrap_or(false)
    }

    /// Clear resolved bounds for the active tour (e.g., on window resize).
    pub fn clear_resolved_bounds(&mut self) {
        if let Some(state) = self.active_state_mut() {
            state.clear_resolved_bounds();
        }
    }

    /// Returns the widget ID that needs bounds resolution for the current step,
    /// or `None` if the current step doesn't use widget ID targeting or bounds
    /// are already resolved.
    pub fn pending_widget_id(&self) -> Option<&str> {
        let state = self.active_state()?;
        if state.needs_bounds_resolution() {
            state.current_step()?.widget_id()
        } else {
            None
        }
    }

    /// Returns an `iced::Task` that resolves the bounds of the current step's
    /// widget ID target. Returns `Task::none()` if no resolution is needed.
    ///
    /// The caller should map the result to their app's Message type and
    /// call `set_resolved_bounds()` when it arrives.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // In your update() function, after handling Tour messages:
    /// let task = self.tour_manager.resolve_bounds_task(Message::TourBoundsResolved);
    /// return task;
    /// ```
    pub fn resolve_bounds_task<Message: Send + 'static>(
        &self,
        on_resolved: impl Fn(iced::Rectangle) -> Message + Send + 'static,
    ) -> iced::Task<Message> {
        match self.pending_widget_id() {
            Some(id) => {
                let widget_id = iced::widget::Id::from(id.to_string());
                crate::bounds::visible_bounds(widget_id).map(move |bounds| match bounds {
                    Some(rect) => on_resolved(rect),
                    None => on_resolved(iced::Rectangle::default()),
                })
            }
            None => iced::Task::none(),
        }
    }
}

impl Default for TourManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tour_steps;

    fn sample_manager() -> TourManager {
        TourManager::new()
            .add_tour(
                "welcome",
                tour_steps!["Hi" => "Welcome", "Bye" => "Goodbye"],
            )
            .add_tour(
                "editor",
                tour_steps!["Edit" => "Editor", "Save" => "Saving", "Done" => "Finished"],
            )
    }

    #[test]
    fn new_manager_is_empty() {
        let mgr = TourManager::new();
        assert_eq!(mgr.tour_count(), 0);
        assert!(!mgr.is_active());
        assert!(mgr.active_tour().is_none());
    }

    #[test]
    fn add_tour_registers_tours() {
        let mgr = sample_manager();
        assert_eq!(mgr.tour_count(), 2);
        assert!(mgr.has_tour("welcome"));
        assert!(mgr.has_tour("editor"));
        assert!(!mgr.has_tour("nonexistent"));
    }

    #[test]
    fn start_activates_tour() {
        let mut mgr = sample_manager();
        mgr.start("welcome");
        assert!(mgr.is_active());
        assert_eq!(mgr.active_tour(), Some("welcome"));
        assert!(mgr.active_state().unwrap().is_active());
    }

    #[test]
    fn start_nonexistent_tour_does_nothing() {
        let mut mgr = sample_manager();
        let events = mgr.start("nonexistent");
        assert!(events.is_empty());
        assert!(!mgr.is_active());
    }

    #[test]
    fn starting_new_tour_interrupts_current() {
        let mut mgr = sample_manager();
        mgr.start("welcome");
        let events = mgr.start("editor");

        // Should have interrupted welcome tour
        assert!(events.iter().any(|e| matches!(
            e,
            TourManagerEvent::TourInterrupted { name, .. } if name == "welcome"
        )));

        assert_eq!(mgr.active_tour(), Some("editor"));
        assert!(mgr.is_completed("welcome")); // interrupted = completed
    }

    #[test]
    fn update_routes_to_active_tour() {
        let mut mgr = sample_manager();
        mgr.start("welcome");

        let events = mgr.update(TourMessage::Next.into());
        assert!(events.iter().any(|e| matches!(
            e,
            TourManagerEvent::Tour { name, event: TourEvent::StepEntered { index: 1 } } if name == "welcome"
        )));
    }

    #[test]
    fn update_when_no_active_tour_returns_empty() {
        let mut mgr = sample_manager();
        let events = mgr.update(TourMessage::Next.into());
        assert!(events.is_empty());
    }

    #[test]
    fn completing_tour_tracks_completion() {
        let mut mgr = sample_manager();
        mgr.start("welcome");
        mgr.update(TourMessage::Next.into()); // step 1
        let events = mgr.update(TourMessage::Next.into()); // finish

        assert!(mgr.is_completed("welcome"));
        assert!(!mgr.is_active());
        assert!(events.iter().any(|e| matches!(
            e,
            TourManagerEvent::Tour {
                event: TourEvent::Completed,
                ..
            }
        )));
    }

    #[test]
    fn skipping_tour_tracks_completion() {
        let mut mgr = sample_manager();
        mgr.start("editor");
        mgr.update(TourMessage::Skip.into());
        assert!(mgr.is_completed("editor"));
        assert!(!mgr.is_active());
    }

    #[test]
    fn stop_skips_active_tour() {
        let mut mgr = sample_manager();
        mgr.start("welcome");
        let events = mgr.stop();

        assert!(!mgr.is_active());
        assert!(mgr.is_completed("welcome"));
        assert!(!events.is_empty());
    }

    #[test]
    fn stop_when_inactive_returns_empty() {
        let mut mgr = sample_manager();
        let events = mgr.stop();
        assert!(events.is_empty());
    }

    #[test]
    fn mark_completed_without_running() {
        let mut mgr = sample_manager();
        mgr.mark_completed("welcome");
        assert!(mgr.is_completed("welcome"));
        assert!(!mgr.is_completed("editor"));
    }

    #[test]
    fn reset_completion() {
        let mut mgr = sample_manager();
        mgr.mark_completed("welcome");
        assert!(mgr.is_completed("welcome"));
        mgr.reset_completion("welcome");
        assert!(!mgr.is_completed("welcome"));
    }

    #[test]
    fn completed_tours_returns_set() {
        let mut mgr = sample_manager();
        mgr.mark_completed("welcome");
        mgr.mark_completed("editor");
        assert_eq!(mgr.completed_tours().len(), 2);
    }

    #[test]
    fn get_returns_tour_state() {
        let mgr = sample_manager();
        let state = mgr.get("welcome").unwrap();
        assert_eq!(state.steps().len(), 2);
        assert!(mgr.get("nonexistent").is_none());
    }

    #[test]
    fn insert_tour_on_mutable_ref() {
        let mut mgr = TourManager::new();
        mgr.insert_tour("dynamic", tour_steps!["A" => "First"]);
        assert!(mgr.has_tour("dynamic"));
    }

    #[test]
    fn default_creates_empty_manager() {
        let mgr = TourManager::default();
        assert_eq!(mgr.tour_count(), 0);
    }

    #[test]
    fn replacing_tour_keeps_new_version() {
        let mgr = TourManager::new()
            .add_tour("test", tour_steps!["A" => "One"])
            .add_tour("test", tour_steps!["B" => "Two", "C" => "Three"]);

        assert_eq!(mgr.tour_count(), 1);
        assert_eq!(mgr.get("test").unwrap().steps().len(), 2);
    }
}
