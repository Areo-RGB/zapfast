//! Standalone widgets: avatar, badge, menu row, empty state, search, bubbles.
use egui::{Align, Align2, Color32, CornerRadius, Frame, Response, RichText, Sense, Stroke, Ui, Vec2, vec2};
use crate::theme::{Palette, RADIUS, ROW_HEIGHT};

/// Deterministic avatar with initials, without image loading or domain models.
pub fn avatar(ui: &mut Ui, palette: &Palette, name: &str, size: f32) -> Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
    if ui.is_rect_visible(rect) {
        let seed = name.bytes().fold(0u32, |h, b| h.wrapping_mul(31).wrapping_add(b as u32));
        let colors = [
            Color32::from_rgb(76, 132, 171),
            Color32::from_rgb(126, 95, 180),
            Color32::from_rgb(193, 121, 87),
            palette.accent,
        ];
        ui.painter().circle_filled(rect.center(), size / 2.0, colors[(seed as usize) % colors.len()]);
        let initials: String = name.split_whitespace().filter_map(|s| s.chars().next()).take(2).collect::<String>().to_uppercase();
        ui.painter().text(rect.center(), Align2::CENTER_CENTER, initials, egui::FontId::proportional(size * 0.38), Color32::WHITE);
    }
    response
}

/// Compact counter; returns the containing response.
pub fn badge(ui: &mut Ui, palette: &Palette, count: u32) -> Response {
    let text = if count > 99 { "99+".to_owned() } else { count.to_string() };
    let width = (text.len() as f32 * 8.0 + 12.0).max(22.0);
    let (rect, response) = ui.allocate_exact_size(vec2(width, 21.0), Sense::hover());
    ui.painter().rect_filled(rect, 11.0, palette.accent);
    ui.painter().text(rect.center(), Align2::CENTER_CENTER, text, egui::FontId::proportional(12.0), palette.on_accent);
    response
}

/// Stateless reusable navigation/menu item.
pub fn menu_item(ui: &mut Ui, palette: &Palette, title: &str, selected: bool) -> Response {
    let size = vec2(ui.available_width(), ROW_HEIGHT);
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());
    if selected || response.hovered() {
        ui.painter().rect_filled(rect, RADIUS, if selected { palette.surface_hover } else { palette.surface });
    }
    ui.painter().text(rect.left_center() + vec2(14.0, 0.0), Align2::LEFT_CENTER, title, egui::FontId::proportional(14.0), palette.text);
    response
}

/// Minimal empty state, independent of any application model.
pub fn empty_state(ui: &mut Ui, palette: &Palette, title: &str, description: &str) {
    ui.vertical_centered(|ui| {
        ui.add_space(20.0);
        ui.heading(RichText::new(title).color(palette.text));
        ui.label(RichText::new(description).color(palette.secondary));
        ui.add_space(20.0);
    });
}

/// Search field tied only to the caller's String.
pub fn search_field(ui: &mut Ui, query: &mut String) -> Response {
    ui.add(egui::TextEdit::singleline(query).hint_text("Search…").desired_width(f32::INFINITY))
}

/// Container similar to the original menus; UI provided by caller.
pub fn menu_frame<R>(ui: &mut Ui, palette: &Palette, content: impl FnOnce(&mut Ui) -> R) -> R {
    Frame::new()
        .fill(palette.panel)
        .stroke(Stroke::new(1.0, palette.outline))
        .corner_radius(CornerRadius::same(RADIUS))
        .inner_margin(10)
        .show(ui, content).inner
}

/// Display-only message bubble; does not depend on messaging models.
pub fn message_bubble(ui: &mut Ui, palette: &Palette, text: &str, outgoing: bool) {
    ui.with_layout(egui::Layout::left_to_right(Align::Min), |ui| {
        if outgoing { ui.add_space((ui.available_width() * 0.20).min(100.0)); }
        Frame::new().fill(if outgoing {palette.bubble_out} else {palette.bubble_in})
            .corner_radius(CornerRadius::same(RADIUS))
            .inner_margin(12)
            .show(ui, |ui| { ui.label(RichText::new(text).color(palette.text)); });
    });
}
