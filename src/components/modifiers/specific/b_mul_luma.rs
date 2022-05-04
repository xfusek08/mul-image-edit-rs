
use cgmath::{num_traits::Pow, Matrix3, Vector3, Matrix};
use image::{GenericImageView, GenericImage};

use crate::{components::modifiers::{SliderCommonDataImp, SliderCommonUiImpl, Modifier, Slider, SliderData, ModifierUi, ModifierResponse}, constants::THUMBNAIL_SIZE, widgets::texts};

pub struct BMulLumaModifier {
    gamma: f32,
    data : SliderData,
}

impl Default for BMulLumaModifier {
    fn default() -> Self {
        Self {
            gamma: 0.5,
            data: SliderData {
                units_name: "",
                percent: 0.0,
                min: -2.0,
                max: 2.0,
                ..Default::default()
            }
        }
    }
}

impl SliderCommonDataImp for BMulLumaModifier {
    fn slider_data(&self) -> &SliderData { &self.data }
    fn slider_data_mut(&mut self) -> &mut SliderData { &mut self.data }
}

impl SliderCommonUiImpl for BMulLumaModifier {
    fn additional_elements(&mut self, ui: &mut egui::Ui) -> ModifierResponse {
        let mut gamma = self.gamma;
        ui.add(egui::DragValue::new(&mut gamma).speed(0.001).clamp_range(0.1..=1.5));
        if gamma != self.gamma {
            self.gamma = gamma;
            return ModifierResponse::Changed
        }
        ModifierResponse::Nothing
    }
    
    fn reset(&mut self) {
        *self.percent_mut() = Self::default().percent();
        self.gamma = Self::default().gamma;
    }
}

impl Modifier for BMulLumaModifier {
    fn title(&self) -> &str {
        "Brightness multiply luma"
    }

    fn apply(&self, mut image: crate::components::Image) -> crate::components::Image {
        if !self.enabled() || self.percent() == 0.0 {
            return image;
        }
        
        let coefficient = self.percent();
        let img = &mut image.raw_image;
        let (width, height) = img.dimensions();
        
        // RGB <=> YUV transforms taken from SDTV with BT.470
        // https://en.wikipedia.org/wiki/YUV#:~:text=Equal%20values%20of%20red%2C%20green,video%20cameras%20use%20Y%E2%80%B2CbCr.
        let rgb_to_yuv = Matrix3::<f32>::new(
            0.299, 0.587, 0.114,
            -0.14713, -0.28886, 0.436,
            0.615, -0.51499, -0.10001
        ).transpose();
        
        let yuv_to_rgb = Matrix3::<f32>::new(
            1.0, 0.0, 1.13983,
            1.0, -0.39465, -0.58060,
            1.0, 2.03211, 0.0
        ).transpose();
        
        for y in 0..height {
            for x in 0..width {
                let mut pixel = img.get_pixel(x, y);
                
                let rgb = Vector3::<f32>::new(
                    pixel[0] as f32,
                    pixel[1] as f32,
                    pixel[2] as f32
                ) / 255.0;
                
                let mut yuv = rgb_to_yuv * rgb;
                yuv.x *= 1.0 + coefficient * (1.0 - yuv.x.pow(self.gamma));
                let rgb = yuv_to_rgb * yuv;
                pixel[0] = (rgb.x * 255.0).clamp(0.0, 255.0) as u8;
                pixel[1] = (rgb.y * 255.0).clamp(0.0, 255.0) as u8;
                pixel[2] = (rgb.z * 255.0).clamp(0.0, 255.0) as u8;
                
                img.put_pixel(x, y, pixel);
            }
        }
        
        image
    }
}
