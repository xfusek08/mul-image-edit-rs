
use egui::{Stroke, Pos2, style::Margin};

use crate::data::ImageEditorState;


pub struct EditorUi;

impl EditorUi {
    pub fn ui(state: &mut ImageEditorState, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                state.zoom_to_fit_size(ui.available_size() * 0.98);
                let image_size = state.image.size_vec2() * state.zoom;
                let offset = 0.5 * (ui.available_size() - image_size);
                
                egui::Frame::default()
                    .margin(Margin {
                        left: offset.x,
                        top: offset.y,
                        ..Default::default()
                    })
                    .show(ui, |ui| {
                        egui::Frame::none()
                            .shadow(ctx.style().visuals.popup_shadow)
                            .show(ui, |ui| {
                                state.image.show_scaled(ui, state.zoom);
                            });
                    });

                // egui::ScrollArea::both()
                //     .max_width(ui.available_width())
                //     .max_height(ui.available_height())
                //     .stick_to_right()
                //     .stick_to_bottom()
                //     .enable_scrolling(false)
                //     // .min_scrolled_height(ui.available_width())
                //     // .min_scrolled_width(ui.available_height())
                //     .show_viewport(ui,|ui, rect| {
                //         egui::Resize::default()
                //             // .fixed_size(rect.size() * 2.0)
                //             .resizable(false)
                //             .show(ui, |ui| {
                //                 egui::Frame::default()
                //                     .margin(Margin::symmetric(rect.width(), rect.height()))
                //                     .show(ui, |ui| {
                //                         egui::Frame::popup(&ctx.style())
                //                             .stroke(Stroke::none())
                //                             .show(ui, |ui| {
                //                                 state.image.show(ui);
                //                             });
                //                     });
                //             });
                //     });
            });
    }
}