use crate::data::AppState;

pub struct DebugBottomPanel;

impl DebugBottomPanel {
    pub fn ui(state: &mut AppState, ctx: &egui::Context, frame: &epi::Frame) {
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .min_height(0.0)
            .show(ctx,|ui| {
                let mut debug_on_hover = ui.ctx().debug_on_hover();
                let pixels_per_point = state.pixels_per_point.get_or_insert_with(|| {
                    frame.info().native_pixels_per_point.unwrap_or_else(|| ui.ctx().pixels_per_point())
                });
        
                ui.horizontal(|ui| {
                    ui.checkbox(&mut debug_on_hover, "🐛 Debug on hover");
                    ui.separator();
                    ui.spacing_mut().slider_width = 90.0;
                    ui.add(
                        egui::Slider::new(pixels_per_point, 0.5..=1.6)
                            .logarithmic(true)
                            .clamp_to_range(true)
                            .text("Scale"),
                    )
                    .on_hover_text("Physical pixels per point.");
                    if let Some(native_pixels_per_point) = frame.info().native_pixels_per_point {
                        let enabled = *pixels_per_point != native_pixels_per_point;
                        if ui
                            .add_enabled(enabled, egui::Button::new("Reset"))
                            .on_hover_text(format!(
                                "Reset scale to native value ({:.1})",
                                native_pixels_per_point
                            ))
                            .clicked()
                        {
                            *pixels_per_point = native_pixels_per_point;
                        }
                    }
                });
                
                if !ui.ctx().is_using_pointer() {
                    ui.ctx().set_pixels_per_point(*pixels_per_point);
                }
                ui.ctx().set_debug_on_hover(debug_on_hover);
            });
    }
}
