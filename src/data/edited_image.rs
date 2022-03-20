
use egui::{Vec2};
use image::{DynamicImage, imageops::FilterType};

use super::Viewport;

/// Sliders to adjust image properties
/// values are always offsets from original ( -x <-- 0 --> +x )
#[derive(Default)]
pub struct ImageSettings {
    contrast: f32,
    exposure: f32,
    blur: f32,
}

pub struct EditedImage {
    original: DynamicImage,
    
    // copy of the image scaled to fit into the viewport
    preview_original: Option<DynamicImage>,
    preview_working: Option<DynamicImage>,
    
    /// Current image modifications
    settings: ImageSettings,
}

// factories
impl EditedImage {
    pub fn from_bytes(image_bytes: &[u8]) -> Result<Self, String> {
        let original = image::load_from_memory(image_bytes).map_err(|err| err.to_string())?;
        
        Ok(Self {
            original,
            preview_original: None,
            preview_working: None,
            settings: ImageSettings::default(),
        })
    }
}


// mutable methods
impl EditedImage {
    
    /// change preview copies of image to fit into viewport
    pub fn update_preview(&mut self, target_size: &Vec2) {
        // let scaled_size = viewport.size * ((1.0 - viewport.zoom_level));
        self.preview_original = Some(self.original.resize(
            target_size.x as u32,
            target_size.y as u32,
            FilterType::Gaussian
        ));
        
        // TODO: apply settings to create preview working
        self.preview_working = None;
    }
    
}

// immutable methods
impl EditedImage {
    
    pub fn original(&self) -> &DynamicImage {
        &self.original
    }
    
    /// make sure that something is returned
    pub fn preview_working(&self) -> &DynamicImage {
        match &self.preview_working {
            Some(image) => image,
            None => self.preview_original(),
        }
    }
        
    pub fn preview_original(&self) -> &DynamicImage {
        self.preview_original
            .as_ref()
            .unwrap_or(&self.original)
    }
    
}

// extend interface for DynamicImage to comply with needs of this implementation
pub trait EditedImageComponent {
    fn size_vec2(&self) -> Vec2;
    fn raw_size(&self) -> u64;
}

impl EditedImageComponent for DynamicImage {
    fn size_vec2(&self) -> Vec2 {
        [self.width() as f32, self.height() as f32].into()
    }
    
    fn raw_size(&self) -> u64 {
        self.as_bytes().len() as _
    }
}
