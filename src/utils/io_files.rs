
use std::path::{PathBuf, Path};

use crate::{constants, data::MultimediaFile};

fn get_supported_extensions() -> Vec<&'static str> {
    constants::SUPPORTED_MULTIMEDIA_FILE_FORMATS
        .iter()
        .map(|f| f.extension())
        .collect()
}

fn common_dialog() -> rfd::FileDialog {
    let extensions = get_supported_extensions();
    let f = rfd::FileDialog::new();
    if !extensions.is_empty() {
        f.add_filter(format!("Images (*.{})", extensions.join(", *.")).as_str(), &extensions)
    } else {
        f
    }
        
}

pub fn load_input_file() -> Option<String> {
    common_dialog()
        .pick_files()?
        .iter()
        .filter_map(|p| Some(format!("{}", p.as_path().display())))
        .next()
}

pub fn save_output_file(original_file: Option<&MultimediaFile>) -> Option<PathBuf> {
    let mut d = common_dialog();
    if let Some(f) = original_file {
        d = d.set_file_name(f.file_name());
        if let Some(dir) = f.get_dir() {
            d = d.set_directory(dir);
        }
    }
    
    d.save_file()
}
