# AGENTS.md

- Canonical keybinding examples live in [`assets/keybindings.toml`](assets/keybindings.toml:1); docs under `docs/src/keybindings/` explain per-section overrides — prefer the asset file for default behavior.
- Docs are generated assets; `docs/` and `img/` are excluded from crates (see [`Cargo.toml`](Cargo.toml:15)). Do not assume docs are authoritative for runtime behavior without confirming corresponding code in [`src/keys.rs`](src/keys.rs:6) or [`src/config.rs`](src/config.rs:119).
- `docs/src/configurations/config-file-format.md` documents the TOML structure but runtime env expansion occurs in [`src/config.rs`](src/config.rs:119-136); note the special handling of `$STU_ROOT_DIR`.
- Image/docs screenshots are produced by Go tools in `tool/imggen` / `tool/imgdiff`; running `make screenshot` requires Go and the `imggen` fixtures — see `tool/` for expected inputs.
- When referencing examples in docs, cite the exact source file (e.g. [`src/keys.rs`](src/keys.rs:236-244)) — doc prose can be out-of-date.