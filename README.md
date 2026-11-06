# ZapFast UI — reusable egui components

A minimal **UI-only Rust crate** derived/adapted from the visual language and reusable widget patterns in [ZapFast](https://github.com/crmne/zapfast). This branch intentionally contains **no WhatsApp implementation**, login, backend runtime, networking, database, or media stack.

## Reuse

Add as a Git dependency in another Rust/egui app:

```toml
zapfast-ui = { git = "https://github.com/Areo-RGB/zapfast", branch = "ui-only" }
```

```rust
use zapfast_ui::{Palette, widgets};
let palette = Palette::dark();
palette.install(ui.ctx());
widgets::menu_item(ui, &palette, "Settings", false);
```

## Gallery

```bash
cargo run --example gallery
```

## Modules

- `src/theme.rs`: light/dark colors, spacing, egui style installer.
- `src/widgets.rs`: avatar, badge, menu item, menu frame, search field, empty state, message bubble.
- `examples/gallery.rs`: self-contained interactive component showcase.

These are **portable adaptations**, not wholesale copies of the full ZapFast UI: original full-screen panels reference app-specific data and need further extraction if desired. See the original `main` branch for the full app. The source project's license and attribution are preserved.

**Validation status:** The project structure and public API were prepared but have not been cargo-compiled in this environment. Run `cargo check --all-targets` locally to verify your toolchain and dependency compatibility.
