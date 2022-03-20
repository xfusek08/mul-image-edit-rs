
use crate::{
    data::AppState,
    utils::load_input_file
};

#[cfg(debug_assertions)]
use super::DebugBottomPanel;
use super::{utils::{self, BigText}, EditorUi};

pub struct AppUi;

impl AppUi {
    
    pub fn ui(state: &mut AppState, ctx: &egui::Context, frame: &epi::Frame) {
        
        #[cfg(debug_assertions)]
        DebugBottomPanel::ui(state, ctx, frame);
        
        if state.error_message.is_some() {
            
            // TODO: render error message as overlay
            
            // Render error of invalid file
            utils::CenteredWindow::show(ctx, |ui| {
                // let msg = message.clone();
                ui.label(BigText("Selected file is invalid"));
                ui.label(format!("{}", state.error_message.as_ref().unwrap()));
                Self::open_file_dialog(state, ui);
            });
            
        } else if let Some(editor_state) = &mut state.editor_state {
            
            EditorUi::ui(editor_state, ctx);
            
        } else {
            
            // No file
            utils::CenteredWindow::show(ctx, |ui| {
                ui.label(BigText("Select image to edit"));
                Self::open_file_dialog(state, ui);
            });
        }
    }
    
    fn open_file_dialog(state : &mut AppState, ui : &mut egui::Ui) {
        if ui.button("Open file").clicked() {
            if let Some(file_name) = load_input_file() {
                state.load_image_from_file_name(file_name.as_str());
            }
        }
        let drops = Self::handle_dropped_files(ui.ctx());
        if !drops.is_empty() {
            state.load_image_from_file_name(drops[0].as_str());
        }
    }
    
    /// function renders overlay for file dropping
    fn handle_dropped_files(context: &egui::Context) -> Vec<String> {
        use egui::*;
            
        // Preview hovering files:
        if !context.input().raw.hovered_files.is_empty() {
            let mut text = "Dropping files:\n".to_owned();
            
            for file in &context.input().raw.hovered_files {
                if let Some(path) = &file.path {
                    text += &format!("\n{}", path.display());
                } else if !file.mime.is_empty() {
                    text += &format!("\n{}", file.mime);
                } else {
                    text += "\n???";
                }
            }
            
            let screen_rect = context.input().screen_rect();
            let new_layer = LayerId::new(Order::Foreground, Id::new("file_drop_target"));
            let painter = context.layer_painter(new_layer);
            
            painter.rect_filled(screen_rect,0.0, Color32::from_black_alpha(192));
            painter.text(
                screen_rect.center(),
                Align2::CENTER_CENTER,
                text,
                TextStyle::Heading.resolve(&context.style()),
                Color32::WHITE,
            );
        }

        // Collect dropped files:
        if !context.input().raw.dropped_files.is_empty() {
            context.input().raw.dropped_files
                .iter()
                .filter_map(|f| match &f.path {
                    Some(path) => Some(path.display().to_string()),
                    _ => None
                })
                .collect()
        } else {
            vec![]
        }
    }
}
