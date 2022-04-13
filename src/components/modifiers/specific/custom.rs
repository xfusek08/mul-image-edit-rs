
use cgmath::{Matrix3, Vector3, InnerSpace, Matrix, Matrix4};
use egui::{Layout, Align};
use image::{GenericImageView, GenericImage, Pixel};

use crate::constants::THUMBNAIL_SIZE;
use crate::utils::math::clamp_magnitude;
use crate::{utils::math::lramp};

use crate::components::modifiers::{SliderCommonDataImp, Modifier, Slider, SliderData, ModifierUi, ModifierResponse};

pub struct CustomModifier {
    data : SliderData,
    grading_matrix: Matrix3<f32>,
}

impl Default for CustomModifier {
    fn default() -> Self {
        Self {
            data: SliderData {
                min: 0.0,
                max: 100.0,
                ..Default::default()
            },
            grading_matrix: Matrix3::from_scale(1.0),
        }
    }
}

impl SliderCommonDataImp for CustomModifier {
    fn slider_data(&self) -> &SliderData { &self.data }
    fn slider_data_mut(&mut self) -> &mut SliderData { &mut self.data }
}

impl ModifierUi for CustomModifier {
    fn ui(&mut self, ui: &mut egui::Ui) -> crate::components::modifiers::ModifierResponse {
        
        let mut res = ModifierResponse::Nothing;
        let mut matrix_changed = false;
        
        ui.vertical(|ui| {
            
            // draw matrix editor
            ui.horizontal(|ui| {
                macro_rules! draw_image_option { ($e:expr) => {
                    if let Some(image) = $e {
                        ui.vertical(|ui| {
                            ui.set_height(THUMBNAIL_SIZE);
                            image.show_sized(ui, [THUMBNAIL_SIZE, THUMBNAIL_SIZE].into());
                        });
                    };
                }}
                
                draw_image_option!(self.min_thumbnail());
                ui.vertical(|ui| {
                    
                    let min = self.min_percent();
                    let max = self.max_percent();
                    let mut percent = self.percent();
                    
                    ui.label(format!("{}:", self.title()));
                    ui.horizontal(|ui| {
                        ui.spacing_mut().slider_width = ui.available_width() - THUMBNAIL_SIZE * 2.0 - 30.0;
                        ui.add(egui::Slider::new(&mut percent, min..=max).clamp_to_range(true));
                        ui.label(self.units_name());
                    });
                    
                    if percent != self.percent() {
                        self.set_percent(percent);
                        res = ModifierResponse::Changed;
                    }
                    
                    let rgb = ["R", "G", "B"];
                    for (i, label) in rgb.iter().enumerate() {
                        let mut row = self.grading_matrix[i];
                        ui.horizontal(|ui| {
                            ui.label(*label);
                            ui.label("=");
                            for (i, label) in rgb.iter().enumerate() {
                                ui.add(
                                    egui::DragValue::new(&mut row[i])
                                        .speed(0.001).clamp_range(0.0..=2.0)
                                );
                                ui.label(*label);
                                if i < 2 {
                                    ui.label("+");
                                }
                            }
                        });
                        
                        if row != self.grading_matrix[i] {
                            self.grading_matrix[i] = row;
                            res = ModifierResponse::Changed;
                            matrix_changed = true;
                        }
                        
                        ui.add_space(10.0);
                    }
                    
                });
                draw_image_option!(self.max_thumbnail());
            });
            
            // select presets
            
            ui.horizontal(|ui| {
                for (label, mat) in PREDEFINED_FILTERS.iter() {
                    if ui.button(*label).clicked() {
                        self.grading_matrix = *mat;
                        res = ModifierResponse::Changed;
                        matrix_changed = true;
                    }
                }
            });
            
        });
        
         // update max settings thumbnail
         if matrix_changed {
            if let Some(min_i) = self.data.min_thumbnail.clone() {
                let p = self.percent();
                self.set_percent(100.0);
                *self.max_thumbnail() = Some(self.apply(min_i));
                self.set_percent(p);
            }
        }
        
        res
    }
}

impl Modifier for CustomModifier {
    fn title(&self) -> &str {
        "Custom"
    }

    fn apply(&self, mut image: crate::components::Image) -> crate::components::Image {
        
        if self.percent() == 0.0 {
            return image;
        }
        
        let img = &mut image.raw_image;
        let (width, height) = img.dimensions();
        
        let level = self.percent() / 100.0;
        
        
        let rfz = |target: f32| lramp(0.0, target, level); // ramp from one
        let rfo = |target: f32| lramp(1.0, target, level); // ramp from zero
          
        let m = self.grading_matrix.transpose();
        let m = Matrix3::new(
            rfo(m[0][0]), rfz(m[0][1]), rfz(m[0][2]),
            rfz(m[1][0]), rfo(m[1][1]), rfz(m[1][2]),
            rfz(m[2][0]), rfz(m[2][1]), rfo(m[2][2])
        );
        
        // TODO: multithreaded + SIMD optimization?
        for y in 0..height {
            for x in 0..width {
                
                let mut pixel = img.get_pixel(x, y);
                
                let rgb = Vector3::new(
                    pixel[0] as f32,
                    pixel[1] as f32,
                    pixel[2] as f32
                );
                
                let rgb_transformed = m * rgb;
                
                for i in 0..3 {
                    pixel[i] = rgb_transformed[i].clamp(0.0, 255.0) as u8;
                }
                
                img.put_pixel(x, y, pixel);
            }
        }
        
        image
    }
}


static PREDEFINED_FILTERS: &[(&str, Matrix3<f32>)] = &[
    (
        "Grayscale Average", Matrix3::<f32>::new(
            0.333, 0.333, 0.333,
            0.333, 0.333, 0.333,
            0.333, 0.333, 0.333
        )
    ),
    (
        "Luma Brightness", Matrix3::<f32>::new(
            0.299, 0.587, 0.114,
            0.299, 0.587, 0.114,
            0.299, 0.587, 0.114,
        )
    ),
    (
        "Sepia", Matrix3::<f32>::new(
            0.393, 0.769, 0.189,
            0.349, 0.686, 0.168,
            0.272, 0.534, 0.131,
        )
    ),
];