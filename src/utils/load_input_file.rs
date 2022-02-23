
pub fn load_input_file() -> Option<String> {
    let extensions: Vec<&str> = crate::data::SUPPORTED_FORMATS
        .iter()
        .map(|f| f.extension())
        .collect();
    
    rfd::FileDialog::new()
        .add_filter("Audio files", &extensions)
        .pick_files()?
        .iter()
        .filter_map(|p| Some(format!("{}", p.as_path().display())))
        .next()
}
