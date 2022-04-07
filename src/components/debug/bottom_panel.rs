
//! This file is inspired by:
//!    https://github.com/emilk/egui/blob/master/egui_demo_lib/src/backend_panel.rs

use crate::data::Tick;

use super::FrameHistory;

#[derive(Default)]
pub struct BottomPanel {
    pub pixels_per_point: Option<f32>, // TODO load this value from config file
    pub frame_history: FrameHistory,
    pub ticks: u64,
    pub average_delta_between_tics_sec: f32
}

// ui code
impl BottomPanel {
    
    pub fn tick(&mut self, tick: &Tick) {
        self.ticks = self.ticks + 1;
        let delta = tick.delta.as_secs_f32();
        
        // see: https://math.stackexchange.com/a/106720
        self.average_delta_between_tics_sec = self.average_delta_between_tics_sec + (delta - self.average_delta_between_tics_sec) / (self.ticks as f32);
    }
    
    pub fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        self.frame_history.on_new_frame(
            ctx.input().time,
            frame.info().cpu_usage
        );
    }
    
    pub fn ui(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .min_height(0.0)
            .show(ctx,|ui| {
                let mut debug_on_hover = ui.ctx().debug_on_hover();
                let pixels_per_point = self.pixels_per_point.get_or_insert_with(|| {
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
                    
                    self.frame_history.ui(ui);
                    ui.separator();
                    ui.label(format!("Ticks: {} | {} ms average duration", self.ticks, self.average_delta_between_tics_sec));
                });
                
                if !ui.ctx().is_using_pointer() {
                    ui.ctx().set_pixels_per_point(*pixels_per_point);
                }
                ui.ctx().set_debug_on_hover(debug_on_hover);
            });
    }
}
    