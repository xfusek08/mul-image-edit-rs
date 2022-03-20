
pub fn big(text: &str) -> impl Into<egui::WidgetText> {
    sized(text, 30.0)
}

pub fn sized(text: &str, size: f32) -> impl Into<egui::WidgetText> {
    egui::RichText::new(text).font(egui::FontId::proportional(size))
}
