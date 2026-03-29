use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 200.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Microstructure Variable Selector",
        options,
        Box::new(|cc| {
            // Set black and white theme
            let mut visuals = egui::Visuals::dark();

            // strictly black and white
            visuals.window_fill = egui::Color32::BLACK;
            visuals.panel_fill = egui::Color32::BLACK;

            visuals.widgets.noninteractive.bg_fill = egui::Color32::BLACK;
            visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);

            visuals.widgets.inactive.bg_fill = egui::Color32::BLACK;
            visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);
            visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);

            visuals.widgets.hovered.bg_fill = egui::Color32::BLACK;
            visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);
            visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);

            visuals.widgets.active.bg_fill = egui::Color32::WHITE;
            visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, egui::Color32::BLACK);
            visuals.widgets.active.bg_stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);

            visuals.selection.bg_fill = egui::Color32::WHITE;
            visuals.selection.stroke = egui::Stroke::new(1.0, egui::Color32::BLACK);

            cc.egui_ctx.set_visuals(visuals);

            Ok(Box::new(MyApp::default()))
        }),
    )
}

#[derive(Default)]
struct MyApp {
    user_input: String,
    submitted_input: Option<String>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        #[allow(deprecated)]
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui(ui, frame);
        });
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("Microstructure Variable Selector");

            ui.add_space(10.0);

            ui.label("Choose a microstructure variable amongst the following choices: 1) Trade size (Enter '1')");

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Your choice:");
                let response = ui.text_edit_singleline(&mut self.user_input);
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.submitted_input = Some(self.user_input.clone());
                }
                if ui.button("Submit").clicked() {
                    self.submitted_input = Some(self.user_input.clone());
                }
            });

            ui.add_space(10.0);

            if let Some(input) = &self.submitted_input {
                if input.trim() == "1" {
                    ui.label("You've selected trade size for your microstructure variable");
                } else {
                    ui.label(format!("You've selected '{}'", input));
                }
            }
    }
}
