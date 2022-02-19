
use eframe::epi;
use eframe::egui;

pub struct App;

impl Default for App {
    fn default() -> Self {
        Self {  }
    }
}

impl epi::App for App {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &epi::Frame) {
        egui::TopBottomPanel::top("my_panel").show(ctx, |ui| {
            ui.label("Top panel");
        });
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Central panel");
         });
    }

    fn name(&self) -> &str {
        env!("CARGO_PKG_NAME")
    }
}
