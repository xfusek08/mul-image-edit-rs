
use egui::Vec2;
use egui_extras::RetainedImage;

use indoc::indoc;

use super::MultimediaFile;

pub struct ImageEditorState {
    pub media_file: MultimediaFile,
    pub image: RetainedImage,
    pub center: Vec2,
    pub zoom: f32,
}

impl ImageEditorState {
    
    pub fn from_file(mut media_file: MultimediaFile) -> Result<Self, String> {
        
        let image = RetainedImage::from_image_bytes(
            media_file.file_name_owned(),
            media_file.bytes().as_slice(),
        );
        
        match image {
            Ok(image) => Ok(Self {
                media_file,
                image,
                center: [0.0, 0.0].into(),
                zoom: 1.0,
            }),
            Err(message) => Err(format!(indoc!("
                Image editor initiation error:
                    Media File:
                        {}
                    Message: \"{}\"
            "), media_file, message)),
        }
    }
    
    pub fn zoom_to_fit_size(&mut self, size: Vec2) {
        let scaling  = size / self.image.size_vec2();
        self.zoom = scaling.min_elem().min(1.0);
    }
    
}
