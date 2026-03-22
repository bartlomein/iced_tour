# iced_tour

Guided tour / onboarding overlay for iced 0.14 apps.

## How to Integrate (for LLMs)

### Step 1: Analyze the app
- Read the app's main `view()` function to identify major UI panels/areas
- Look for `stack![]` usage — the tour overlay drops into an existing stack
- Identify the app's `Message` enum and `update()` function

### Step 2: Propose tour steps (DO NOT IMPLEMENT YET)
- For each major UI panel, draft a step with:
  - A short title (2-3 words)
  - A one-sentence description of what the panel does
  - Whether to use `target_id("widget_id")` or no target (centered)
- Present the proposed steps to the user in a numbered list
- Ask: "Does this look right? Want to add, remove, or reword any steps?"

### Step 3: STOP and wait for user approval
- Do NOT write any code until the user approves the proposed steps
- The user may want to reword descriptions, reorder steps, or skip panels
- Steps without `.target()` or `.target_id()` show centered cards — this is fine for intro/outro steps

### Step 4: Implement (only after approval)
After the user approves, add these to the app:

1. Add `iced_tour` to Cargo.toml dependencies
2. Add `tour_manager: TourManager` field to the App struct
3. Add `Tour(TourManagerMessage)` variant to the Message enum
4. In `new()`: create TourManager with named tours and steps
5. In `view()`: wrap target widgets in `container(widget).id(widget::Id::new("my_id"))`
6. In `view()`: add `tour_manager_overlay(&self.tour_manager, &self.tour_theme, Message::Tour)` to the stack
7. In `update()`: handle `Message::Tour(msg)` by calling `self.tour_manager.update(msg)`
8. After tour messages: dispatch `self.tour_manager.resolve_bounds_task(Message::TourBoundsResolved)`
9. Handle `TourBoundsResolved(bounds)` by calling `self.tour_manager.set_resolved_bounds_animated(bounds, &theme.animation)`
10. In `subscription()`: add 16ms tick when `self.tour_manager.is_animating()`
11. Trigger on first launch: check a persisted flag, call `tour_manager.start("tour_name")` if first time
12. Persist completion: save a flag when tour finishes so it doesn't repeat

### Quick Integration Template

```rust
// 1. Cargo.toml
iced_tour = { path = "crates/iced_tour" }  // or version = "0.1"

// 2. App struct
tour_manager: TourManager,
tour_theme: TourTheme,

// 3. Message enum
Tour(TourManagerMessage),
TourBoundsResolved(Rectangle),
TourAnimationTick,

// 4. Initialization
let tour_manager = TourManager::new()
    .add_tour("welcome", vec![
        TourStep::new("Welcome", "Let's take a quick tour"),
        TourStep::new("Open Video", "Click here to import")
            .target_id("open_video")
            .card_position(CardPosition::Right),
    ]);

// 5. View — wrap target widgets with container IDs
container(my_button).id(widget::Id::new("open_video"))

// 6. View — add overlay to your existing stack
stack![
    self.view_editor(),
    tour_manager_overlay(&self.tour_manager, &self.tour_theme, Message::Tour),
]

// 7-9. Update
Message::Tour(msg) => {
    self.tour_manager.update(msg);
    return self.tour_manager.resolve_bounds_task(Message::TourBoundsResolved);
}
Message::TourBoundsResolved(bounds) => {
    self.tour_manager.set_resolved_bounds_animated(bounds, &self.tour_theme.animation);
}
Message::TourAnimationTick => {} // no-op, triggers redraw

// 10. Subscription
if self.tour_manager.is_animating() {
    subscriptions.push(
        iced::time::every(Duration::from_millis(16))
            .map(|_| Message::TourAnimationTick)
    );
}

// 11. First launch trigger
if !preferences.tour_complete {
    self.tour_manager.start("welcome");
}
```

## API Reference

### Core Types
- `TourManager` — manages multiple named tours with completion tracking
- `TourState` — state machine for a single tour
- `TourStep` — individual step with builder pattern
- `TourTarget` — `None` (centered), `Manual(Rectangle)`, `WidgetId(String)`
- `TourMessage` — Next, Back, Skip, Finish, BackdropClicked
- `TourManagerMessage` — wraps TourMessage for routing through TourManager
- `TourEvent` — StepEntered, StepExited, Completed, Skipped
- `TourManagerEvent` — enriched events with tour name

### Step Builders
- `TourStep::new(title, description)` — centered card (no cutout)
- `.target(rect)` — spotlight a fixed rectangle
- `.target_id("widget_id")` — spotlight a widget by container ID (resolved at runtime)
- `.card_position(pos)` — control card placement (Auto/Top/Bottom/Left/Right)

### Theme & Animation
- `TourTheme::dark()` / `TourTheme::light()` — preset themes
- `TourTheme::default()` — dark theme
- `.with_animation(TourAnimation::default())` — configure animation (300ms EaseOutCubic)
- `.with_animation(TourAnimation::none())` — disable animation
- All theme properties have `.with_*()` builder methods

### Rendering
- `tour_overlay(state, theme, msg_mapper)` — single-tour overlay
- `tour_manager_overlay(manager, theme, msg_mapper)` — multi-tour overlay

### Bounds Resolution
- `visible_bounds(widget_id)` — query widget bounds via iced Operation
- `TourManager::resolve_bounds_task(on_resolved)` — returns Task for bounds resolution
- `TourManager::set_resolved_bounds_animated(bounds, config)` — set bounds with smooth animation

### Utilities
- `tour_steps![...]` — convenience macro for quick step definitions
- `integration_checklist(&state, &theme)` — debug helper, prints what's missing
