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

## Video Search Backend

- Currently uses Invidious API (`/api/v1/search?q=...`) via `InvidiousClient` in `src/invidious/`. This works in automated tests (via wiremock) but **not in production** — see "Abandoning Invidious" below.
- `App::update()` returns `iced::Task<Message>` — the `Message::Search` arm spawns a Tokio runtime inside `Task::perform` because Iced's executor is not a Tokio runtime (reqwest needs one).
- `iced_test::Simulator` does **not** execute `Task`s returned from `update()`. Cucumber steps must manually call `world.app.client.search()` and send `Message::SearchResultsReceived` to simulate the async flow. The tests use the app's own `client` field (pointed at a wiremock `MockServer`) to stay as close to the real code path as possible.

### Abandoning Invidious

Public Invidious instances are locked down — tested multiple instances (yewtu.be, inv.nadeko.net, invidious.nerdvpn.de, iv.nbohr.de) and got 403/401/connection errors across the board. A browser-like User-Agent didn't help either. Invidious is not viable as a backend for a desktop app.

### Next step: Direct InnerTube API

The plan is to replace the Invidious backend with direct calls to YouTube's InnerTube API (`https://www.youtube.com/youtubei/v1/search`). This is the same private API that YouTube's own frontend uses — no API key required. FreeTube uses this approach (via youtubei.js). We'll implement it ourselves with reqwest to avoid depending on stale third-party crates (rustypipe, rusty_ytdl — both ~10 months without updates as of 2026-04).

The `VideoService` trait and wiremock test setup should make swapping the backend straightforward.

## Current work-in-progress

Error alert modal (branch: `search-video`) is complete. The alert displays via `stack!` overlay with a semi-transparent backdrop when `error_message` is `Some`. The dismiss button is centered via `.align_x(Alignment::Center)` on the column.

Next up: replace the Invidious client with a direct InnerTube client to unblock the search happy path in production.

## Key Details

- Rust edition 2024 — uses the latest edition features
- Dependencies: `iced = "0.14.0"`, dev-dep `iced_test`, `tokio` (runtime for reqwest inside Iced tasks)
- Master branch is `master`
