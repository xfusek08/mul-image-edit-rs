
use crate::utils::math::lramp;

use crate::components::modifiers::{SliderCommonDataImp, SliderCommonUiImpl, Modifier, Slider, SliderData};

const MAX_BLUR: f32 = 3.0;

pub struct BlurModifier {
    data : SliderData
}

impl Default for BlurModifier {
    fn default() -> Self {
        Self {
            data: SliderData {
                min: 0.0,
                max: 100.0,
                ..Default::default()
            }
        }
    }
}

impl SliderCommonDataImp for BlurModifier {
    fn slider_data(&self) -> &SliderData { &self.data }
    fn slider_data_mut(&mut self) -> &mut SliderData { &mut self.data }
}

impl SliderCommonUiImpl for BlurModifier {}

impl Modifier for BlurModifier {
    fn title(&self) -> &str {
        "Blur"
    }

    fn apply(&self, mut image: crate::components::Image) -> crate::components::Image {
        if !self.enabled() || self.percent() == 0.0 {
            return image;
        }
        image.raw_image =  image.raw_image.blur(lramp(0.0, MAX_BLUR, self.percent() * 0.01));
        image
    }
}
