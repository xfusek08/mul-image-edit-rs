use cgmath::{Vector3, Matrix3, num_traits::Pow};
use image::{GenericImageView, GenericImage, Pixel};

use crate::components::modifiers::{SliderCommonDataImp, SliderCommonUiImpl, Modifier, Slider, SliderData};

pub struct BMulLumaModifier {
    data : SliderData
}

impl Default for BMulLumaModifier {
    fn default() -> Self {
        Self {
            data: SliderData {
                units_name: "",
                percent: 1.0,
                min: -1.0,
                max: 5.0,
                ..Default::default()
            }
        }
    }
}

impl SliderCommonDataImp for BMulLumaModifier {
    fn slider_data(&self) -> &SliderData { &self.data }
    fn slider_data_mut(&mut self) -> &mut SliderData { &mut self.data }
}

impl SliderCommonUiImpl for BMulLumaModifier {}

impl Modifier for BMulLumaModifier {
    fn title(&self) -> &str {
        "Brightness multiply luma"
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
                
                let r = pixel[0] as f32;
                let g = pixel[1] as f32;
                let b = pixel[2] as f32;
                
                let l = 0.299 * r + 0.587 * g + 0.114 * b;
                // let l = (r + g + b) / 3.0;

                // let u = -0.16875 * r - 0.33126 * g + 0.5 * b;
                // let v = 0.5 * r - 0.41869 * g - 0.08131 * b;
                
                let c = if coefficient > 1.0 {
                    let mut c = coefficient - 1.0;
                    c *= 1.0 - (l / 255.0).pow(0.5);
                    1.0 + c
                } else {
                    let c = (1.0 - coefficient) * (1.0 - (l / 255.0).pow(1.5));
                    1.0 - c
                };
                
                for (i, v) in [r,g,b].iter().enumerate() {
                    let p = *v * c;
                    pixel[i] = p.clamp(0.0, 255.0) as u8;
                }
                // pixel[0] = (().clamp(0.0, 255.0) as u8_;
                // pixel[1] = p.clamp(0.0, 255.0) as u8;
                // pixel[2] = p.clamp(0.0, 255.0) as u8;
                
                img.put_pixel(x, y, pixel);
            }
        }
        
        image
    }
}
