//! Widget bounds resolution via iced's Operation system.
//!
//! Provides `visible_bounds(id)` which returns an `iced::Task` that resolves
//! to the screen-space `Rectangle` of a container widget with the given ID.
//!
//! This is the iced equivalent of JavaScript's `getBoundingClientRect()`.

use iced::advanced::widget::operation::{Operation, Outcome};
use iced::widget::Id;
use iced::{Rectangle, Task};

/// Query the visible bounds of a container widget by its ID.
///
/// Returns a `Task` that resolves to the widget's screen-space `Rectangle`,
/// or `None` if no container with that ID exists in the widget tree.
///
/// # Example
///
/// ```ignore
/// // In your view(), tag a widget:
/// container(my_button).id(widget::Id::new("open_video"))
///
/// // In your update(), query its bounds:
/// let task = visible_bounds(widget::Id::new("open_video"))
///     .map(|bounds| Message::BoundsResolved(bounds));
/// ```
pub fn visible_bounds(id: Id) -> Task<Option<Rectangle>> {
    iced::advanced::widget::operate(FindBounds {
        target: id,
        bounds: None,
    })
}

/// Custom Operation that walks the widget tree to find a container's bounds.
struct FindBounds {
    target: Id,
    bounds: Option<Rectangle>,
}

impl Operation<Option<Rectangle>> for FindBounds {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<Option<Rectangle>>)) {
        // Continue traversing if we haven't found our target yet
        if self.bounds.is_none() {
            operate(self);
        }
    }

    fn container(&mut self, id: Option<&Id>, bounds: Rectangle) {
        if id == Some(&self.target) {
            self.bounds = Some(bounds);
        }
    }

    fn finish(&self) -> Outcome<Option<Rectangle>> {
        Outcome::Some(self.bounds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_bounds_starts_empty() {
        let op = FindBounds {
            target: Id::new("test"),
            bounds: None,
        };
        assert!(matches!(op.finish(), Outcome::Some(None)));
    }
}
