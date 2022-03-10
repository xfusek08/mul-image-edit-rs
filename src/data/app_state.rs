
use super::{MultimediaFile, ImageEditorState};

#[derive(Default)]
pub struct AppState {
    
    // global apps settings
    pub should_quit: bool,
    pub pixels_per_point: Option<f32>, // TODO load this value from config file
    pub error_message: Option<String>,
    
    // media editor state
    pub editor_state: Option<ImageEditorState>,
}

impl AppState {
    
    /// sets state to quitted
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
    
    /// tries to loaf image from file name
    pub fn load_image_from_file_name(&mut self, s : &str) {
        
        (self.editor_state, self.error_message) = match MultimediaFile::from_file(s) {
            Ok(image_file) => match ImageEditorState::from_file(image_file) {
                Ok(state) => (Some(state), None),
                Err(message) => ( None, Some(message)),
            },
            Err(error) => (None, Some(format!("{}", error))),
        }
        
    }
}
