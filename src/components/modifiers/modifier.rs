
use crate::components::Image;

#[derive(PartialEq)]
pub enum ModifierResponse {
    Nothing,
    Selected,
    Changed
}

pub trait ModifierUi {
    fn ui(&mut self, ui: &mut egui::Ui) -> ModifierResponse;
}

pub trait Modifier : ModifierUi {
    
    fn title(&self) -> &str;
    
    fn apply(&self, image: Image) -> Image;
    
    fn show_preview(&mut self, ui: &mut egui::Ui) -> ModifierResponse {
        // TODO: render inner ui behind clickable overlay
        ui.group(|ui| self.ui(ui));
        ModifierResponse::Nothing
    }
    
    fn show_applied(&mut self, ui: &mut egui::Ui) -> ModifierResponse {
        ui.group(|ui| ui.label(self.title()));
        ModifierResponse::Nothing
    }
    
    fn show_active(&mut self, ui: &mut egui::Ui) -> ModifierResponse {
        let r = ui.group(|ui| self.ui(ui));
        r.inner
    }
}
