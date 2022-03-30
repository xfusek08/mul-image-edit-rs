
use std::sync::Arc;
use std::time::{Instant, Duration};

use egui::Vec2;
use egui::emath::Numeric;
use egui::style::Margin;
use epi::backend::RepaintSignal;
use indoc::indoc;

use crate::utils::{fit_into, format_size};
use crate::data::{MultimediaFile, EditedImage, Viewport, EditedImageComponent, ImageSettings};
use crate::widgets::EditorSlider;

pub struct ImageEditor {
    last_view_change_time: Option<Instant>,
    media_file: MultimediaFile,
    image: EditedImage,
    preview_size: Vec2,
    repaint_signal: Arc<dyn RepaintSignal>,
    texture: Option<egui::TextureHandle>,
    viewport: Viewport,
}

// constructors
impl ImageEditor {
    
    pub fn from_file(mut media_file: MultimediaFile, repaint_signal: Arc<dyn RepaintSignal>) -> Result<Self, String> {
        match EditedImage::from_bytes(media_file.bytes().as_slice()) {
            Ok(image) => {
                let viewport = Viewport::new().sized(image.original().size_vec2());
                let preview_size = viewport.size;
                
                Ok(Self {
                    image: image,
                    last_view_change_time: None,
                    media_file,
                    preview_size,
                    repaint_signal,
                    texture: None,
                    viewport,
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
        
        // if size has changed and no resize work is in progress
        if self.viewport.size != size {
            self.viewport.size = size;
            self.preview_size = fit_into(
                &self.viewport.size,
                &self.image.original().size_vec2(),
                false
            );
            
            let diff = self.image.preview_working().size_vec2() - self.preview_size;
            if diff.length() > 20.0 {
                self.last_view_change_time = Some(Instant::now());
                self.repaint_signal.request_repaint();
            }
            
            return;
        }
        
        // if timer is running and it is time to start a resize work -> start resize work
        if let Some(time) = self.last_view_change_time {
            if time.elapsed() >= Duration::from_millis(100) {
                
                let repaint_signal = self.repaint_signal.clone();
                let started = self.image.resize(
                    &self.preview_size,
                    move || repaint_signal.request_repaint()
                );
                
                // if job has not started then do not stop trying to start it
                if started {
                    self.last_view_change_time = None;
                    self.repaint_signal.request_repaint();
                }
            }
            return;
        }
        
        // if preview image was updated then invalidate texture to be loaded in next pass
        if self.image.update_check() {
            self.texture = None;
        }
    }
    
    pub fn texture(&mut self, ctx: &egui::Context) -> &egui::TextureHandle {
        self.texture
            .get_or_insert_with(|| {
                let image = self.image.preview_working().to_rgba8();
                let pixels = image.as_flat_samples();
                let image = egui::ColorImage::from_rgba_unmultiplied(
                    [image.width() as _, image.height() as _],
                    pixels.as_slice(),
                );
                ctx.load_texture("working_image", image)
            })
    }
    
}

// ui code
impl ImageEditor {
    pub fn ui(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        
        let is_resizing = self.image.is_resizing();
        
        // bottom panel with image data
        egui::TopBottomPanel::bottom("info_bar")
            .show(ctx, |ui| {
                let original_size = self.image.original().size_vec2();
                let preview_size = self.image.preview_working().size_vec2();
                
                let sw = 20.0;
                let w = ui.available_width() - sw;
                
                ui.horizontal(|ui| {
                    ui.horizontal_wrapped(|ui| {
                        if is_resizing {
                            ui.set_max_width(w);
                        }
                        ui.label(self.media_file.file_name());
                        ui.separator();
                        ui.label( format!("Original size: {} x {}", original_size.x, original_size.y));
                        ui.separator();
                        ui.label( format!("Preview size: {} x {}", preview_size.x, preview_size.y));
                        ui.separator();
                        ui.label( format!("File size: {}", format_size(self.media_file.size())));
                        ui.separator();
                        ui.label( format!("Raw size: {}", format_size(self.image.original().raw_size())));
                        ui.separator();
                        ui.label( format!("Raw preview size: {}", format_size(self.image.preview_working().raw_size())));
                    });
                    if is_resizing  {
                        ui.with_layout(egui::Layout::right_to_left().with_cross_justify(true), |ui| {
                            ui.set_min_width(sw);
                            // ui.horizontal(|ui| {
                                ui.add(egui::Spinner::new());
                                frame.request_repaint();
                            // });
                        });
                    }
                });
        });
        
        // right editor panel
        egui::SidePanel::right("editor_panel")
            // .min_width(250.0)
            .resizable(false)
            .show(ctx,  |ui| {
                ui.add_space(10.0);
                
                let mut e = self.image.settings().exposure;
                EditorSlider(ui, "Exposure", &mut e, -1.0..=1.0);
                if e != self.image.settings().exposure {
                    self.image.update_settings(ImageSettings {exposure: e, ..*self.image.settings() });
                    self.texture = None;
                }
                
                ui.add_space(10.0);
            });
        
        // image viewport
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                self.size_viewport(ui.available_size() * 0.98);
                
                ctx.request_repaint();
                
                let offset = 0.5 * (ui.available_size() - self.preview_size);
                
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
                                ui.image(self.texture(ctx).id(), self.preview_size);
                            });
                    });
            });
            
    }
}
