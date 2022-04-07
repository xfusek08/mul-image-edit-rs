
use image::imageops::colorops;

use super::{Slider, SliderData, Modifier, SliderWithData};

#[derive(Default)]
pub struct ExposureModifier {
    data : SliderData
}

impl SliderWithData for ExposureModifier {
    fn slider_data_mut(&mut self) -> &mut SliderData {
        &mut self.data
    }
}

impl Modifier for ExposureModifier {
    fn title(&self) -> &str {
        "exposure"
    }

    fn apply(&self, image: crate::components::Image) -> crate::components::Image {
        colorops::brighten_in_place(
            &mut image.raw_image,
            (self.percent().clamp(-100.0, 100.0) * 2.55) as i32
        );
        image
    }
}
