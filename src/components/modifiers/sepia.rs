
use image::{GenericImageView, GenericImage};

use crate::utils::math::lramp;

use super::{SliderData, SliderCommonDataImp, SliderCommonUiImpl, Modifier, Slider};

pub struct SepiaModifier {
    data : SliderData
}

impl Default for SepiaModifier {
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

impl SliderCommonDataImp for SepiaModifier {
    fn slider_data(&self) -> &SliderData { &self.data }
    fn slider_data_mut(&mut self) -> &mut SliderData { &mut self.data }
}

impl SliderCommonUiImpl for SepiaModifier {}

impl Modifier for SepiaModifier {
    fn title(&self) -> &str {
        "Sepia"
    }

    fn apply(&self, mut image: crate::components::Image) -> crate::components::Image {
        
        if self.percent() == 0.0 {
            return image;
        }
        
        let img = &mut image.raw_image;
        let (width, height) = img.dimensions();
        
        let level = self.percent() / 100.0;
        
        // ramp from one
        let rfz = |target: f32| lramp(0.0, target, level);
        
        // ramp from zero
        let rfo = |target: f32| lramp(1.0, target, level);
        
        for y in 0..height {
            for x in 0..width {
                
                let mut pixel = img.get_pixel(x, y);
                let r = pixel[0] as f32;
                let g = pixel[1] as f32;
                let b = pixel[2] as f32;
                
                let tr = rfo(0.393) * r + rfz(0.769) * g + rfz(0.189) * b;
                let tg = rfz(0.349) * r + rfo(0.686) * g + rfz(0.168) * b;
                let tb = rfz(0.272) * r + rfz(0.534) * g + rfo(0.131) * b;
                
                pixel[0] = tr.min(255.0) as u8;
                pixel[1] = tg.min(255.0) as u8;
                pixel[2] = tb.min(255.0) as u8;

                img.put_pixel(x, y, pixel);
            }
        }
        
        image
    }
}
