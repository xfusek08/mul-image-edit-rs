
use crate::components::Image;
use super::{Modifier, ModifierResponse, ModifierUi};

pub const THUMBNAIL_SIZE : f32 = 100.0;

#[derive(Default)]
pub(crate) struct SliderData {
    percent: f32,
    min_thumbnail: Option<Image>,
    max_thumbnail: Option<Image>,
}

pub trait Slider : Modifier + Default {
    
    fn percent(&self) -> f32;
    fn percent_mut(&mut self) -> &mut f32;
    fn min_thumbnail(&mut self) -> &mut Option<crate::components::Image>;
    fn max_thumbnail(&mut self) -> &mut Option<crate::components::Image>;
    
    // Construction with thumbnails
    
    fn with_thumbnails(original_thumbnail: &Image) -> Self {
        let mut tmp_instance = Self::default();
        
        // if given image is bigger than allowed thumbnail size create a thumbnail of proper size
        let mut thumbnail = original_thumbnail;
        if thumbnail.size_vec2().max_elem() > THUMBNAIL_SIZE {
            let i = thumbnail.thumbnail(THUMBNAIL_SIZE as u32, THUMBNAIL_SIZE  as u32);
            thumbnail = &i;
        }
        
        // create thumbnail of low applied filter
        *tmp_instance.percent_mut() = 20.0;
        let low_image = tmp_instance.apply(thumbnail.clone());
        
        // create thumbnail of high applied filter
        *tmp_instance.percent_mut() = 80.0;
        let high_image = tmp_instance.apply(thumbnail.clone());
        
        // create true result and attach the generated previews
        let this = Self::default();
        *this.min_thumbnail() = Some(low_image);
        *this.max_thumbnail() = Some(high_image);
        this
    }
}

/// implement common ui method for all Sliders
impl<T> ModifierUi for T
where
    T: Slider,
{
    fn ui(&mut self, ui: &mut egui::Ui) -> ModifierResponse {
        let h = ui.available_height();
        
        let draw_thumbnail = |image : &mut Option<Image>| {
            match image {
                Some(image) =>  { image.show_sized(ui, [h ,h].into()); },
                None => { },
            };
        };
        
        let mut res = ModifierResponse::Nothing;
        
        ui.horizontal(|ui| {
            draw_thumbnail(self.min_thumbnail());
            ui.vertical(|ui| {
                ui.label(self.title());
                let mut percent : f32 = self.percent();
                ui.add(egui::Slider::new(&mut percent, -100.0..=100.0).clamp_to_range(true));
                if percent != self.percent() {
                    *self.percent_mut() = percent;
                    res = ModifierResponse::Changed;
                }
            });
            draw_thumbnail(self.max_thumbnail());
        });
        
        res
    }
}

// helper trait implementing common data required by Slider

pub trait SliderWithData {
    fn slider_data_mut(&mut self) -> &mut SliderData;
}

impl<T> Slider for T
where
    T: SliderWithData + Default + Modifier
{
    fn percent(&self) -> f32 {
        self.slider_data_mut().percent
    }
 
    fn percent_mut(&mut self) -> &mut f32 {
        &mut self.slider_data_mut().percent
    }

    fn min_thumbnail(&mut self) -> &mut Option<Image> {
        &mut self.slider_data_mut().min_thumbnail
    }

    fn max_thumbnail(&mut self) -> &mut Option<Image> {
        &mut self.slider_data_mut().max_thumbnail
    }
}
