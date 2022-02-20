
pub fn load_input_files() -> Iterator<Item=&str> {
    
    // Open dialog to select files
    let res = rfd::FileDialog::new()
        .add_filter("Audio files", crate::data::)
        .pick_files();
        
    res
        .unwrap()
        .iter()
        .filter_map(|p| p.as_path().as_os_str().to_str())
}
