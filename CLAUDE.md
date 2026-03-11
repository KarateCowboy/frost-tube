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

## Development Practice

- **Test-driven development (TDD):** Write the test first, then make it pass, whenever possible.

## Key Details

- Rust edition 2024 — uses the latest edition features
- Only dependency is `iced = "0.14.0"`
- Master branch is `master`
