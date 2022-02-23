
use eframe::{egui::{self, Style}, epaint::{Stroke, Color32}};

use crate::data::TrackSegment;

pub struct TrackSegmentEditor;

impl TrackSegmentEditor {
    
    pub fn Ui(ui: &mut egui::Ui, segment: &mut TrackSegment) {
        let orig_vis = ui.visuals_mut().clone();
        
        ui.visuals_mut().widgets.noninteractive.bg_stroke = Stroke {
            color: Color32::TRANSPARENT,
            ..Stroke::default()
        };
        
        ui.group(|ui| {
            ui.set_width(ui.available_width());
            // ui.set_style(Style::default());
            
            ui.horizontal(|ui| {
                ui.label(segment.title.as_str());
            });
        });
        
        ui.style_mut().visuals = orig_vis;
    }
    
}
