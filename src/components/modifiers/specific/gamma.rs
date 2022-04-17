use cgmath::{Vector3, num_traits::Pow};
use image::{GenericImageView, GenericImage};

use crate::components::modifiers::{SliderCommonDataImp, SliderCommonUiImpl, Modifier, Slider, SliderData};

pub struct GammaModifier {
    data : SliderData
}

impl Default for GammaModifier {
    fn default() -> Self {
        Self {
            data: SliderData {
                units_name: "",
                percent: 0.0,
                min: -1.0,
                max: 1.0,
                ..Default::default()
            }
        }
    }
}

impl SliderCommonDataImp for GammaModifier {
    fn slider_data(&self) -> &SliderData { &self.data }
    fn slider_data_mut(&mut self) -> &mut SliderData { &mut self.data }
}

impl SliderCommonUiImpl for GammaModifier {}

impl Modifier for GammaModifier {
    fn title(&self) -> &str {
        "Gamma"
    }

    fn apply(&self, mut image: crate::components::Image) -> crate::components::Image {
        if !self.enabled() || self.percent() == 0.0 {
            return image;
        }
        
        let gamma = 1.0 - self.percent();
        let img = &mut image.raw_image;
        let (width, height) = img.dimensions();
        
        for y in 0..height {
            for x in 0..width {
                let mut pixel = img.get_pixel(x, y);
                    
                for i in 0..3 {
                    let mut p = pixel[i] as f32;
                    p /= 255.0;
                    p = p.pow(gamma);
                    p *= 255.0;
                    pixel[i] = p.clamp(0.0, 255.0) as u8;
                }
                img.put_pixel(x, y, pixel);
            }
        }
        
        image
    }
}
