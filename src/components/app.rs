
use eframe::epi;
use eframe::egui;

pub struct App {
    loaded_files : Vec<String>,
}

impl epi::App for App {
    
    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &epi::Frame) {
        
        egui::TopBottomPanel::top("my_panel").show(ctx, |ui| {
            let open_button = ui.add(egui::widgets::Button::new("Open..."));
            if open_button.clicked() {
                self.load_input_files();
            }
        });
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(&self.loaded_files.join("\n"));
        });
        
        self.detect_files_being_dropped(ctx);
    }

    fn name(&self) -> &str {
        env!("CARGO_PKG_NAME")
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            loaded_files: vec![],
        }
    }
}

impl App {
    
    /// This function will ask user to select input files for the app to process
    fn load_input_files(&mut self) {
        
        // Open dialog to select files
        let res = rfd::FileDialog::new()
            .add_filter("Audio files", &["mp3", "wav"])
            .pick_files();
        
        self.loaded_files =
            if res.is_some() {
                res
                    .unwrap()
                    .iter()
                    .filter_map(|p| p.as_path().as_os_str().to_str())
                    .map(|s| s.to_owned())
                    .collect()
            } else {
                vec![ "No file selected".to_string() ]
            }
    }
    
    /// Function handles file drops into an app
    /// Taken from: https://github.com/emilk/egui/blob/master/eframe/examples/file_dialog.rs
    fn detect_files_being_dropped(&mut self, ctx: &egui::CtxRef) {
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
            self.loaded_files.extend(ctx
                .input()
                .raw
                .dropped_files
                .iter()
                .filter_map(|f| match &f.path {
                    Some(path) => Some(path.display().to_string()),
                    _ => None
                })
            );
        }
    }
}
