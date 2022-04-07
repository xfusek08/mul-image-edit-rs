
use image::imageops::colorops;

use super::{Slider, SliderData, Modifier, SliderWithData};

#[derive(Default)]
pub struct ContrastModifier {
    data : SliderData
}

impl SliderWithData for ContrastModifier {
    fn slider_data_mut(&mut self) -> &mut SliderData {
        &mut self.data
    }
}

impl Modifier for ContrastModifier {
    fn title(&self) -> &str {
        "exposure"
    }

    fn apply(&self, image: crate::components::Image) -> crate::components::Image {
        colorops::contrast_in_place(
            &mut image.raw_image,
            self.percent(),
        );
        image
    }
}
