use eframe::egui;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 200.0])
            .with_transparent(false),
        ..Default::default()
    };
    eframe::run_native(
        "Microstructure Variable Selector",
        options,
        Box::new(|cc| Ok(setup_app(cc))),
    )
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn start(canvas_id: &str) -> Result<(), wasm_bindgen::JsValue> {
    let web_options = eframe::WebOptions::default();

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let canvas = document.get_element_by_id(canvas_id).expect("canvas id not found");
    let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>().expect("canvas is not a HtmlCanvasElement");

    eframe::WebRunner::new()
        .start(
            canvas,
            web_options,
            Box::new(|cc| Ok(setup_app(cc))),
        )
        .await
}

#[cfg(target_arch = "wasm32")]
fn main() {}

fn setup_app(cc: &eframe::CreationContext<'_>) -> Box<dyn eframe::App> {
    // Set black and white theme
    let mut visuals = egui::Visuals::dark();

    visuals.override_text_color = Some(egui::Color32::WHITE);

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

    Box::new(MyApp::default())
}

#[derive(Default)]
struct MyApp {
    user_input: String,
    submitted_input: Option<String>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        #[allow(deprecated)]
        egui::CentralPanel::default()
            .frame(egui::Frame::central_panel(&ctx.style()).fill(egui::Color32::BLACK))
            .show(ctx, |ui| {
                self.ui(ui, frame);
            });
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 1.0]
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
