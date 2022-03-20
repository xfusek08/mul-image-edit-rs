
use egui::Vec2;

pub fn fit_into(outer: &Vec2, inner: &Vec2, allow_upscale : bool) -> Vec2 {
    let scale = (*outer / *inner).min_elem();
    *inner * match allow_upscale {
        true => scale,
        false => scale.min(1.0),
    }
}
