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
- **BDD with Cucumber:** Scenarios live in `tests/features/*.feature`. Step definitions are in `tests/steps/`. The test harness is `tests/cucumber.rs`.
- **Run BDD tests:** `cargo test --test cucumber`
- **UI testing with `iced_test`:** Use the `Simulator` API for headless widget tree inspection. The `&str` selector finds widgets by visible text and returns `selector::target::Text::Input` for text inputs or `Text::Raw` for plain text — use `matches!` to assert the widget type.
- **Don't over-abstract:** Keep helpers inline in steps until repetition justifies extraction.

## Key Details

- Rust edition 2024 — uses the latest edition features
- Dependencies: `iced = "0.14.0"`, dev-dep `iced_test`
- Master branch is `master`
