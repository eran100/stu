# Copilot instructions for this repo

Purpose: STU is a terminal UI for browsing S3 (and compatible endpoints) built with Ratatui. Focus changes around the event-driven UI loop, page stack, and the S3 client abstraction.

Architecture quick map
- Entry and loop: `src/main.rs` parses flags, builds `Config`, `Environment`, key mapper, and S3 `Client`; sets up event channel from `src/event.rs`; then `src/run.rs` renders and drives the main loop.
- Events: `AppEventType` in `src/event.rs` is the backbone. Long ops follow Start/Do/Complete patterns (e.g., `StartDownloadObject` → `DownloadObject` → `CompleteDownloadObject`). Use the provided `Complete*Result::new(...)` helpers to propagate errors early.
- State: `App<C: Client>` in `src/app.rs` holds the page stack, key mapper, `AppObjects` caches, `notification`, and `is_loading`. Async work is spawned inside `App` methods; results are sent back via `Sender`.
- Pages: `src/pages/page.rs` defines `Page` enum and `PageStack`; concrete page types live under `src/pages/**`. Use `Page::of_*` constructors and push/pop onto the stack; footer shows `short_helps`, header hides on Help page.
- Input mapping: `src/keys.rs` builds a `UserEventMapper` from `assets/keybindings.toml` merged with `$STU_ROOT_DIR/keybindings.toml`. Map crossterm `KeyEvent` → `UserEvent` per section (bucket_list, object_list, etc.).
- S3: `src/client.rs` defines the `Client` trait and the AWS SDK implementation (addressing style, pagination, copy, downloads). UI never calls AWS directly—always via `Client`.

Developer workflows
- Build/run: `cargo build`; `cargo run -- --debug [-r us-east-1] [--bucket B --prefix P]`. Debug logs write to `~/.stu/debug.log` when `--debug` is set; errors always to `~/.stu/error.log`.
- Lint/format/tests: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test`. MSRV is 1.87.0.
- Config: `src/config.rs`; file is `$STU_ROOT_DIR/config.toml` (defaults to `~/.stu`). Env expansion is strict: undefined envs error, except `$STU_ROOT_DIR` which resolves to the app dir.
- Image/docs tooling in `tool/` and `Makefile` is optional (used for screenshots only) and requires building with `--features imggen`.

Adding a new user action (end-to-end)
1) Declare a `UserEvent` in `src/keys.rs` and bind it in `assets/keybindings.toml` (or instruct users to override). Ensure it’s included in the correct section; then wire it in `build_user_event_mapper`.
2) Handle it where appropriate: either in `run.rs` pre-handling (for global `Quit`/`DumpApp`) or in the current page via `Page::handle_user_events` → `page.handle_key(...)`.
3) If async work is needed, add `AppEventType::{StartX, X, CompleteX}` (or reuse existing), extend the `match` in `src/run.rs`, and implement `App::{start_x, x, complete_x}` in `src/app.rs`.
4) In long ops, set/reset `is_loading`, send progress via `NotifyInfo`, and on success/failure send `NotifySuccess/NotifyError`. Errors are logged via `Config::error_log_path()`.
5) Update page `helps()`/`short_helps()` to surface the new action and key.

Patterns and gotchas
- Event naming: prefer `StartX` (UI initiates) → `X` (actual async) → `CompleteX(Result<T>)` (UI apply and clear `is_loading`). Reuse result wrappers like `CompleteDownloadObjectResult::new` to bubble errors.
- Caching: object lists/details/versions are cached in `AppObjects`; check cache before enqueuing load events to avoid redundant calls.
- Large transfers: progress notifications are rate-limited in `download_object` and only enabled for files ≥ 10 MB (`handle_loading_size`).
- Concurrency: multi-object operations (download/copy) use `buffered(max_concurrent_requests)`; configurable via `Config.max_concurrent_requests`.
- External open: Management Console URLs are opened via `open` crate methods on the `Client` (`open_management_console_*`).

Testing
- Avoid network I/O. Implement a `FakeClient: Client` returning canned data and inject it into `App::new(...)`. Use `rstest` for tables.
- Sandboxing: run tests with `STU_ROOT_DIR=$(mktemp -d)` to isolate config/log writes.

Useful file pointers
- CLI/entry: `src/main.rs`; Loop: `src/run.rs`; Events/channels: `src/event.rs`.
- App/state/UI orchestration: `src/app.rs`; Pages and stack: `src/pages/page.rs` and `src/pages/**`.
- Key mapping: `src/keys.rs`; Config: `src/config.rs`; Client/S3: `src/client.rs`.

References: README has usage and links to full docs; `AGENTS.md` mirrors these conventions for contributors.
