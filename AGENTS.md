# Repository Guidelines

## Project Structure & Module Organization
- `src/`: application code. Entrypoint `src/main.rs`; core modules `app.rs`, `run.rs`, `client.rs`; UI under `pages/` and `widget/`.
- `img/`, `assets/`: screenshots and static assets used in docs/previews.
- `docs/`: documentation sources; not required for local development.
- `tool/` and `Makefile`: screenshot/VRT tools (Go); optional for normal dev.
- Local config: `~/.stu` or `$STU_ROOT_DIR`.

## Architecture Overview
- Event-driven TUI built with `ratatui`; render loop in `run.rs`.
- Input: crossterm -> `AppEventType::Key` -> `UserEventMapper` -> current `Page`.
- Async S3 via `tokio`; results return as `Complete*` events.
- `client::Client` abstracts AWS SDK; `--debug` writes tracing logs to `~/.stu/debug.log`.

## Build, Test, and Development Commands
- `cargo build`: compile (MSRV 1.87; stable supported).
- `cargo run -- --debug --bucket my-bucket`: run the TUI locally.
- `cargo test`: run unit tests.
- `cargo fmt -- --check` / `cargo fmt`: check/fix formatting per `rustfmt.toml`.
- `cargo clippy -- -D warnings`: lint; deny warnings.
- Optional: `make screenshot` regenerates screenshots (requires Go).

## Coding Style & Naming Conventions
- Format with `rustfmt` (4-space indent); keep diffs minimal.
- Lint with `clippy` and fix all warnings.
- Naming: modules/files `snake_case`; types/enums `CamelCase`; functions `snake_case`; constants `SCREAMING_SNAKE_CASE`.
- Prefer `tracing` for logs; add `///` docs on public items when useful.

## Testing Guidelines
- Place unit tests next to code: `#[cfg(test)] mod tests { ... }`.
- Use `rstest` for parameterized/table tests.
- Avoid live AWS calls; use fakes/mocks and small deterministic data.
- Add integration tests under `tests/` for cross-module behavior.

## Commit & Pull Request Guidelines
- Commits: short, imperative subject (e.g., "Add …", "Fix …"); include rationale and link issues (e.g., `Closes #123`).
- PRs: clear description; screenshots/GIFs for UI changes; update docs/help for user-facing changes.
- Ensure `cargo build`, `cargo test`, `cargo fmt`, and `cargo clippy` all pass before requesting review.

## Security & Configuration Tips
- Do not commit credentials or secrets.
- Use `AWS_PROFILE`/`AWS_REGION` and flags (e.g., `--endpoint-url`) for testing.
- Keep sensitive data out of tests and logs.

