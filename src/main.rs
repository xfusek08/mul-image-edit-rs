
mod app;

fn main() {
    let app = app::App::default();
    let options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), options);
}
