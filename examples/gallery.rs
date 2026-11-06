use eframe::egui;
use zapfast_ui::{Palette, widgets};

struct Gallery { dark: bool, search: String, selected: usize, clicks: u32 }
impl Default for Gallery {
    fn default() -> Self { Self { dark: true, search: String::new(), selected: 0, clicks: 0 } }
}
impl eframe::App for Gallery {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let p = if self.dark { Palette::dark() } else { Palette::light() };
        p.install(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("ZapFast UI components");
            ui.label("Standalone egui primitives; no messaging backend.");
            ui.checkbox(&mut self.dark, "Dark theme");
            ui.separator();
            ui.heading("Search");
            widgets::search_field(ui, &mut self.search);
            ui.heading("Avatars and badge");
            ui.horizontal(|ui| {
                widgets::avatar(ui, &p, "Ada Lovelace", 44.0);
                widgets::avatar(ui, &p, "Grace Hopper", 44.0);
                widgets::badge(ui, &p, 12);
            });
            ui.heading("Menu");
            for (index, title) in ["Overview", "Components", "Settings"].iter().enumerate() {
                if widgets::menu_item(ui, &p, title, self.selected == index).clicked() {
                    self.selected = index;
                }
            }
            ui.heading("Menu frame");
            widgets::menu_frame(ui, &p, |ui| {
                if ui.button("Click me").clicked() { self.clicks += 1; }
                ui.label(format!("Clicks: {}", self.clicks));
            });
            ui.heading("Messages");
            widgets::message_bubble(ui, &p, "Reusable incoming bubble", false);
            widgets::message_bubble(ui, &p, "Reusable outgoing bubble", true);
            ui.heading("Empty state");
            widgets::empty_state(ui, &p, "Nothing here yet", "Add your own application data.");
        });
    }
}
fn main() -> eframe::Result {
    eframe::run_native("ZapFast UI Gallery", eframe::NativeOptions::default(), Box::new(|_| Ok(Box::new(Gallery::default()))))
}
