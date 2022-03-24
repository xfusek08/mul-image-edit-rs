
use std::thread::JoinHandle;
use std::sync::{Mutex, Arc};

use egui::Vec2;
use image::{DynamicImage, imageops::FilterType};

use crate::utils::with_data;

/// Sliders to adjust image properties
/// values are always offsets from original ( -x <-- 0 --> +x )
#[derive(Default, Clone, Debug)]
pub struct ImageSettings {
    pub contrast: f32,
    pub exposure: f32,
    pub blur: f32,
}

impl ImageSettings {
    
    /// Get the image settings's contrast.
    pub fn contrast(&self) -> f32 {
        self.contrast
    }

    /// Get the image settings's exposure.
    pub fn exposure(&self) -> f32 {
        self.exposure
    }

    /// Get the image settings's blur.
    pub fn blur(&self) -> f32 {
        self.blur
    }
    
}

pub struct EditedImage {
    original: Arc<DynamicImage>, // this is resource shared between this struct and computational thread
    
    // copy of the image scaled to fit into the viewport
    preview_original: Option<DynamicImage>,
    preview_working: Option<DynamicImage>,
    
    /// Current image modifications
    settings: ImageSettings,
    
    resize_job: ResizeJob,
}

// factories
impl EditedImage {
    pub fn from_bytes(image_bytes: &[u8]) -> Result<Self, String> {
        let original = image::load_from_memory(image_bytes).map_err(|err| err.to_string())?;
        
        Ok(Self {
            original: Arc::new(original),
            preview_original: None,
            preview_working: None,
            settings: Default::default(),
            resize_job: Default::default(),
        })
    }
}

// mutable methods
impl EditedImage {
    
    /// change preview copies of image to fit into viewport
    pub fn resize<F>(&mut self, target_size: &Vec2, on_finished: F) -> bool
    where
        F: FnOnce() + Send + 'static
    {
        self.resize_job.start(self.original.clone(), target_size.clone(), on_finished)
    }
    
    pub fn update_check(&mut self) -> bool {
        match self.resize_job.collect() {
            Some((o, p)) => {
                self.preview_original = Some(o);
                self.preview_working = Some(p);
                true
            },
            None => false,
        }
    }
    
    pub fn update_settings(&mut self, new_settings: ImageSettings) {
        self.settings = new_settings;
        dbg!("New settings: {}", &self.settings);
        
        if let Some(p_orig) = &self.preview_original {
            self.preview_working = Some(p_orig.brighten((self.settings.exposure * 255.0) as i32));
        }
    }
}

// immutable methods
impl EditedImage {
    
    pub fn original(&self) -> &DynamicImage {
        &self.original
    }
    
    /// make sure that something is returned
    pub fn preview_working(&self) -> &DynamicImage {
        match &self.preview_working {
            Some(image) => image,
            None => self.preview_original(),
        }
    }
    
    pub fn preview_original(&self) -> &DynamicImage {
        self.preview_original
            .as_ref()
            .unwrap_or(&self.original)
    }
    
    pub fn is_resizing(&self) -> bool {
        self.resize_job.state() == ResizeJobState::Running
    }
    
    /// Get a reference to the edited image's settings.
    pub fn settings(&self) -> &ImageSettings {
        &self.settings
    }
    
}

// extend interface for DynamicImage to comply with needs of this implementation
pub trait EditedImageComponent {
    fn size_vec2(&self) -> Vec2;
    fn raw_size(&self) -> u64;
}

impl EditedImageComponent for DynamicImage {
    fn size_vec2(&self) -> Vec2 {
        [self.width() as f32, self.height() as f32].into()
    }
    
    fn raw_size(&self) -> u64 {
        self.as_bytes().len() as _
    }
}

#[derive(PartialEq, Clone)]
enum ResizeJobState {
    NotRunning,
    Running,
    WaitingToCollect
}

struct ResizeJobData {
    result: Option<(DynamicImage, DynamicImage)>,
    state: ResizeJobState,
}

impl Default for ResizeJobData {
    fn default() -> Self {
        Self {
            state: ResizeJobState::NotRunning,
            result: Default::default(),
        }
    }
}

struct ResizeJob {
    data: Arc<Mutex<ResizeJobData>>,
    join_handle: Option<JoinHandle<()>>,
}

impl Default for ResizeJob {
    fn default() -> Self {
        Self {
            data: Arc::new(Mutex::new(ResizeJobData::default())),
            join_handle: None,
        }
    }
}

impl Drop for ResizeJob {
    fn drop(&mut self) {
        self.join_handle();
    }
}

impl ResizeJob {
    
    pub fn state(&self) -> ResizeJobState {
        self.data.lock().unwrap().state.clone()
    }
    
    
    /// inspired by: https://stackoverflow.com/a/59072336
    pub fn with_data<R>(&mut self, func: impl FnOnce(&mut ResizeJobData) -> R) -> Result<R, String> {
        with_data(self.data.clone(), func)
    }
    
    
    pub fn collect(&mut self) -> Option<(DynamicImage, DynamicImage)> {
        match self.state() {
            ResizeJobState::WaitingToCollect => {
                self.join_handle();
                self.with_data(|data| {
                    let res = data.result.take();
                    data.state = ResizeJobState::NotRunning;
                    res
                }).unwrap() // NOTE: here it can panic!
            },
            _ => None,
        }
    }
    
    
    pub fn start<F>(&mut self, original: Arc<DynamicImage>, target_size: Vec2, on_finished: F) -> bool
    where
        F: FnOnce() + Send + 'static
    {
        if self.state() == ResizeJobState::Running {
            return false;
        }
        
        if self.state() == ResizeJobState::WaitingToCollect {
            self.join_handle();
        }
        
        dbg!(format!("Starting job -> {} x {}", target_size.x, target_size.y));
        
        let data = self.data.clone();
        
        self.join_handle = Some(std::thread::spawn(move || {
            let preview_original = original.resize(
                target_size.x as u32,
                target_size.y as u32,
                FilterType::CatmullRom
            );
            let preview_working = preview_original.clone();
            
            with_data(data, |data| {
                data.result = Some((preview_original, preview_working));
                data.state = ResizeJobState::WaitingToCollect;
            }).unwrap(); // NOTE: can panic!
            
            on_finished();
            dbg!("Job finished");
        }));
        
        self.with_data(|data| {
            data.state = ResizeJobState::Running;
        }).unwrap(); // NOTE: can panic!
        
        true
    }
    
    
    fn join_handle(&mut self) {
        if let Some(handle) = self.join_handle.take() {
            handle.join().expect("Could not join handle");
        }
    }
}
