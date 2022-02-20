/// Inspired by: https://github.com/emilk/egui/blob/master/eframe/examples/file_dialog.rs

use eframe::egui;


pub fn handle_dropped_files(ctx: &egui::CtxRef) -> Vec<String> {
    use egui::*;
        
    // Preview hovering files:
    if !ctx.input().raw.hovered_files.is_empty() {
        let mut text = "Dropping files:\n".to_owned();
        
        for file in &ctx.input().raw.hovered_files {
            if let Some(path) = &file.path {
                text += &format!("\n{}", path.display());
            } else if !file.mime.is_empty() {
                text += &format!("\n{}", file.mime);
            } else {
                text += "\n???";
            }
        }
        
        let painter = ctx.layer_painter(
            LayerId::new(Order::Foreground, Id::new("file_drop_target"))
        );
        
        painter.rect_filled(
            ctx.input().screen_rect(),
            0.0,
            Color32::from_black_alpha(192)
        );
        
        painter.text(
            ctx.input().screen_rect().center(),
            Align2::CENTER_CENTER,
            text,
            TextStyle::Heading,
            Color32::WHITE,
        );
    }

    // Collect dropped files:
    if !ctx.input().raw.dropped_files.is_empty() {
        ctx.input().raw.dropped_files
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
