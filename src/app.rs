
use eframe::epi;
use eframe::egui;
use crate::ui::*;

#[derive(Default)]
pub struct App {
    loaded_files : Vec<String>,
    pixels_per_point: Option<f32>, // TODO load this value from config file
}

impl epi::App for App {
    
    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &epi::Frame) {
        
        #[cfg(debug_assertions)]
        self.debug_before(ctx, frame);
        
        egui::CentralPanel::default().show(ctx, |ui| {
            for file_name in &mut self.loaded_files {
                ui.add(egui::TextEdit::singleline(file_name)
                    .hint_text("Write something here")
                    .desired_width(f32::INFINITY)
                );
            }
        });
        
        #[cfg(debug_assertions)]
        self.debug_after(ctx, frame);
        
        let mut drops = handle_dropped_files(ctx);
        if !drops.is_empty() {
            self.loaded_files.append(&mut drops);
        }
    }
    
    fn name(&self) -> &str {
        env!("CARGO_PKG_NAME")
    }
}

impl App {
    
    #[cfg(debug_assertions)]
    /// Function rendering backend debug panel only for debug builds
    fn debug_before(&mut self, ctx: &eframe::egui::CtxRef, frame: &epi::Frame) {
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
                });
                
                if !ui.ctx().is_using_pointer() {
                    ui.ctx().set_pixels_per_point(*pixels_per_point);
                }
                ui.ctx().set_debug_on_hover(debug_on_hover);
            });
    }
    
    #[cfg(debug_assertions)]
    /// Function rendering backend debug panel only for debug builds
    fn debug_after(&mut self, ctx: &eframe::egui::CtxRef, frame: &epi::Frame) {
        
    }
}
