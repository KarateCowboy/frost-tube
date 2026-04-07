# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Frost Tube is a desktop GUI application built with Rust (2024 edition) and the [Iced](https://iced.rs/) UI framework (v0.14). It is a Rust-based competitor to the electron-based FreeTube.

## Build & Run Commands

- **Build:** `cargo build`
- **Run:** `cargo run`
- **Run tests:** `cargo test`
- **Run a single test:** `cargo test test_name`
- **Check (no compile):** `cargo check`
- **Run BDD tests:** `cargo test --test cucumber`
- **Run rectum tests:** `cd rectum && cargo test`

## Architecture

The app uses the Iced Elm-like architecture pattern:

- **`App` struct** — holds application state
- **`Message` enum** — defines all UI/async events
- **`App::update()`** — handles messages and mutates state
- **`App::view()`** — renders the UI from current state
- **`App::theme()`** — returns `Theme::Dark`

Entry point is `src/main.rs` using `iced::application()` with the title "Frost Tube".

### Pages

- **`Page::Index`** — search input + Go button
- **`Page::SearchResults`** — clickable list of video results
- **`Page::VideoDetail`** — video metadata (title, author, views, duration, description) with Back button

### Async pattern

`App::update()` returns `iced::Task<Message>`. Async work (search, video detail fetch) spawns a Tokio runtime inside `Task::perform` because Iced's executor is not a Tokio runtime (reqwest needs one). `iced_test::Simulator` does **not** execute `Task`s — Cucumber steps must manually call the client and send the result message.

## Testing

- **Test-driven development (TDD):** Follow the red-green-refactor cycle. Write a failing test first (red), write the minimum code to make it pass (green), then refactor. Do not skip ahead.
- **Teaching Driven Development:** Do NOT write code to files unless explicitly asked. Instead, explain the problem, suggest the solution with code snippets, and let the user key it in themselves. This reinforces understanding. Only write to files when the user says to.
- **BDD with Cucumber:** Scenarios live in `tests/features/*.feature`. Step definitions are in `tests/steps/`. The test harness is `tests/cucumber.rs`.
- **UI testing with `iced_test`:** Use the `Simulator` API for headless widget tree inspection. The `&str` selector finds widgets by visible text and returns `selector::target::Text::Input` for text inputs or `Text::Raw` for plain text — use `matches!` to assert the widget type.
- **Don't over-abstract:** Keep helpers inline in steps until repetition justifies extraction.

## Rectum — InnerTube API Library

`rectum/` is a git submodule (`github.com/karatecowboy/rectum`) — a standalone Rust library wrapping YouTube's InnerTube API. All YouTube API interaction goes through this library.

### Completed milestones

- **M0: Scaffolding** — module structure, deps, CI
- **M1: Core Infrastructure** — `InnerTubeClient` with builder pattern, `Error` enum (thiserror), `InnerTubeContext` for client impersonation (Web, Android, Ios, TvEmbedded)
- **M2: Search** — `client.search(query)` → `SearchResults` with `VideoResult` items. Raw serde types with `TryFrom` conversion to clean public API.
- **M3: Video Details** — `client.get_video(video_id)` → `VideoDetails` (title, author, channel_id, views, duration, description, keywords, thumbnails)

### Remaining milestones

- **M4: Streaming URLs** — get playable video/audio URLs
- **M5-M8:** Browse, related videos, comments, URL resolution

See `rectum/ROADMAP.md` for details.

### Key patterns

- Builder: `InnerTubeClient::builder().base_url(url).build()` — base_url is how tests point at wiremock
- Fixture-based testing: real InnerTube JSON responses saved in `rectum/tests/fixtures/`
- Private `raw::` serde types mirror InnerTube's nested JSON; `TryFrom` converts to clean public types

## Current status

**Working features:**
- Video search via InnerTube (production-ready)
- Search results → click → video detail page with full metadata
- Back navigation from detail to results
- Error modal on API failure

**Next up: Video playback**

The plan is to use `iced_video_player` (v0.6, targets Iced 0.14) backed by GStreamer for embedded video playback. This requires:
1. `brew install gstreamer` on the dev machine
2. Rectum M4 (streaming URL resolution) to get playable URLs
3. Pass streaming URLs to `iced_video_player::Video::new(url)`

GStreamer was chosen over libmpv (no Iced integration, OpenGL-only on macOS) and libvlc (Rust bindings unmaintained since 2018). `iced_video_player` is the only maintained Iced video widget — it uses GStreamer's appsink with YUV→RGB GPU shaders.

See `ROADMAP.md` for the full feature parity roadmap targeting FreeTube.

## Key Details

- Rust edition 2024
- Dependencies: `iced = "0.14.0"`, `rectum` (path submodule), `tokio`
- Dev deps: `cucumber`, `iced_test`, `wiremock`, `serde_json`
- Master branch is `master`
