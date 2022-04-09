
use image::imageops::colorops;

use super::{SliderData, SliderCommonDataImp, SliderCommonUiImpl, Modifier, Slider};


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
        
        colorops::brighten_in_place(
            &mut image.raw_image,
            (self.percent().clamp(-100.0, 100.0) * 2.55) as i32
        );
        
        image
    }
}
