
mod app;
mod data;
mod constants;
mod components;
mod widgets;
mod utils;

fn main() {
    let options = epi::NativeOptions {
        drag_and_drop_support: true,
        ..Default::default()
    };
    app::run("Image - Editor", &options);
}
