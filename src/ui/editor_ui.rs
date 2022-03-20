
use egui::{style::Margin, vec2};

use crate::{data::{ImageEditorState, EditedImageComponent}, utils::format_size};

pub struct EditorUi;

impl EditorUi {
    pub fn ui(state: &mut ImageEditorState, ctx: &egui::Context) {
        
        // bottom panel with image data
        egui::TopBottomPanel::bottom("info_bar")
            .show(ctx, |ui| {
                let original_size = state.image().original().size_vec2();
                let preview_size = state.image().preview_working().size_vec2();
                
                ui.horizontal_wrapped(|ui| {
                    ui.label(state.media_file().file_name());
                    ui.separator();
                    ui.label( format!("Original size: {} x {}", original_size.x, original_size.y));
                    ui.separator();
                    ui.label( format!("Preview size: {} x {}", preview_size.x, preview_size.y));
                    ui.separator();
                    ui.label( format!("File size: {}", format_size(state.media_file().size())));
                    ui.separator();
                    ui.label( format!("Raw size: {}", format_size(state.image().original().raw_size())));
                    ui.separator();
                    ui.label( format!("Raw preview size: {}", format_size(state.image().preview_working().raw_size())));
                    ui.separator();
                });
            });
        
        // image viewport
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                state.size_viewport(ui.available_size() * 0.98);
                
                let preview_size = *state.preview_size();
                let offset = 0.5 * (ui.available_size() - preview_size);
                
                egui::Frame::default()
                    .margin(Margin {
                        left: offset.x,
                        top: offset.y,
                        ..Default::default()
                    })
                    .show(ui, |ui| {
                        egui::Frame::none()
                            .shadow(ctx.style().visuals.popup_shadow)
                            .show(ui, |ui| {
                                ui.image(state.texture(ctx).id(), preview_size);
                            });
                    });
            });
            
        
    }
}