
pub struct CenteredWindow;
impl CenteredWindow {
    pub fn show<R>(ctx: &egui::Context, add_ui: impl FnOnce(&mut egui::Ui) -> R) {
        let wh = ctx.available_rect().height();
        let h = 50.0;

        egui::CentralPanel::default()
            .frame(egui::Frame::none()
                .margin(egui::style::Margin::symmetric(10.0, wh * 0.5 - h))
            )
            .show(ctx, |ui| {
                egui::Window::new("asd")
                    .title_bar(false)
                    .collapsible(false)
                    .resizable(false)
                    .fixed_pos(ui.max_rect().min)
                    .fixed_size(ui.min_rect().size() - ui.max_rect().min.to_vec2())
                    .frame(egui::Frame::popup(&ctx.style()))
                    .show(ui.ctx(), |ui| {
                        egui::TopBottomPanel::top("top_panel")
                            .frame(egui::Frame::default()
                                .margin(egui::style::Margin::same(20.0))
                            )
                            .resizable(false)
                            .show_inside(ui, |ui| {
                                ui.vertical_centered(add_ui);
                            });
                    });
            });
    }
}
