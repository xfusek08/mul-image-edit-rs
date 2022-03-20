
use std::time::{Instant, Duration};

use egui::Vec2;
use egui::style::Margin;
use indoc::indoc;

use crate::utils::{fit_into, format_size};
use crate::data::{MultimediaFile, EditedImage, Viewport, EditedImageComponent};

pub struct ImageEditor {
    media_file: MultimediaFile,
    image: EditedImage,
    preview_size: Vec2,
    viewport: Viewport,
    texture: Option<egui::TextureHandle>,
    last_view_change_time: Option<Instant>,
}

// constructors
impl ImageEditor {
    
    pub fn from_file(mut media_file: MultimediaFile) -> Result<Self, String> {
        match EditedImage::from_bytes(media_file.bytes().as_slice()) {
            Ok(image) => {
                let viewport = Viewport::new().sized(image.original().size_vec2());
                let preview_size = viewport.size;
                
                Ok(Self {
                    media_file,
                    image,
                    viewport,
                    preview_size,
                    texture: None,
                    last_view_change_time: None
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
        if self.viewport.size != size {
            self.viewport.size = size;
            self.preview_size = fit_into(
                &self.viewport.size,
                &self.image.original().size_vec2(),
                false
            );
            self.last_view_change_time = Some(Instant::now());
            println!("Resized.");
        } else if let Some(time) = self.last_view_change_time {
            if time.elapsed() >= Duration::from_secs(2) {
                println!("Updating...");
                self.image.update_preview(&self.preview_size);
                println!("Finished");
                self.last_view_change_time = None;
            }
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
    pub fn ui(&mut self, ctx: &egui::Context) {
        
        // bottom panel with image data
        egui::TopBottomPanel::bottom("info_bar")
            .show(ctx, |ui| {
                let original_size = self.image.original().size_vec2();
                let preview_size = self.image.preview_working().size_vec2();
                
                ui.horizontal_wrapped(|ui| {
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
                    ui.separator();
                });
            });
        
        // image viewport
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                self.size_viewport(ui.available_size() * 0.98);
                
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
