# ZapFast UI — reusable egui shell and widgets

A backend-free Rust UI crate adapted from [ZapFast](https://github.com/crmne/zapfast), primarily the upstream `src/ui/mod.rs`, `src/ui/rail.rs` and `src/ui/pane.rs` composition patterns. The original `main` branch contains the complete application.

## Run the app-shell gallery

```bash
cargo run --example gallery
cargo check --all-targets
```

## Consume from another project

```toml
zapfast-ui = { git = "https://github.com/Areo-RGB/zapfast", branch = "ui-only" }
```

```rust
use zapfast_ui::{Palette, shell::{self, Destination, ShellConfig, ShellState}};
let mut state = ShellState::default();
let tabs = [Destination::new("home", "Home", "⌂")];
// Within an egui/eframe update callback:
shell::show(ctx, &Palette::dark(), &mut state, &tabs,
    ShellConfig::default(),
    |ui| shell::header_title(ui, "My app", None),
    |ui, state| { if ui.button("Open item").clicked() { state.open_detail(); } },
    |ui, _| { ui.label("Your content"); },
    |ui| { ui.label("Your inspector"); });
```

## Extracted pieces

- `src/shell.rs` — app shell, fixed-width navigation rail, resizable sidebar, top header, central detail and optional inspector, narrow viewport behavior. All rendering delegated to closures.
- `src/widgets.rs` — avatar, badge, menu item, menu frame, search field, empty state and message bubble.
- `src/theme.rs` — color palettes and style tokens.
- `examples/gallery.rs` — interactive shell demo showing all basic widgets.

### Design boundaries

- No network, database, WhatsApp protocol, auth, media/audio, account state or app-specific `Action` enums.
- The shell is a **portable adaptation** of the original layout, not a verbatim copy of its App-dependent implementation.
- App data and callbacks remain the host's responsibility.
- Narrow screens switch between sidebar and detail using `ShellState::open_detail` and `close_detail`.
- License and original author credit are preserved.

**Build note:** Compilation has not been executed in this environment. Run `cargo check --all-targets` with a recent Rust toolchain to validate.

## Linux AppImage (CI)

The `Build UI Gallery AppImage` workflow builds a **Linux x86_64** AppImage of `examples/gallery.rs` on pushes to `ui-only` or by manual dispatch.

1. Open [Actions → Build UI Gallery AppImage](https://github.com/Areo-RGB/zapfast/actions/workflows/appimage.yml).
2. Choose **Run workflow** on the `ui-only` branch, or use the run triggered by your latest push.
3. Under the completed run's **Artifacts**, download `zapfast-ui-gallery-appimage-x86_64`.
4. Extract the artifact ZIP, then run `chmod +x ZapFast-UI-Gallery-*.AppImage && ./ZapFast-UI-Gallery-*.AppImage`.

This packages the **UI gallery**, not the original WhatsApp client. It builds on Ubuntu 22.04 and expects ordinary desktop graphics and windowing system libraries on the host; it is not a fully statically linked application.
