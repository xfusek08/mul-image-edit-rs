
use crate::{components::Image, constants::{THUMBNAIL_SIZE, RIGHT_PANEL_WIDTH}, widgets::texts};
use super::{Modifier, ModifierResponse, ModifierUi};

pub trait Slider : Modifier + Default {
    
    // common data for any slider
    fn percent(&self) -> f32;
    fn percent_mut(&mut self) -> &mut f32;
    fn min_percent(&self) -> f32;
    fn max_percent(&self) -> f32;
    fn units_name(&self) -> &str;
    
    fn min_thumbnail(&mut self) -> &mut Option<crate::components::Image>;
    fn max_thumbnail(&mut self) -> &mut Option<crate::components::Image>;
    
    fn enabled(&self) -> bool;
    fn enabled_mut(&mut self) -> &mut bool;
    
    fn set_percent(&mut self, value: f32) {
        *self.percent_mut() = value.clamp(self.min_percent(), self.max_percent());
    }
    
    fn with_thumbnails(original_thumbnail: &Image) -> Self {
        let mut tmp_instance = Self::default();
        *tmp_instance.enabled_mut() = true;
        
        // if given image is bigger than allowed thumbnail size create a thumbnail of proper size
        let thumbnail =  if original_thumbnail.size_vec2().max_elem() > THUMBNAIL_SIZE {
            original_thumbnail.thumbnail(THUMBNAIL_SIZE as u32, THUMBNAIL_SIZE  as u32)
        } else {
            original_thumbnail.clone()
        };
        
        // create thumbnail of low applied filter
        tmp_instance.set_percent(tmp_instance.min_percent() * 0.8);
        let low_image = tmp_instance.apply(thumbnail.clone());
        
        // create thumbnail of high applied filter
        tmp_instance.set_percent(tmp_instance.max_percent() * 0.8);
        let high_image = tmp_instance.apply(thumbnail);
        
        // create true result and attach the generated previews
        let mut this = Self::default();
        *this.min_thumbnail() = Some(low_image);
        *this.max_thumbnail() = Some(high_image);
        this
    }
}

// Trait with common implementation for slider Ui
pub(crate) trait SliderCommonUiImpl : Slider {
    fn ui(&mut self, ui: &mut egui::Ui) -> ModifierResponse {
        let mut res = ModifierResponse::Nothing;
        
        ui.vertical(|ui| {
            ui.set_min_height(THUMBNAIL_SIZE);
            ui.horizontal(|ui| {
                let l = format!("{}:", self.title());
                if ui.checkbox(self.enabled_mut(), texts::sized(&l, 20.0)).changed() {
                    res = ModifierResponse::Changed;
                }
                ui.add_space(ui.available_width() - 55.0);
                if ui.button(texts::sized("Reset", 17.0)).clicked() {
                    self.reset();
                    res = ModifierResponse::Changed;
                }
            });
            ui.separator();
            ui.horizontal(|ui| {
                macro_rules! draw_image_option { ($e:expr) => {
                    if let Some(image) = $e {
                        image.show_sized(ui, [THUMBNAIL_SIZE, THUMBNAIL_SIZE].into());
                    };
                }}
                
                draw_image_option!(self.min_thumbnail());
                ui.vertical(|ui| {
                    
                    let min = self.min_percent();
                    let max = self.max_percent();
                    // let step = (max - min) / 100.0;
                    let mut percent = self.percent();
                    
                    ui.horizontal(|ui| {
                        ui.set_height(THUMBNAIL_SIZE);
                        ui.spacing_mut().slider_width = ui.available_width() - THUMBNAIL_SIZE * 2.0 - 25.0;
                        ui.add(
                            egui::Slider::new(&mut percent, min..=max)
                                // .step_by(step as f64)
                                .clamp_to_range(true)
                        );
                        ui.label(self.units_name());
                    });
                    
                    if percent != self.percent() {
                        self.set_percent(percent);
                        res = ModifierResponse::Changed;
                    }
                    
                    if self.additional_elements(ui) == ModifierResponse::Changed {
                        res = ModifierResponse::Changed;
                    }
                });
                draw_image_option!(self.max_thumbnail());
            });
        });
        res
    }
    
    fn additional_elements(&mut self, ui: &mut egui::Ui) -> ModifierResponse {
        ModifierResponse::Nothing
    }
    
    fn reset(&mut self) {
        *self.percent_mut() = Self::default().percent();
    }
}

impl<T: SliderCommonUiImpl> ModifierUi for T {
    fn ui(&mut self, ui: &mut egui::Ui) -> ModifierResponse {
        SliderCommonUiImpl::ui(self, ui)
    }
}

// Trait with common implementation for slider data
pub(crate) struct SliderData {
    pub enabled: bool,
    pub percent: f32,
    pub units_name: &'static str,
    pub min: f32,
    pub max: f32,
    pub min_thumbnail: Option<Image>,
    pub max_thumbnail: Option<Image>,
}
impl Default for SliderData {
    fn default() -> Self {
        Self {
            enabled: true,
            percent: 0.0,
            units_name: "%",
            min: -100.0,
            max: 100.0,
            min_thumbnail: None,
            max_thumbnail: None,
        }
    }
}
pub(crate) trait SliderCommonDataImp {
    fn slider_data(&self) -> &SliderData;
    fn slider_data_mut(&mut self) -> &mut SliderData;
}
impl<T: SliderCommonDataImp + Modifier + Default> Slider for T {
    fn percent(&self) -> f32 { self.slider_data().percent }
    fn percent_mut(&mut self) -> &mut f32 { &mut self.slider_data_mut().percent }
    
    fn units_name(&self) -> &str { self.slider_data().units_name }
    fn min_percent(&self) -> f32 { self.slider_data().min }
    fn max_percent(&self) -> f32 { self.slider_data().max }
    
    fn min_thumbnail(&mut self) -> &mut Option<Image> { &mut self.slider_data_mut().min_thumbnail }
    fn max_thumbnail(&mut self) -> &mut Option<Image> { &mut self.slider_data_mut().max_thumbnail }
    
    fn enabled(&self) -> bool  { self.slider_data().enabled }
    fn enabled_mut(&mut self) -> &mut bool { &mut self.slider_data_mut().enabled }
}
