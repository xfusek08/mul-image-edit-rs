
/// function renders overlay for file dropping

pub struct FileDropper;
impl FileDropper {
    pub fn handle(context: &egui::Context) -> Vec<String> {
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
