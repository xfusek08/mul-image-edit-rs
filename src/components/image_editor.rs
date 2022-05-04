
use std::sync::Arc;

use egui::{style::Margin, Vec2};
use epi::backend::RepaintSignal;
use indoc::indoc;

use crate::{
    widgets::{texts, BigButton},
    utils::{save_output_file, fit_into, format_size},
    data::{MultimediaFile, Viewport},
    constants::{RIGHT_PANEL_WIDTH, THUMBNAIL_SIZE}
};

use super::{
    ModifierPipeline,
    Image,
    modifiers::{
        Slider,
        specific::{
            ExposureModifier,
            ContrastModifier,
            BlurModifier,
            TintModifier,
            CustomModifier,
            BMulModifier,
            BMulLumaModifier,
            GammaModifier
        }
    },
};

pub enum EditorResult {
    Nothing,
    LoadNewImage,
}

pub struct ImageEditor {
    pipeline: ModifierPipeline,
    media_file: MultimediaFile,
    viewport: Viewport,
}

// constructors
impl ImageEditor {
    
    pub fn from_file(mut media_file: MultimediaFile, repaint_signal: Arc<dyn RepaintSignal>) -> Result<Self, String> {
        match Image::from_bytes(media_file.bytes().as_slice()) {
            Ok(original_image) => {
                let viewport = Viewport::new().sized(original_image.size_vec2());
                let preview_size = viewport.size;
                let mut pipeline = ModifierPipeline::new(original_image, preview_size);
                
                let tm = pipeline.original_image().thumbnail(THUMBNAIL_SIZE as u32, THUMBNAIL_SIZE as u32);
                pipeline.push_modifier(Box::new(ExposureModifier::with_thumbnails(&tm)));
                pipeline.push_modifier(Box::new(GammaModifier::with_thumbnails(&tm)));
                pipeline.push_modifier(Box::new(BMulLumaModifier::with_thumbnails(&tm)));
                pipeline.push_modifier(Box::new(BMulModifier::with_thumbnails(&tm)));
                pipeline.push_modifier(Box::new(ContrastModifier::with_thumbnails(&tm)));
                pipeline.push_modifier(Box::new(BlurModifier::with_thumbnails(&tm)));
                pipeline.push_modifier(Box::new(TintModifier::with_thumbnails(&tm)));
                pipeline.push_modifier(Box::new(CustomModifier::with_thumbnails(&tm)));
                
                Ok(Self {
                    media_file,
                    viewport,
                    pipeline,
                })
            },
            
            Err(message) => Err(format!(indoc!("
                Image editor initiation error:
                    Media File:
                        {}
                    Message: \"{}\"
            "), media_file, message)),
        }
    }
    
}


// mutating methods
impl ImageEditor {
    
    pub fn size_viewport(&mut self, size: Vec2) {
        
        if *self.pipeline.preview_size() != size {
            self.viewport.size = size;
            self.pipeline.resize(fit_into(
                &self.viewport.size,
                &self.pipeline.original_image().size_vec2(),
                false
            ));
        }
    }
    
    fn save_image_to_file(&self) {
        if let Some(p) = save_output_file(Some(&self.media_file)) {
            self.pipeline.apply_to_original().raw_image.save(p);
        }
    }
}

// ui code
impl ImageEditor {
    pub fn ui(&mut self, ctx: &egui::Context, frame: &epi::Frame) -> EditorResult {
        
        let mut result = EditorResult::Nothing;
        
        // bottom panel with image data
        egui::TopBottomPanel::bottom("info_bar")
            .show(ctx, |ui| {
                let original = self.pipeline.original_image();
                let current = self.pipeline.current_image();
                let original_size = original.size_vec2();
                let current_size = current.size_vec2();
                
                // let sw = 20.0;
                // let w = ui.available_width() - sw;
                
                ui.horizontal(|ui| {
                    ui.horizontal_wrapped(|ui| {
                        // if is_resizing {
                        //     ui.set_max_width(w);
                        // }
                        ui.label( format!(
                            "Image file: {} - {}",
                            self.media_file.file_name(),
                            format_size(self.media_file.size())
                        ));
                        ui.separator();
                        ui.label( format!(
                            "Original size: {} x {} - {}",
                            original_size.x,
                            original_size.y,
                            format_size(original.raw_size())
                        ));
                        ui.separator();
                        ui.label( format!(
                            "Current size: {} x {} - {}",
                            current_size.x,
                            current_size.y,
                            format_size(current.raw_size())
                        ));
                    });
                    // if is_resizing  {
                    //     ui.with_layout(egui::Layout::right_to_left().with_cross_justify(true), |ui| {
                    //         ui.set_min_width(sw);
                    //         // ui.horizontal(|ui| {
                    //             ui.add(egui::Spinner::new());
                    //             frame.request_repaint();
                    //         // });
                    //     });
                    // }
                });
        });
        
        // right editor panel
        egui::SidePanel::right("editor_panel")
            .min_width(RIGHT_PANEL_WIDTH)
            .default_width(RIGHT_PANEL_WIDTH)
            .show(ctx,  |ui| {
                if BigButton::ui(ui, "📂  Open file").clicked() {
                    result = EditorResult::LoadNewImage;
                }
                
                ui.label(texts::sized("Filters: ", 20.0));
                
                let h = ui.available_height() - 55.0;
                egui::ScrollArea::vertical()
                    .max_height(h)
                    .show(ui, |ui| {
                        // ui.set_height(h);
                        self.pipeline.ui(ui);
                    });
                
                if BigButton::ui(ui, "💾  Save").clicked() {
                   self.save_image_to_file();
                }
        });
        
        // image viewport
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                self.size_viewport(ui.available_size() * 0.98);
                
                let offset = 0.5 * (ui.available_size() - *self.pipeline.preview_size());
                
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
                            if ui.ctx().input().key_down(egui::Key::Space) {
                                self.pipeline.show_original_image(ui);
                            } else {
                                self.pipeline.show_current_image(ui);
                            }
                        });
                    });
        
        });
        
        result
    }
}
