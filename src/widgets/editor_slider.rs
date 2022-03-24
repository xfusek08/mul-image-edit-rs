
use std::ops::RangeInclusive;

use egui::{Ui, Response};

pub fn EditorSlider(ui: &mut Ui, label: &str, value: &mut f32, range: RangeInclusive<f32>) -> Response {
    let response = ui.group(|ui| {
        ui.label(label);
        ui.set_min_width(250.0);
        ui.horizontal(|ui| {
            ui.spacing_mut().slider_width = 250.0 - 65.0;
            ui.add(egui::Slider::new(value, range).clamp_to_range(true))
        })
    });
    response.inner.inner
}
