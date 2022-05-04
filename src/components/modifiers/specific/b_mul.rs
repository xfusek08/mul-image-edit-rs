
use image::{GenericImageView, GenericImage};

use crate::components::modifiers::{SliderCommonDataImp, SliderCommonUiImpl, Modifier, Slider, SliderData};

pub struct BMulModifier {
    data : SliderData
}

impl Default for BMulModifier {
    fn default() -> Self {
        Self {
            data: SliderData {
                percent: 1.0,
                units_name: "",
                min: 0.0,
                max: 3.0,
                ..Default::default()
            }
        }
    }
}

impl SliderCommonDataImp for BMulModifier {
    fn slider_data(&self) -> &SliderData { &self.data }
    fn slider_data_mut(&mut self) -> &mut SliderData { &mut self.data }
}

impl SliderCommonUiImpl for BMulModifier {}

impl Modifier for BMulModifier {
    fn title(&self) -> &str {
        "Brightness multiply"
    }
    
    fn apply(&self, mut image: crate::components::Image) -> crate::components::Image {
        if !self.enabled() || self.percent() == 1.0 {
            return image;
        }
        
        let coefficient = self.percent();
        
        let img = &mut image.raw_image;
        let (width, height) = img.dimensions();
        
        for y in 0..height {
            for x in 0..width {
                let mut pixel = img.get_pixel(x, y);
                for i in 0..3 {
                    let mut v = pixel[i] as f32;
                    v *= coefficient;
                    pixel[i] = v.clamp(0.0, 255.0) as u8;
                }
                img.put_pixel(x, y, pixel);
            }
        }
        
        image
    }
}
