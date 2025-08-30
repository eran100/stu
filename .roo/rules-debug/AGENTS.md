# AGENTS.md

- Enable debug with `--debug`; initialization happens in [`main.rs`](src/main.rs:115-124) and uses Config::debug_log_path() to open the file (so ensure Config::get_app_base_dir() succeeds before relying on the file).
- Tracing writer is a blocking file writer wrapped in a Mutex via `tracing_subscriber::fmt().with_writer(Mutex::new(file))` — expect serialized writes and no ANSI sequences (`with_ansi(false)`) (see [`src/main.rs`](src/main.rs:119-124)).
- Error logging is synchronous and fatal on failure: App::handle_error saves to `Config::error_log_path()` and unwraps/save_error_log — a failed write will panic the process (see [`src/app.rs`](src/app.rs:976-982)).
- Debug and error log paths are derived from $STU_ROOT_DIR or default to `~/.stu` via `Config::get_app_base_dir()`; tests must sandbox this (see [`src/config.rs`](src/config.rs:168-176) and [`src/config.rs`](src/config.rs:143-156)).