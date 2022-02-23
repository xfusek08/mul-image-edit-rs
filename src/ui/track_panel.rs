
use eframe::egui;
use eframe::egui::Layout;
use eframe::egui::RichText;
use eframe::egui::WidgetText;
use eframe::emath::Align;
use eframe::epaint::FontId;

use crate::data::Track;
use crate::data::TrackLoadingError;

pub struct TrackPanel;
pub struct InvalidTrackPanel;
pub struct EmptyTrackPanel;

pub enum TrackPanelResult {
    None, OpenFile, Analyze
}

impl TrackPanel {
    pub fn ui(ui : &mut egui::Ui, track : &Track)  -> TrackPanelResult {
        let mut res = TrackPanelResult::None;
        
        ui.horizontal(|ui| {
            ui.label(BigLabel(track.file_name()));
            if ui.button(SizedText("Analyze", 20.0)).clicked() {
                res = TrackPanelResult::Analyze;
            }
        });
        
        res
    }
}

impl InvalidTrackPanel {
    pub fn ui(ui : &mut egui::Ui, load_error : &TrackLoadingError) -> TrackPanelResult {
        
        TrackPanelResult::None
    }
}

impl EmptyTrackPanel {
    pub fn ui(ui : &mut egui::Ui) -> TrackPanelResult {
        let mut res = TrackPanelResult::None;
        
        ui.spacing_mut().item_spacing.y = 20.0;
        ui.with_layout(
            Layout::default().with_cross_align(Align::Center),
            |ui| {
                ui.label(BigLabel("Select audio file"));
                if ui.button(SizedText("Open", 20.0)).clicked() {
                    res = TrackPanelResult::OpenFile;
                }
            }
        );
        
        res
    }
}


fn BigLabel(text: &str) -> impl Into<WidgetText> {
    SizedText(text, 30.0)
}

fn SizedText(text: &str, size: f32) -> impl Into<WidgetText> {
    RichText::new(text).font(FontId::proportional(size))
}
