
use std::sync::Arc;
use std::time::Instant;

use egui::Vec2;
use egui::style::Margin;
use epi::backend::RepaintSignal;
use indoc::indoc;

use crate::{
    utils::{fit_into, format_size},
    data::{MultimediaFile, Viewport},
    constants::{RIGHT_PANEL_WIDTH, THUMBNAIL_SIZE, MIN_SLIDER_WIDTH}
};

use super::{
    ModifierPipeline,
    Image,
    modifiers::{ExposureModifier, Slider, ContrastModifier, SepiaModifier, BlurModifier}
};

pub struct ImageEditor {
    pipeline: ModifierPipeline,
    last_view_change_time: Option<Instant>,
    media_file: MultimediaFile,
    preview_size: Vec2,
    repaint_signal: Arc<dyn RepaintSignal>,
    texture: Option<egui::TextureHandle>,
    viewport: Viewport,
    right_panel_width: f32,
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
                pipeline.push_modifier(Box::new(ContrastModifier::with_thumbnails(&tm)));
                pipeline.push_modifier(Box::new(BlurModifier::with_thumbnails(&tm)));
                pipeline.push_modifier(Box::new(SepiaModifier::with_thumbnails(&tm)));
                
                Ok(Self {
                    last_view_change_time: None,
                    media_file,
                    preview_size,
                    repaint_signal,
                    texture: None,
                    viewport,
                    right_panel_width: RIGHT_PANEL_WIDTH,
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
        
        // // if size has changed and no resize work is in progress
        // if self.viewport.size != size {
        //     self.viewport.size = size;
        //     self.preview_size = fit_into(
        //         &self.viewport.size,
        //         &self.image.original().size_vec2(),
        //         false
        //     );
            
        //     let diff = self.image.preview_working().size_vec2() - self.preview_size;
        //     if diff.length() > 20.0 {
        //         self.last_view_change_time = Some(Instant::now());
        //         self.repaint_signal.request_repaint();
        //     }
            
        //     return;
        // }
        
        // // if timer is running and it is time to start a resize work -> start resize work
        // if let Some(time) = self.last_view_change_time {
        //     if time.elapsed() >= Duration::from_millis(100) {
                
        //         let repaint_signal = self.repaint_signal.clone();
        //         let started = self.image.resize(
        //             &self.preview_size,
        //             move || repaint_signal.request_repaint()
        //         );
                
        //         // if job has not started then do not stop trying to start it
        //         if started {
        //             self.last_view_change_time = None;
        //             self.repaint_signal.request_repaint();
        //         }
        //     }
        //     return;
        // }
        
        // // if preview image was updated then invalidate texture to be loaded in next pass
        // if self.image.update_check() {
        //     self.texture = None;
        // }
    }
}

// ui code
impl ImageEditor {
    pub fn ui(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        
        // let is_resizing = self.image.is_resizing();
        
        
        // bottom panel with image data
        egui::TopBottomPanel::bottom("info_bar")
            .show(ctx, |ui| {
                let original    = self.pipeline.original_image();
                let current     = self.pipeline.current_image();
                let original_size = original.size_vec2();
                let current_size  = current.size_vec2();
                
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
        
        
        // // Right editor panel
        let min_w = ctx.available_rect().width() * 0.15;
        let max_w = ctx.available_rect().width() * 0.7;
        // let mut right_panel = egui::SidePanel::right("editor_panel");
        // let right_panel_resize_id = egui::Id::new("editor_panel").with("__resize");
        // let right_panel_resizing = ctx.memory().is_being_dragged(right_panel_resize_id);
        // if !right_panel_resizing {
        //     right_panel = right_panel.min_width(self.right_panel_width);
        // } else {
        //     right_panel = right_panel.min_width(min_w).max_width(max_w);
        // }
        // right_panel.show(ctx,  |ui| {
        //     if right_panel_resizing {
        //         self.right_panel_width = ui.available_width().clamp(min_w, max_w);
        //     }
        //     self.pipeline.ui(ui);
        // });
        
        // // right editor panel
        egui::SidePanel::right("editor_panel")
            .min_width(RIGHT_PANEL_WIDTH)
            .max_width(RIGHT_PANEL_WIDTH)
            .default_width(RIGHT_PANEL_WIDTH)
            .resizable(false)
            .show(ctx,  |ui| self.pipeline.ui(ui));
        
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
                                self.pipeline.show_current_image(ui);
                            });
                    });
            });
            
    }
}
