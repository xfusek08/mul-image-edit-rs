
use image::imageops::colorops;

use crate::components::modifiers::{SliderCommonDataImp, SliderCommonUiImpl, Modifier, Slider, SliderData};

pub struct TintModifier {
    data : SliderData
}

impl Default for TintModifier {
    fn default() -> Self {
        Self {
            data: SliderData {
                min: 0.0,
                max: 360.0,
                ..Default::default()
            }
        }
    }
}

impl SliderCommonDataImp for TintModifier {
    fn slider_data(&self) -> &SliderData { &self.data }
    fn slider_data_mut(&mut self) -> &mut SliderData { &mut self.data }
}

impl SliderCommonUiImpl for TintModifier {}

impl Modifier for TintModifier {
    fn title(&self) -> &str {
        "Tint"
    }

    fn apply(&self, mut image: crate::components::Image) -> crate::components::Image {
        if self.percent() == 0.0 {
            return image;
        }
        colorops::huerotate_in_place(&mut image.raw_image, self.percent() as i32);
        image
    }
}
