
use image::imageops::colorops;

use crate::components::modifiers::{SliderCommonDataImp, SliderCommonUiImpl, Modifier, Slider, SliderData};

pub struct ContrastModifier {
    data : SliderData
}

impl Default for ContrastModifier {
    fn default() -> Self {
        Self {
            data: SliderData {
                ..Default::default()
            }
        }
    }
}

impl SliderCommonDataImp for ContrastModifier {
    fn slider_data(&self) -> &SliderData { &self.data }
    fn slider_data_mut(&mut self) -> &mut SliderData { &mut self.data }
}

impl SliderCommonUiImpl for ContrastModifier {}

impl Modifier for ContrastModifier {
    fn title(&self) -> &str {
        "Contrast"
    }

    fn apply(&self, mut image: crate::components::Image) -> crate::components::Image {
        if self.percent() == 0.0 {
            return image;
        }
          
        colorops::contrast_in_place(
            &mut image.raw_image,
            self.percent() * 0.6,
        );
        
        image
    }
}
