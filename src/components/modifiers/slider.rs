
use crate::{components::Image, constants::THUMBNAIL_SIZE};
use super::{Modifier, ModifierResponse, ModifierUi};

pub trait Slider : Modifier + Default {
    
    // common data for any slider
    fn percent(&self) -> f32;
    fn percent_mut(&mut self) -> &mut f32;
    fn min_percent(&self) -> f32;
    fn max_percent(&self) -> f32;
    fn min_thumbnail(&mut self) -> &mut Option<crate::components::Image>;
    fn max_thumbnail(&mut self) -> &mut Option<crate::components::Image>;
    
    fn set_percent(&mut self, value: f32) {
        *self.percent_mut() = value.clamp(self.min_percent(), self.max_percent());
    }
    
    fn with_thumbnails(original_thumbnail: &Image) -> Self {
        let mut tmp_instance = Self::default();
        
        // if given image is bigger than allowed thumbnail size create a thumbnail of proper size
        let thumbnail =  if original_thumbnail.size_vec2().max_elem() > THUMBNAIL_SIZE {
            original_thumbnail.thumbnail(THUMBNAIL_SIZE as u32, THUMBNAIL_SIZE  as u32)
        } else {
            original_thumbnail.clone()
        };
        
        // create thumbnail of low applied filter
        tmp_instance.set_percent(tmp_instance.min_percent() * 0.8);
        let low_image = tmp_instance.apply(thumbnail.clone());
        
        // create thumbnail of high applied filter
        tmp_instance.set_percent(tmp_instance.max_percent() * 0.8);
        let high_image = tmp_instance.apply(thumbnail);
        
        // create true result and attach the generated previews
        let mut this = Self::default();
        *this.min_thumbnail() = Some(low_image);
        *this.max_thumbnail() = Some(high_image);
        this
    }
}

// Trait with common implementation for slider Ui
pub(crate) trait SliderCommonUiImpl : Slider {}
impl<T: SliderCommonUiImpl> ModifierUi for T {
    fn ui(&mut self, ui: &mut egui::Ui) -> ModifierResponse {
        let mut res = ModifierResponse::Nothing;
        
        ui.horizontal(|ui| {
            
            macro_rules! draw_image_option { ($e:expr) => {
                if let Some(image) = $e {
                    image.show_sized(ui, [THUMBNAIL_SIZE, THUMBNAIL_SIZE].into());
                };
            }}
            
            draw_image_option!(self.min_thumbnail());
            ui.vertical(|ui| {
                
                let min = self.min_percent();
                let max = self.max_percent();
                let mut percent = self.percent();
                
                ui.label(format!("{}:", self.title()));
                ui.horizontal(|ui| {
                    ui.spacing_mut().slider_width = ui.available_width() - THUMBNAIL_SIZE * 2.0 - 20.0;
                    ui.add(egui::Slider::new(&mut percent, min..=max).clamp_to_range(true));
                });
                
                if percent != self.percent() {
                    self.set_percent(percent);
                    res = ModifierResponse::Changed;
                }
            });
            draw_image_option!(self.max_thumbnail());
        });
        
        res
    }
}

// Trait with common implementation for slider data
pub(crate) struct SliderData {
    pub percent: f32,
    pub min: f32,
    pub max: f32,
    pub min_thumbnail: Option<Image>,
    pub max_thumbnail: Option<Image>,
}
impl Default for SliderData {
    fn default() -> Self {
        Self {
            percent: 0.0,
            min: -100.0,
            max: 100.0,
            min_thumbnail: None,
            max_thumbnail: None,
        }
    }
}
pub(crate) trait SliderCommonDataImp {
    fn slider_data(&self) -> &SliderData;
    fn slider_data_mut(&mut self) -> &mut SliderData;
}
impl<T: SliderCommonDataImp + Modifier + Default> Slider for T {
    fn percent(&self) -> f32 { self.slider_data().percent }
    fn percent_mut(&mut self) -> &mut f32 { &mut self.slider_data_mut().percent }
    fn min_percent(&self) -> f32 { self.slider_data().min }
    fn max_percent(&self) -> f32 { self.slider_data().max }
    fn min_thumbnail(&mut self) -> &mut Option<Image> { &mut self.slider_data_mut().min_thumbnail }
    fn max_thumbnail(&mut self) -> &mut Option<Image> { &mut self.slider_data_mut().max_thumbnail }
}
