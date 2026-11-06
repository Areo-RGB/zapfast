//! Backend-free adaptation of ZapFast's rail + list + detail window composition.
//! Inspired by src/ui/{mod,rail,chats,pane}.rs on the original main branch.
use egui::{Align, Align2, CentralPanel, Context, Frame, Id, Layout, Panel, RichText, Sense, Ui, UiBuilder, vec2};
use crate::theme::{Palette, TOP_BAR_HEIGHT};

pub const RAIL_WIDTH: f32 = 52.0;
pub const DEFAULT_SIDEBAR_WIDTH: f32 = 300.0;
pub const MIN_SIDEBAR_WIDTH: f32 = 190.0;
pub const NARROW_BREAKPOINT: f32 = 640.0;

/// A caller-defined navigation destination. IDs are app-specific but UI has no knowledge of them.
#[derive(Clone, Debug)]
pub struct Destination {
    pub id: String,
    pub label: String,
    /// Short symbol drawn in the small rail cell; avoid depending on an icon pack.
    pub glyph: String,
}
impl Destination {
    pub fn new(id: impl Into<String>, label: impl Into<String>, glyph: impl Into<String>) -> Self {
        Self { id: id.into(), label: label.into(), glyph: glyph.into() }
    }
}

/// The shell owns only presentation state; the host owns all actual application data.
#[derive(Clone, Debug)]
pub struct ShellState {
    pub selected: String,
    pub sidebar_width: f32,
    pub sidebar_visible: bool,
    /// On narrow windows, true opens the detail view instead of the list.
    pub detail_open: bool,
    pub inspector_open: bool,
}
impl Default for ShellState {
    fn default() -> Self {
        Self { selected: String::new(), sidebar_width: DEFAULT_SIDEBAR_WIDTH,
            sidebar_visible: true, detail_open: false, inspector_open: false }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ShellConfig {
    pub narrow_breakpoint: f32,
    pub rail_width: f32,
    pub min_sidebar_width: f32,
    pub inspector_width: f32,
}
impl Default for ShellConfig {
    fn default() -> Self {
        Self { narrow_breakpoint: NARROW_BREAKPOINT, rail_width: RAIL_WIDTH,
            min_sidebar_width: MIN_SIDEBAR_WIDTH, inspector_width: 300.0 }
    }
}

/// A result describing layout decisions, useful to a host and its tests.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ShellLayout {
    pub narrow: bool,
    pub sidebar_drawn: bool,
    pub inspector_drawn: bool,
}

/// Composable application shell. Each closure paints arbitrary caller-supplied content.
/// Navigation and shell state are UI-only; callbacks/closures do not receive any messaging types.
/// On small windows, call `state.open_detail()` after selecting a list item.
pub fn show(
    ctx: &Context,
    palette: &Palette,
    state: &mut ShellState,
    destinations: &[Destination],
    config: ShellConfig,
    header: impl FnOnce(&mut Ui),
    sidebar: impl FnOnce(&mut Ui, &mut ShellState),
    content: impl FnOnce(&mut Ui, &mut ShellState),
    inspector: impl FnOnce(&mut Ui),
) -> ShellLayout {
    let narrow = ctx.content_rect().width() < config.narrow_breakpoint;
    let show_list = state.sidebar_visible && (!narrow || !state.detail_open);
    let show_inspector = state.inspector_open && !narrow
        && ctx.content_rect().width() >= config.narrow_breakpoint + config.inspector_width;
    // egui 0.36 panels draw into a root Ui rather than directly into Context.
    let mut root = Ui::new(ctx.clone(), Id::new("zapfast-ui-root"), UiBuilder::new().max_rect(ctx.content_rect()));
    Panel::top("zapfast-ui-shell-header")
        .exact_size(TOP_BAR_HEIGHT)
        .frame(Frame::new().fill(palette.panel).inner_margin(egui::Margin::symmetric(12, 7)))
        .show(&mut root, |ui| {
            ui.horizontal_centered(|ui| {
                if narrow && state.detail_open && ui.button("‹ Back").clicked() {
                    state.detail_open = false;
                }
                header(ui);
            });
        });
    Panel::left("zapfast-ui-shell-rail")
        .exact_size(config.rail_width)
        .resizable(false)
        .frame(Frame::new().fill(palette.panel))
        .show(&mut root, |ui| {
            rail(ui, palette, state, destinations);
        });
    // Unlike the original App-dependent list pane, this surface accepts any egui content.
    if show_list {
        Panel::left("zapfast-ui-shell-sidebar")
            .default_width(state.sidebar_width.max(config.min_sidebar_width))
            .min_width(config.min_sidebar_width)
            .resizable(!narrow)
            .frame(Frame::new().fill(palette.panel).inner_margin(egui::Margin::same(10)))
            .show(&mut root, |ui| {
                state.sidebar_width = ui.max_rect().width();
                sidebar(ui, state);
            });
    }
    if show_inspector {
        Panel::right("zapfast-ui-shell-inspector")
            .default_width(config.inspector_width)
            .resizable(true)
            .frame(Frame::new().fill(palette.panel).inner_margin(egui::Margin::same(12)))
            .show(&mut root, inspector);
    }
    CentralPanel::default()
        .frame(Frame::new().fill(palette.window).inner_margin(egui::Margin::same(14)))
        .show(&mut root, |ui| {
            if narrow && !state.detail_open && show_list {
                ui.vertical_centered(|ui| {
                    ui.add_space(35.0);
                    ui.label(RichText::new("Select an item").color(palette.secondary));
                });
            } else {
                content(ui, state);
            }
        });
    ShellLayout { narrow, sidebar_drawn: show_list, inspector_drawn: show_inspector }
}

impl ShellState {
    pub fn open_detail(&mut self) { self.detail_open = true; }
    pub fn close_detail(&mut self) { self.detail_open = false; }
}

/// Public rail widget, useful even without the full app shell.
pub fn rail(ui: &mut Ui, palette: &Palette, state: &mut ShellState, destinations: &[Destination]) {
    ui.add_space(8.0);
    for destination in destinations {
        let size = vec2(ui.available_width(), 48.0);
        let (rect, response) = ui.allocate_exact_size(size, Sense::click());
        let selected = state.selected == destination.id;
        if selected || response.hovered() {
            let plate = egui::Rect::from_center_size(rect.center(), vec2(36.0, 36.0));
            ui.painter().rect_filled(plate, 10.0, if selected { palette.surface_hover } else { palette.surface });
        }
        ui.painter().text(rect.center(), Align2::CENTER_CENTER, &destination.glyph,
            egui::FontId::proportional(19.0), if selected { palette.accent } else { palette.text });
        if response.on_hover_text(&destination.label).clicked() {
            state.selected = destination.id.clone();
            state.detail_open = false;
        }
    }
}

/// Reusable header/title row intended for the caller's `header` callback.
pub fn header_title(ui: &mut Ui, title: &str, subtitle: Option<&str>) {
    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
        ui.heading(title);
        if let Some(subtitle) = subtitle {
            ui.separator();
            ui.label(RichText::new(subtitle).weak().size(12.0));
        }
    });
}
