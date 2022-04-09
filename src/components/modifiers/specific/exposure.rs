
use image::imageops::colorops;

use crate::components::modifiers::{SliderCommonDataImp, SliderCommonUiImpl, Modifier, Slider, SliderData};

pub struct ExposureModifier {
    data : SliderData
}

impl Default for ExposureModifier {
    fn default() -> Self {
        Self {
            data: SliderData {
                ..Default::default()
            }
        }
    }
}

impl SliderCommonDataImp for ExposureModifier {
    fn slider_data(&self) -> &SliderData { &self.data }
    fn slider_data_mut(&mut self) -> &mut SliderData { &mut self.data }
}

impl SliderCommonUiImpl for ExposureModifier {}

impl Modifier for ExposureModifier {
    fn title(&self) -> &str {
        "Exposure"
    }

    fn apply(&self, mut image: crate::components::Image) -> crate::components::Image {
        if self.percent() == 0.0 {
            return image;
        }
        
        let val = self.percent().clamp(-100.0, 100.0) * 0.6;
        
        colorops::brighten_in_place(
            &mut image.raw_image,
            (val * 2.55) as i32
        );
        
        image
    }
}
