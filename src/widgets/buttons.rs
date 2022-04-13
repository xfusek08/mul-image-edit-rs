
use egui::{Ui, Response, Layout, Direction, vec2, CursorIcon};

use super::texts;
pub struct BigButton;
impl BigButton {
    pub fn ui(ui: &mut Ui, label: &str) -> Response {
        ui.vertical(|ui| {
            ui.set_height(25.0);
            ui.spacing_mut().button_padding = vec2(8.0, 8.0);
            ui.with_layout(
                Layout::centered_and_justified(Direction::LeftToRight),
                |ui| {
                    let r = ui.button(texts::sized(label, 20.0));
                    if r.hovered() {
                        ui.ctx().output().cursor_icon = CursorIcon::PointingHand;
                    }
                    r
                }).inner
        }).inner
    }
}
