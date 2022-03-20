
use std::time::{Instant, Duration};

use egui::Vec2;

use indoc::indoc;

use crate::utils::fit_into;

use super::{MultimediaFile, EditedImage, EditedImageComponent};

pub struct ImageEditorState {
    media_file: MultimediaFile,
    
    // working data
    edited_image: EditedImage,
    
    // immediate drawing
    preview_size: Vec2,
    viewport: Viewport,
    texture: Option<egui::TextureHandle>,
    last_view_change_time: Option<Instant>,
}

// factories
impl ImageEditorState {
    
    pub fn from_file(mut media_file: MultimediaFile) -> Result<Self, String> {
        match EditedImage::from_bytes(media_file.bytes().as_slice()) {
            Ok(edited_image) => {
                let viewport = Viewport::new().sized(edited_image.original().size_vec2());
                let preview_size = viewport.size;
                
                Ok(Self {
                    media_file,
                    edited_image,
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

// mutable methods
impl ImageEditorState {
    
    pub fn size_viewport(&mut self, size: Vec2) {
        if self.viewport.size != size {
            self.viewport.size = size;
            self.preview_size = fit_into(
                &self.viewport.size,
                &self.edited_image.original().size_vec2(),
                false
            );
            self.last_view_change_time = Some(Instant::now());
            println!("Resized.");
        } else if let Some(time) = self.last_view_change_time {
            if time.elapsed() >= Duration::from_secs(2) {
                println!("Updating...");
                self.edited_image.update_preview(&self.preview_size);
                println!("Finished");
                self.last_view_change_time = None;
            }
        }
    }
    
    pub fn texture(&mut self, ctx: &egui::Context) -> &egui::TextureHandle {
        self.texture
            .get_or_insert_with(|| {
                let image = self.edited_image.preview_working().to_rgba8();
                let pixels = image.as_flat_samples();
                let image = egui::ColorImage::from_rgba_unmultiplied(
                    [image.width() as _, image.height() as _],
                    pixels.as_slice(),
                );
                ctx.load_texture("working_image", image)
            })
    }
    
}

// immutable methods
impl ImageEditorState {
    
    pub fn media_file(&self) -> &MultimediaFile {
        &self.media_file
    }
    
    pub fn image(&self) -> &EditedImage {
        &self.edited_image
    }
    
    pub fn viewport(&self) -> &Viewport {
        &self.viewport
    }
    
    pub fn preview_size(&self) -> &Vec2 {
        &self.preview_size
    }
    
}


#[derive(PartialEq, Default)]
pub struct Viewport {
    pub offset: Vec2,
    pub size: Vec2,
    pub zoom_level: f32,
}

impl Viewport {
    pub const ZERO: Self = Self {
        offset: Vec2::ZERO,
        size: Vec2::ZERO,
        zoom_level: 0.0,
    };
    
    pub fn new() -> Self {
        Self::ZERO
    }
    
    pub fn sized(mut self, size: impl Into<Vec2>) -> Self {
        self.size = size.into();
        self
    }
}
