
mod data;
mod app;
mod ui;

fn main() {
    let app = app::App::default();
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        ..Default::default()
    };
    eframe::run_native(Box::new(app), options);
}
