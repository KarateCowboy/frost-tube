# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Frost Tube is a desktop GUI application built with Rust (2024 edition) and the [Iced](https://iced.rs/) UI framework (v0.14). It is a Rust-based competitor to the electron-based FreeTube

## Build & Run Commands

- **Build:** `cargo build`
- **Run:** `cargo run`
- **Run tests:** `cargo test`
- **Run a single test:** `cargo test test_name`
- **Check (no compile):** `cargo check`

## Architecture

The app uses the Iced Elm-like architecture pattern:

- **`App` struct** — holds application state
- **`Message` enum** — defines all UI/async events
- **`App::update()`** — handles messages and mutates state
- **`App::view()`** — renders the UI from current state
- **`App::theme()`** — returns `Theme::Dark`

Entry point is `src/main.rs` using `iced::application()` with the title "Frost Tube".

## Testing

- **Test-driven development (TDD):** Follow the red-green-refactor cycle. Write a failing test first (red), write the minimum code to make it pass (green), then refactor. Do not skip ahead.
- **Teaching Driven Development:** Do NOT write code to files unless explicitly asked. Instead, explain the problem, suggest the solution with code snippets, and let the user key it in themselves. This reinforces understanding. Only write to files when the user says to.
- **BDD with Cucumber:** Scenarios live in `tests/features/*.feature`. Step definitions are in `tests/steps/`. The test harness is `tests/cucumber.rs`.
- **Run BDD tests:** `cargo test --test cucumber`
- **UI testing with `iced_test`:** Use the `Simulator` API for headless widget tree inspection. The `&str` selector finds widgets by visible text and returns `selector::target::Text::Input` for text inputs or `Text::Raw` for plain text — use `matches!` to assert the widget type.
- **Don't over-abstract:** Keep helpers inline in steps until repetition justifies extraction.

## Invidious Integration

- Search uses the Invidious API (`/api/v1/search?q=...`) via `InvidiousClient` in `src/invidious/`.
- `App::update()` returns `iced::Task<Message>` — the `Message::Search` arm spawns a Tokio runtime inside `Task::perform` because Iced's executor is not a Tokio runtime (reqwest needs one).
- `iced_test::Simulator` does **not** execute `Task`s returned from `update()`. Cucumber steps must manually call `world.app.client.search()` and send `Message::SearchResultsReceived` to simulate the async flow. The tests use the app's own `client` field (pointed at a wiremock `MockServer`) to stay as close to the real code path as possible.

### Current blocker

Public Invidious instances return **403 Forbidden** for API requests from non-browser User-Agents. The default instance is `https://yewtu.be`. Next steps to unblock:
- Try a browser-like User-Agent string, or
- Let the user configure their own Invidious instance URL (some instances allow API access)

## Current work-in-progress

Error alert modal (branch: `search-video`). The alert displays via `stack!` overlay when `error_message` is `Some`. The button rendering is fixed (no spacing/padding on the alert column). Next steps:
1. Remove the duplicate inline error text from the `SearchResults` page view (`src/lib.rs` lines 72-74) so it doesn't bleed through behind the alert
2. Decide whether to add a full-screen semi-transparent backdrop layer to the `stack!` for a proper modal look
3. The Iced `column!` `.spacing()` and `.padding()` caused the button label to render outside the button — left off for now, may revisit once layout is understood better

## Key Details

- Rust edition 2024 — uses the latest edition features
- Dependencies: `iced = "0.14.0"`, dev-dep `iced_test`, `tokio` (runtime for reqwest inside Iced tasks)
- Master branch is `master`
