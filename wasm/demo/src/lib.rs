use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}! This is from Rust!", name));
}

use eframe::egui;
use wasm_bindgen::JsCast;
#[derive(Default)]
pub struct MyApp {
    name: String,
    age: u32,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            name: "Zola User".to_owned(),
            age: 18,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Egui WebAssembly");

            ui.horizontal(|ui| {
                ui.label("name: ");
                ui.text_edit_singleline(&mut self.name);
            });

            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));

            if ui.button("incress").clicked() {
                self.age += 1;
            }

            ui.separator();
            ui.label(format!("{} now {}。", self.name, self.age));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.label("Powered by Rust + Egui + WebAssembly");
            });
        });
    }
}

#[wasm_bindgen]
pub async fn start(canvas_id: &str) -> Result<(), wasm_bindgen::JsValue> {
    let window = web_sys::window().ok_or("No global `window` exists")?;
    let document = window
        .document()
        .ok_or("Should have a document on window")?;

    let canvas = document
        .get_element_by_id(canvas_id)
        .ok_or_else(|| format!("Failed to find canvas with id '{}'", canvas_id))?;

    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| "Element is not a canvas")?;

    let web_options = eframe::WebOptions::default();

    eframe::WebRunner::new()
        .start(
            canvas,
            web_options,
            Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
        )
        .await
        .map_err(|e| wasm_bindgen::JsValue::from_str(&format!("Failed to start eframe: {e:?}")))
}
