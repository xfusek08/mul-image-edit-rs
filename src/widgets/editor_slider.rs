
use std::ops::RangeInclusive;

use egui::{Ui, Response};
pub struct EditorSlider;
impl EditorSlider {
    pub fn ui(ui: &mut Ui, label: &str, value: &mut f32, range: RangeInclusive<f32>) -> Response {
        let response = ui.group(|ui| {
            ui.label(label);
            ui.horizontal(|ui| {
                ui.spacing_mut().slider_width = ui.available_width() - 65.0;
                ui.add(egui::Slider::new(value, range).clamp_to_range(true))
            })
        });
        response.inner.inner
    }
}
