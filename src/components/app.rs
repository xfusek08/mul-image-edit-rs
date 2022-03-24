
use std::sync::Arc;

use epi::backend::RepaintSignal;

use crate::utils::load_input_file;
use crate::widgets::{CenteredWindow, FileDropper, texts};
use crate::data::MultimediaFile;

use super::ImageEditor;

#[cfg(debug_assertions)]
use super::debug;

pub struct App {
    
    // global apps settings
    should_quit: bool,
    error_message: Option<String>,
    repaint_signal: Arc<dyn RepaintSignal>,
    
    // ui components
    editor: Option<ImageEditor>,
    
    #[cfg(debug_assertions)]
    debug_bottom_panel: debug::BottomPanel,
}

// constructors
impl App {
    pub fn new(repaint_signal: Arc<dyn RepaintSignal>) -> Self {
        Self {
            repaint_signal,
            should_quit: Default::default(),
            error_message: Default::default(),
            editor: Default::default(),
            
            #[cfg(debug_assertions)]
            debug_bottom_panel: Default::default(),
        }
    }
}

// Data manipulation
impl App {
    
    /// Get the app's should quit.
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }
    
    /// sets state to quitted
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
    
    /// tries to loaf image from file name
    pub fn load_image_from_file_name(&mut self, s : &str) {
        (self.editor, self.error_message) = match MultimediaFile::from_file(s) {
            Ok(image_file) => match ImageEditor::from_file(image_file, self.repaint_signal.clone()) {
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
            
            editor.ui(ctx, frame);
            
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
