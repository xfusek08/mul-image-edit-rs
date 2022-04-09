
use image::imageops::FilterType;

use super::Image;
use super::modifiers::{ModifierResponse, Modifier};

pub struct ModifierPipeline {
    
    /// Image original in full size
    original_image: Image,
    
    /// Original copy in working resolution
    base_image: Option<Image>,
    
    /// Image with all modifiers applied to be rendered
    current_image: Option<Image>,
    
    /// Desired image size.
    ///   When pipeline is evaluated image is sized to this desired size.
    preview_size: egui::Vec2,
    
    /// All modifiers in pipeline.
    modifiers: Vec<Box<dyn Modifier>>,
    
    /// when set then modifier on this index will be rendered as active
    active_index: Option<usize>,
}

// construction
impl ModifierPipeline {
    
    pub fn new(original_image: Image, preview_size: egui::Vec2) -> Self {
        Self {
            original_image,
            base_image: None,
            current_image: None,
            preview_size,
            modifiers: vec![],
            active_index: None,
        }
    }

    fn select_modifier(&self, on_index: usize) {
        todo!()
    }

    fn apply_current_modifier(&self) {
        todo!()
    }
}


// non-mutable methods
impl ModifierPipeline {
    
    pub fn preview_size(&self) -> &egui::Vec2 {
        &self.preview_size
    }

    #[inline]
    pub fn original_image(&self) -> &Image {
        &self.original_image
    }
    
    #[inline]
    pub fn base_image(&self) -> &Image {
        &self.base_image.as_ref().unwrap_or(self.original_image())
    }
    
    #[inline]
    pub fn current_image(&self) -> &Image {
        &self.current_image.as_ref().unwrap_or(self.base_image())
    }
    
}


// mutable methods
impl ModifierPipeline {
    
    pub fn push_modifier(&mut self, modifier: Box<dyn Modifier>) {
        self.modifiers.push(modifier);
    }
    
    pub fn resize(&mut self, size: egui::Vec2) {
        if self.preview_size != size {
            self.preview_size = size;
            let scaling_diff = -1.0 * (1.0 - self.preview_size.x / self.current_image().size_vec2().x);
            
            // react only if image is sized up and beyond threshold
            if scaling_diff.abs() > 0.1 {
                self.reevaluate();
            }
        }
    }
    
    /// Runs whole pipeline from beginning
    ///   Commonly used when image is enlarged and new details should be in focus
    fn reevaluate(&mut self) {
        
        // if based image exists and does not need change -> return copy of it
        // otherwise create size copy of original
        let image = match &self.base_image {
            Some(i) if i.size_vec2() == self.preview_size => i.clone(),
            _ => {
                // create new sized image
                let image = self.original_image.resize(self.preview_size, FilterType::Nearest);
                
                // store sized copy of original for future use
                self.base_image = Some(image.clone());
                
                image // return result
            },
        };
        
        // if there are any modifiers
        if self.modifiers.len() > 0 {
            self.current_image = Some(self.modifiers
                .iter().fold(image, |acc, m| m.apply(acc))
            );
        }
    }
}


// rendering
impl ModifierPipeline {
    
    pub fn show_current_image(&mut self, ui: &mut egui::Ui) {
        // show image if defined in this order: current ?? base ?? original
        self.current_image.as_mut()
            .unwrap_or(self.base_image.as_mut().unwrap_or(&mut self.original_image))
            .show_sized(ui, self.preview_size);
    }
    
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let mut modifier_action = ModifierResponse::Nothing;
        let mut on_index = 0;
        
        self.modifiers
            .iter_mut()
            .enumerate()
            .for_each(|(i, m)| {
                
                let response = match self.active_index {
                    Some(active_index) if i == active_index => m.show_active(ui),
                    _ => m.show_applied(ui),
                };
                
                if response != ModifierResponse::Nothing {
                    modifier_action = response;
                    on_index = i;
                }
            });
        
        // react to action of particular modifier
        match modifier_action {
            ModifierResponse::Selected => self.select_modifier(on_index),
            ModifierResponse::Changed if on_index == self.active_index.unwrap_or(self.modifiers.len()) => self.apply_current_modifier(),
            ModifierResponse::Changed => self.reevaluate(),
            _ => ()
        }
    }
    
}