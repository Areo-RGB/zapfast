//! Portable colors and spacing extracted/adapted from ZapFast's visual language.
use egui::{Color32, Context, CornerRadius, Stroke, Style, Visuals};

pub const RADIUS: u8 = 10;
pub const ROW_HEIGHT: f32 = 44.0;
pub const TOP_BAR_HEIGHT: f32 = 52.0;
pub const SPACING: f32 = 8.0;

#[derive(Clone, Copy, Debug)]
pub struct Palette {
    pub dark: bool,
    pub window: Color32,
    pub panel: Color32,
    pub surface: Color32,
    pub surface_hover: Color32,
    pub outline: Color32,
    pub text: Color32,
    pub secondary: Color32,
    pub accent: Color32,
    pub on_accent: Color32,
    pub danger: Color32,
    pub warning: Color32,
    pub bubble_in: Color32,
    pub bubble_out: Color32,
}
impl Palette {
    pub fn dark() -> Self {
        Self {
            dark: true, window: Color32::from_rgb(11, 20, 26),
            panel: Color32::from_rgb(17, 27, 33),
            surface: Color32::from_rgb(32, 44, 51),
            surface_hover: Color32::from_rgb(42, 57, 66),
            outline: Color32::from_rgb(34, 45, 52),
            text: Color32::from_rgb(233, 237, 239),
            secondary: Color32::from_rgb(151, 165, 174),
            accent: Color32::from_rgb(0, 168, 132),
            on_accent: Color32::from_rgb(11, 20, 26),
            danger: Color32::from_rgb(241, 92, 109),
            warning: Color32::from_rgb(255, 210, 121),
            bubble_in: Color32::from_rgb(32, 44, 51),
            bubble_out: Color32::from_rgb(0, 92, 75),
        }
    }
    pub fn light() -> Self {
        Self {
            dark: false, window: Color32::from_rgb(240, 242, 245),
            panel: Color32::WHITE,
            surface: Color32::from_rgb(240, 242, 245),
            surface_hover: Color32::from_rgb(230, 233, 236),
            outline: Color32::from_rgb(233, 237, 239),
            text: Color32::from_rgb(17, 27, 33),
            secondary: Color32::from_rgb(81, 95, 103),
            accent: Color32::from_rgb(0, 128, 105),
            on_accent: Color32::WHITE,
            danger: Color32::from_rgb(234, 0, 56),
            warning: Color32::from_rgb(160, 107, 0),
            bubble_in: Color32::WHITE,
            bubble_out: Color32::from_rgb(217, 253, 211),
        }
    }
    pub fn install(self, ctx: &Context) {
        let mut style: Style = (*ctx.global_style()).clone();
        let mut visuals = if self.dark { Visuals::dark() } else { Visuals::light() };
        visuals.panel_fill = self.panel;
        visuals.window_fill = self.window;
        visuals.extreme_bg_color = self.surface;
        visuals.override_text_color = Some(self.text);
        visuals.selection.bg_fill = self.accent;
        visuals.widgets.inactive.bg_fill = self.surface;
        visuals.widgets.hovered.bg_fill = self.surface_hover;
        visuals.widgets.active.bg_fill = self.surface_hover;
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, self.outline);
        visuals.widgets.inactive.corner_radius = CornerRadius::same(RADIUS);
        visuals.widgets.hovered.corner_radius = CornerRadius::same(RADIUS);
        visuals.widgets.active.corner_radius = CornerRadius::same(RADIUS);
        style.visuals = visuals;
        style.spacing.item_spacing = egui::vec2(SPACING, SPACING);
        ctx.set_theme(if self.dark { egui::Theme::Dark } else { egui::Theme::Light });
        ctx.set_global_style(style);
    }
}
