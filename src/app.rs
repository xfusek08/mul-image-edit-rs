
use eframe::epaint::Color32;
use eframe::epaint::Rounding;
use eframe::epaint::Shadow;
use eframe::epaint::Vec2;
use eframe::epi;
use eframe::egui;

use crate::data::Track;
use crate::data::TrackAnalyzer;
use crate::data::TrackLoadingError;
use crate::ui::*;
use crate::utils::load_input_file;

#[derive(Default)]
pub struct App {
    analyzer: Option<TrackAnalyzer>,
    track_error: Option<TrackLoadingError>,
    pixels_per_point: Option<f32>, // TODO load this value from config file
}

impl epi::App for App {
    
    fn name(&self) -> &str {
        env!("CARGO_PKG_NAME")
    }
    
    fn setup(&mut self, ctx: &egui::Context, _frame: &epi::Frame, _storage: Option<&dyn epi::Storage>) {
        let mut visuals = egui::Visuals::dark();
        
        // settings common theme
        visuals.widgets.inactive.rounding = Rounding::same(5.0);
        visuals.widgets.active.rounding = Rounding::same(5.0);
        
        ctx.set_visuals(visuals);
    }
    
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        
        #[cfg(debug_assertions)]
        self.debug_before(ctx, frame);
        
        self.layout(ctx, frame);
        
        #[cfg(debug_assertions)]
        self.debug_after(ctx, frame);
        
        let drops = handle_dropped_files(ctx);
        if !drops.is_empty() {
            self.set_track_from_file(drops[0].as_str());
        }
    }
    
}

impl App {
    
    fn layout(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        let quad_height = ctx.available_rect().height() / 4.0;
        
        // top panel -> tack overview
        egui::TopBottomPanel::top("track_panel")
            .frame(egui::Frame::default()
                .margin(egui::style::Margin::same(20.0))
                .fill(Color32::from_gray(27))
                .shadow(Shadow::big_dark())
            )
            .resizable(false)
            .default_height(quad_height)
            .show(ctx, |ui| {
                
                ui.spacing_mut().button_padding = Vec2::new(10.0, 5.0);
                
                match &mut self.analyzer {
                    
                    // Render track placeholder or error when track is not valid and load file if those ui component requests it
                    None => {
                        let action = match &self.track_error {
                            Some(error) => InvalidTrackPanel::ui(ui, error),
                            None => EmptyTrackPanel::ui(ui),
                        };
                        if let TrackPanelResult::OpenFile = action {
                            self.load_file();
                        }
                    },
                    
                    // Render track info from track held by track analyzer
                    Some(analyzer) => match TrackPanel::ui(ui, analyzer.get_track()) {
                        TrackPanelResult::None => (),
                        TrackPanelResult::OpenFile => self.load_file(),
                        TrackPanelResult::Analyze => analyzer.start(),
                    }
                }
            });
            
        
        // central panel -> tack segment list
        egui::CentralPanel::default()
            .frame(egui::Frame::default()
                .margin(egui::style::Margin::same(10.0))
                .fill(Color32::from_gray(15))
            )
            .show(ctx, |ui| {
                if let Some(analyzer) = &mut self.analyzer {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for segment in &mut analyzer.segments {
                            TrackSegmentEditor::Ui(ui, segment);
                            ui.separator();
                        }
                    });
                }
            });
    }
    
    fn set_track_from_file(&mut self, s : &str) {
        
        // TODO: properly handle potentially running already existing tract analyzer -> graceful termination
        self.analyzer = None;
        
        match Track::from_file(s) {
            Ok(track) => self.analyzer = Some(TrackAnalyzer::new(track)),
            Err(error) => self.track_error = Some(error),
        }
    }
    
    fn load_file(&mut self) {
        if let Some(file_name) = load_input_file() {
            self.set_track_from_file(&file_name);
        }
    }
    
    #[cfg(debug_assertions)]
    /// Function rendering backend debug panel only for debug builds
    fn debug_before(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
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
    fn debug_after(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        
    }
}
