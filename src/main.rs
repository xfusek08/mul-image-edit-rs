use egui::vec2;

mod app;
mod data;
mod constants;
mod components;
mod widgets;
mod utils;

fn main() {
    let options = epi::NativeOptions {
        initial_window_size: Some(vec2(1200.0, 720.0)),
        drag_and_drop_support: true,
        ..Default::default()
    };
    app::run("Image - Editor", &options);
}
