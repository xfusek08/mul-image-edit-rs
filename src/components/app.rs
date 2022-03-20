
use crate::utils::load_input_file;
use crate::widgets::{CenteredWindow, FileDropper};
use crate::data::MultimediaFile;
use crate::widgets::texts;

use super::ImageEditor;

#[cfg(debug_assertions)]
use super::debug;

#[derive(Default)]
pub struct App {
    
    // global apps settings
    pub should_quit: bool,
    pub error_message: Option<String>,
    
    // ui components
    pub editor: Option<ImageEditor>,
    
    #[cfg(debug_assertions)]
    pub debug_bottom_panel: debug::BottomPanel,
}

// Data manipulation
impl App {
    
    /// sets state to quitted
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
    
    /// tries to loaf image from file name
    pub fn load_image_from_file_name(&mut self, s : &str) {
        (self.editor, self.error_message) = match MultimediaFile::from_file(s) {
            Ok(image_file) => match ImageEditor::from_file(image_file) {
                Ok(state) => (Some(state), None),
                Err(message) => ( None, Some(message)),
            },
            Err(error) => (None, Some(format!("{}", error))),
        }
    }
    
}

// ui code
impl App {
    
    pub fn ui(&mut self, ctx: &egui::Context, frame: &epi::Frame) {

        // render debug bottom paned
        #[cfg(debug_assertions)]
        {
            self.debug_bottom_panel.update(ctx, frame);
            self.debug_bottom_panel.ui(ctx, frame); // separate method to be able to insert it deeper into ui hierarchy
        }
        
        if self.error_message.is_some() {
            
            // TODO: render error message as overlay
            
            // Render error of invalid file
            CenteredWindow::show(ctx, |ui| {
                // let msg = message.clone();
                ui.label(texts::big("Selected file is invalid"));
                ui.label(format!("{}", self.error_message.as_ref().unwrap()));
                self.open_file_dialog(ui);
            });
            
        } else if let Some(editor) = &mut self.editor {
            
            editor.ui(ctx);
            
        } else {
            
            // No file
            CenteredWindow::show(ctx, |ui| {
                ui.label(texts::big("Select image to edit"));
                self.open_file_dialog(ui);
            });
        }
    }
    
    fn open_file_dialog(&mut self, ui : &mut egui::Ui) {
        if ui.button("Open file").clicked() {
            if let Some(file_name) = load_input_file() {
                self.load_image_from_file_name(file_name.as_str());
            }
        }
        let drops = FileDropper::handle(ui.ctx());
        if !drops.is_empty() {
            self.load_image_from_file_name(drops[0].as_str());
        }
    }
}
