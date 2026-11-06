use std::cell::Cell;
use eframe::egui;
use zapfast_ui::{Palette, shell::{self, Destination, ShellConfig, ShellState}, widgets};

struct Gallery {
    dark: bool, query: String, shell: ShellState, selected_item: usize, clicks: u32,
}
impl Default for Gallery {
    fn default() -> Self {
        Self { dark: true, query: String::new(),
            shell: ShellState { selected: "inbox".into(), ..Default::default() },
            selected_item: 0, clicks: 0 }
    }
}
impl eframe::App for Gallery {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let palette = if self.dark { Palette::dark() } else { Palette::light() };
        palette.install(ctx);
        let destinations = [
            Destination::new("inbox", "Inbox", "✉"),
            Destination::new("work", "Work", "▦"),
            Destination::new("starred", "Starred", "★"),
            Destination::new("settings", "Settings", "⚙"),
        ];
        // Cell lets independent sidebar/detail callbacks share selection safely.
        let selected = Cell::new(self.selected_item);
        let query = &mut self.query;
        let clicks = &mut self.clicks;
        let dark = &mut self.dark;
        shell::show(
            ctx, &palette, &mut self.shell, &destinations, ShellConfig::default(),
            |ui| shell::header_title(ui, "Component Gallery", Some("Reusable native egui shell")),
            |ui, state| {
                ui.heading("Navigation");
                widgets::search_field(ui, query);
                ui.add_space(8.0);
                for (i, item) in ["Overview", "Buttons & badges", "Conversation", "Empty state"].iter().enumerate() {
                    if widgets::menu_item(ui, &palette, item, selected.get() == i).clicked() {
                        selected.set(i);
                        state.open_detail();
                    }
                }
                ui.label("Resize the window to test narrow mode.");
            },
            |ui, state| {
                ui.horizontal(|ui| {
                    ui.heading(["Overview", "Buttons & badges", "Conversation", "Empty state"][selected.get()]);
                    ui.checkbox(dark, "Dark");
                    ui.checkbox(&mut state.inspector_open, "Inspector");
                });
                ui.separator();
                match selected.get() {
                    0 => {
                        ui.label("Rail, resizable sidebar, header, detail, and optional inspector.");
                        ui.label("Navigation and content are data-independent.");
                        widgets::empty_state(ui, &palette, "Ready for your app", "Replace these callbacks with your own views.");
                    }
                    1 => {
                        ui.horizontal(|ui| {
                            widgets::avatar(ui, &palette, "Ada Lovelace", 48.0);
                            widgets::avatar(ui, &palette, "Grace Hopper", 48.0);
                            widgets::badge(ui, &palette, 12);
                        });
                        widgets::menu_frame(ui, &palette, |ui| {
                            if ui.button("Click me").clicked() { *clicks += 1; }
                            ui.label(format!("Clicked {} times", *clicks));
                        });
                    }
                    2 => {
                        widgets::message_bubble(ui, &palette, "This layout no longer needs WhatsApp.", false);
                        widgets::message_bubble(ui, &palette, "Reuse it with your own model.", true);
                    }
                    _ => widgets::empty_state(ui, &palette, "Nothing to show", "Add content from your application."),
                }
            },
            |ui| {
                ui.heading("Inspector");
                ui.separator();
                ui.label("Optional application-defined panel.");
                ui.label("No backend required.");
            },
        );
        self.selected_item = selected.get();
    }
}
fn main() -> eframe::Result {
    eframe::run_native("ZapFast UI Shell Gallery", eframe::NativeOptions::default(), Box::new(|_| Ok(Box::new(Gallery::default()))))
}
