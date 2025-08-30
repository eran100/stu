## Agent Contract (Must Follow)

- Follow the **StartX → async → CompleteX** event pattern.  
  Always toggle `self.is_loading` and emit notifications.  
- Touch only the **smallest set of files**. Prefer minimal diffs.  
- Tests:
  - **No network calls** — always mocks/fakes.
  - Add/keep table tests with `rstest`.
- CI gates must pass locally:
  - `cargo fmt --all -- --check`
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `cargo build`
  - `cargo test`
- Respect **MSRV** from `Cargo.toml`.  
- Naming conventions:  
  - Events: `StartX` / `CompleteX`  
  - Modules/files: `snake_case`  
  - Types/enums: `CamelCase`
- UI rules:
  - Help hides header.  
  - Footer shows latest notification unless Help is visible.  
  - Show `LoadingDialog` when loading.  
- Config & keybindings:
  - Always resolve via `Config` APIs.  
  - **Never hardcode paths**.

## Copilot instructions for this repo

This is a Rust TUI app for browsing S3. Agents should follow the project’s event-driven architecture and CI rules to keep changes easy to review and ship.

### Big picture
- Core loop: `src/run.rs` draws the UI and processes `AppEventType` from an async mpsc channel (see `src/event.rs`).
- State and actions live in `App<C: Client>` (`src/app.rs`). UI pages are a stack of `Page` enum variants (`src/pages/**`) rendered with ratatui widgets (`src/widget/**`).
- Input: crossterm key events → `UserEventMapper` (`src/keys.rs`) → current page handler.
- I/O: AWS S3 operations via the `Client` trait (`src/client.rs`), with an `AwsSdkClient` implementation. Long-running ops are spawned and report back via “Complete*” events.

### Build, test, run
- Minimum supported Rust: from `Cargo.toml` `rust-version` (1.87.0). CI builds on MSRV and stable.
- Before pushing, ensure all pass:
  - Format: `cargo fmt --all -- --check`
  - Lint: `cargo clippy --all-targets --all-features -- -D warnings`
  - Build: `cargo build`
  - Test: `cargo test`
- Run locally (examples):
  - `cargo run -- --debug` writes logs to `~/.stu/debug.log`
  - `cargo run -- --bucket my-bucket --prefix logs/`
  - Use env vars like `AWS_PROFILE`, `AWS_REGION`, and optional `--endpoint-url`.

### Project conventions
- Errors: wrap with `AppError` (`src/error.rs`). UI notifications are set via `App::info/success/warn/error_notification`. Errors are logged to `~/.stu/error.log`.
- Async tasks: In `App` methods, spawn AWS calls and send a matching `Complete*` event. Always manage `self.is_loading` and clear/emit notifications accordingly.
- User input: Map in `assets/keybindings.toml`, optionally overridden by `~/.stu/keybindings.toml` (path via `Config::keybindings_file_path()`). Use `UserEventMapper` APIs to display keys in help/status.
- Config: Load from `$STU_ROOT_DIR/config.toml` or `~/.stu/config.toml` (`src/config.rs`). Includes `download_dir`, `default_region`, and `max_concurrent_requests` for bulk copy/download.
- UI: Hide header on Help page. Footer shows either help or the latest notification. Use `LoadingDialog` overlay when `is_loading` is true.

### Adding a new feature (pattern)
1) Input: add a `UserEvent` if needed (`src/keys.rs`) and bind it in `assets/keybindings.toml`.
2) Intent → event: have the page send an `AppEventType::{Start..., ...}` via `tx` (see page implementations), or handle in `run.rs` if it’s global.
3) Side effect: implement an `App` method that spawns the work (e.g., call `client.*`) and then sends a `Complete*` event; set/reset `is_loading`.
4) Result: handle `AppEventType::Complete*` in `App` to mutate state (push/pop `Page`, update caches in `AppObjects`), and emit notifications.
5) Render: update the appropriate page/widget to reflect new state; keep diffs minimal.

Example events already follow this flow:
- Load objects: `LoadObjects` → `complete_load_objects`
- Preview/download: `PreviewObject`/`DownloadObject` → `complete_preview_object`/`complete_download_object`
- Versions, copy/paste: `LoadObjectVersions`/`PasteObject` → respective `complete_*` handlers

### Integration details
- S3 addressing style: CLI `--path-style` maps to `client::AddressingStyle` (Auto/Always/Never). Endpoint URL support for S3-compatible storage.
- Progress: large downloads (> ~10MB) emit periodic `NotifyInfo` messages with humanized sizes.
- Management Console: open URLs via `Client::open_management_console_*` helpers.

### Where to look
- Entrypoint and CLI: `src/main.rs`
- Event bus: `src/event.rs`
- App state/logic: `src/app.rs`
- Client and AWS calls: `src/client.rs`
- Pages: `src/pages/**`, Widgets: `src/widget/**`
- Config and keybindings: `src/config.rs`, `src/keys.rs`, `assets/keybindings.toml`

Keep changes aligned with the above patterns. If you introduce new events or pages, mirror the existing naming (`StartX`, `CompleteX`) and wiring.