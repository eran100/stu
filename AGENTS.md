# Repository Guidelines

## Quick File Map
- `src/main.rs`: Entry point and CLI flags; debug logging setup.
- `src/run.rs`: Renders UI and processes `AppEventType` from the channel.
- `src/event.rs`: Event types and async channel (`tx`/`rx`).
- `src/app.rs`: App state, `Page` stack, handlers, notifications, error logging.
- `src/keys.rs`: `UserEventMapper`; keybinding resolution from assets or `$STU_ROOT_DIR`.

## Project Structure & Module Organization
- Source: `src/` (entrypoint `src/main.rs`, app logic in `src/app.rs`, client in `src/client.rs`, pages/widgets under `src/pages/**` and `src/widget/**`).
- Assets: `assets/` (default keybindings: `assets/keybindings.toml`).
- Docs and images: `docs/`, `img/`.
- Tools: `tool/` (Go helpers for screenshots); Make targets only for generating docs images.

## Architecture Overview
- Core loop: `src/run.rs` renders the UI and consumes `AppEventType` from the async channel in `src/event.rs`.
- State: `App<C: Client>` (`src/app.rs`) manages data and a `Page` stack (`src/pages/**`); widgets live in `src/widget/**`.
- Input: crossterm key events mapped via `UserEventMapper` (`src/keys.rs`) using keybindings from `assets/` or `$STU_ROOT_DIR`.
- I/O: AWS S3 operations via the `Client` trait (`src/client.rs`); long tasks spawn and report back with `StartX`/`CompleteX` events.
- Config & logs: `src/config.rs` loads `$STU_ROOT_DIR/config.toml`; `--debug` writes to `~/.stu/debug.log`, errors to `~/.stu/error.log`.

## Build, Test, and Development Commands
- Format: `cargo fmt --all -- --check` (CI gate; see `rustfmt.toml`).
- Lint: `cargo clippy --all-targets --all-features -- -D warnings`.
- Build: `cargo build` (MSRV 1.87.0). Image tooling: `cargo build --features imggen`.
- Test: `cargo test` (no network I/O; use mocks/fakes).
- Run: `cargo run -- --debug` (writes logs to `~/.stu/debug.log`). Examples: `--bucket my-bucket --prefix logs/`.

## Coding Style & Naming Conventions
- Rust edition 2021 (crate), rustfmt edition set to 2015 with repo-specific options; always run rustfmt.
- Naming: modules/files `snake_case`; types/enums `CamelCase`; follow event pattern `StartX`/`CompleteX` when adding events.
- Clippy: too-many-arguments threshold = 12 (see `clippy.toml`).

## Testing Guidelines
- Frameworks: standard Rust tests with `rstest` for table tests.
- Isolation: avoid network calls; mock the S3 `Client` trait from `src/client.rs`.
- Locations: tests live alongside modules under `src/**`.

## Commit & Pull Request Guidelines
- Commits: prefer Conventional Commits (`feat:`, `fix:`, `docs:`) with concise imperative subjects. Scopes like `feat(client): ...` are welcome.
- PRs: include a clear description, linked issues, test plan, and screenshots for UI changes. Ensure CI gates pass (fmt, clippy, build, test).

## Security & Configuration Tips
- Config root: `$STU_ROOT_DIR` or default `~/.stu`. User keybindings override at `$STU_ROOT_DIR/keybindings.toml`.
- Env expansion: `Config::expand_env_vars` errors on unset non-`STU_ROOT_DIR` vars; set required env vars explicitly in dev/tests.

## Event Flow Checklist
- Bind key in `assets/keybindings.toml` → map in `src/keys.rs`.
- Add `StartX`/`CompleteX` in `src/event.rs` if new.
- From page/app, send `StartX`, spawn async work, then `tx.send(CompleteX(...))` in `src/app.rs`.
- Handle `CompleteX` to update state and render (`src/pages/**`, `src/widget/**`).

## Fake S3 Client (Tests)
- Create `FakeClient` implementing `Client` (`src/client.rs` or `tests/` helper).
- Inject with `App::new(mapper, FakeClient, ctx, tx)` in `rstest`.
- Return canned data; avoid any network calls.

## UI Rules
- Help hides header; footer shows latest notification unless Help is visible.
- Show `LoadingDialog` while `is_loading`; toggle around Start/Complete events.
- Use `App::info/success/warn/error_notification`; expose shortcuts via page `helps()`.

## Config & Sandboxing Tips
- Isolate tests: `STU_ROOT_DIR=$(mktemp -d) cargo test`.
- Logs: debug `Config::debug_log_path()`; errors `Config::error_log_path()`.
- Env vars in paths must exist or expansion fails.

## Keybinding Changes
- Edit `assets/keybindings.toml`, map to `UserEvent` in `src/keys.rs`.
- Wire the handler in the relevant page or `App` method; update page `helps()`.

## Troubleshooting
- Fmt/lint failures: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`.
- Terminal garbled after panic: run `reset`.
- Missing Go for image tools: install Go; image targets are optional.

## Image Tooling
- `make screenshot` / `make demo` generate docs assets (requires Go).
- Not required for building or running the application.
