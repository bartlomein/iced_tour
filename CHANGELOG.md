# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 2026-03-22

### Added

- `minimal` example — simplest possible tour with centered cards
- `widget_id` example — spotlight follows widgets by ID, works with window resize
- `theming` example — dark, light, and custom color themes

### Fixed

- README MSRV now correctly states 1.88 (matching Cargo.toml and iced 0.14)
- Removed hardcoded iced version from description — now says "iced applications"
- Added compatibility section with minimum iced and Rust versions

## [0.1.0] - 2026-03-22

### Added

- `TourState` — state machine for guided tour navigation
- `TourManager` — manage multiple named tours with completion tracking
- `TourStep` — builder pattern for step definitions (title, description, target, position)
- `TourTarget` — `None` (centered), `Manual(Rectangle)`, `WidgetId(String)`
- `TourAnimation` — configurable spotlight transition animations (duration, easing, enable/disable)
- `TourTheme` — dark/light presets with full visual customization and builder API
- `tour_overlay()` / `tour_manager_overlay()` — rendering functions for iced `stack![]`
- `visible_bounds()` — custom Operation for runtime widget bounds resolution
- `tour_steps![]` — convenience macro for quick step definitions
- `integration_checklist()` — debug helper to verify integration
- Widget ID targeting via `target_id()` — spotlight widgets by container ID
- Smooth spotlight animations between steps with configurable easing
- Canvas-based backdrop with EvenOdd fill rule for rounded spotlight cutouts
- Keyboard navigation (Escape, Arrow keys, Enter)
- Lifecycle events: `StepEntered`, `StepExited`, `Completed`, `Skipped`
- Completion tracking via `mark_completed()` / `is_completed()`
- Dot indicators showing tour progress
- Working standalone example (`examples/basic.rs`)
