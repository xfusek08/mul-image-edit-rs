
use std::fmt::Debug;

use image::DynamicImage;
use image::imageops::FilterType;
use image::imageops::colorops::{brighten_in_place, contrast_in_place};

pub struct Image {
    pub raw_image: DynamicImage,
    texture: Option<egui::TextureHandle>,
}

impl Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = match &self.texture {
            Some(t) => {
                let id_val = match t.id() {
                    egui::TextureId::Managed(v) => v,
                    egui::TextureId::User(v) => v,
                };
                format!("id: {id_val}")
            },
            None => "None".to_string(),
        };
        
        let i_size = self.size_vec2();
        
        f.debug_struct("Image")
            .field("raw_image", &format!("{} x {} - {}", i_size.x, i_size.y, self.raw_size()))
            .field("texture", &a)
            .finish()
    }
}

impl Clone for Image {
    fn clone(&self) -> Self {
        Self { raw_image: self.raw_image.clone(), texture: None }
    }
}

// constructors
impl Image {
    
    pub fn from_image(image: DynamicImage) -> Self {
        Self {
            raw_image: image,
            texture: None
        }
    }
    
    pub fn from_bytes(image_bytes: &[u8]) -> Result<Self, String> {
        let image = image::load_from_memory(image_bytes).map_err(|err| err.to_string())?;
        Ok(Self::from_image(image))
    }
    
}

// properties
impl Image {
    
    pub fn size_vec2(&self) -> egui::Vec2 {
        [self.raw_image.width() as f32, self.raw_image.height() as f32].into()
    }
    
    pub fn raw_size(&self) -> u64 {
        self.raw_image.as_bytes().len() as _
    }
    
    pub fn texture(&mut self, ctx: &egui::Context) -> &egui::TextureHandle {
        self.texture.get_or_insert_with(|| {
            let image = self.raw_image.to_rgba8();
            let pixels = image.as_flat_samples();
            let image = egui::ColorImage::from_rgba_unmultiplied(
                [image.width() as _, image.height() as _],
                pixels.as_slice(),
            );
            ctx.load_texture("", image)
        })
    }
}

// operations
impl Image {
    pub fn resize(&self, size: egui::Vec2, filter: FilterType) -> Image {
        Image {
            raw_image: self.raw_image.resize(
                size.x as u32,
                size.y as u32,
                filter
            ),
            texture: None,
        }
    }
    
    pub fn thumbnail(&self, w: u32, h: u32) -> Image {
        Image {
            raw_image: self.raw_image.thumbnail(w, h),
            texture: None,
        }
    }
    
    pub fn brighten_in_place(&mut self, level: f32) {
        let level = level.clamp(-100.0, 100.0) * 2.0;
        brighten_in_place(&mut self.raw_image, level as i32);
    }
    
    pub fn adjust_contrast(&mut self, level: f32) {
        let level = level.clamp(-100.0, 100.0);
        contrast_in_place(&mut self.raw_image, level);
    }
}

// ui rendering
impl Image {
    pub fn show(&mut self, ui: &mut egui::Ui) -> egui::Response {
        self.show_sized(ui, self.size_vec2())
    }
    
    pub fn show_sized(&mut self, ui: &mut egui::Ui, desired_size: egui::Vec2) -> egui::Response {
        ui.image(self.texture(ui.ctx()).id(), desired_size)
    }
}
