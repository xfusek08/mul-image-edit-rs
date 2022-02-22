
use eframe::epi;
use eframe::egui;
use crate::data::Track;
use crate::data::TrackAnalyzer;
use crate::ui::*;

#[derive(Default)]
pub struct App {
    analyzer : Option<TrackAnalyzer>,
    pixels_per_point: Option<f32>, // TODO load this value from config file
}

impl epi::App for App {
    
    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &epi::Frame) {
        
        #[cfg(debug_assertions)]
        self.debug_before(ctx, frame);
        
        egui::CentralPanel::default().show(ctx, |ui| self.ui(ui));
        
        #[cfg(debug_assertions)]
        self.debug_after(ctx, frame);
        
        let drops = handle_dropped_files(ctx);
        if !drops.is_empty() {
            self.setTrackFromFile(drops[0].as_str());
        }
    }
    
    fn name(&self) -> &str {
        env!("CARGO_PKG_NAME")
    }
}

impl App {
    
    // rendering of current state with possible calling of setters when input changes
    fn ui(&mut self, ui : &mut egui::Ui) {
        if let Some(analyzer) = &self.analyzer {
            match &analyzer.track {
                Track::Invalid {path, message} => {
                    ui.label(format!("Track {} is invalid: {}", path.display(), message));
                },
                Track::Valid { path, format, file } => {
                    ui.label(format!("Track {} is valid of type: {}", path.display(), format));
                },
            }
        }
    }
    
    fn setTrackFromFile(&mut self, s : &str) {
        // TODO: properly handle potentially running already existing tract analyzer -> graceful termination
        
        self.analyzer = Some(TrackAnalyzer::new(Track::from_file(s)));
        
        // TODO: initialize analyzing the track
    }
    
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
