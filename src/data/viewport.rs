
use egui::Vec2;

#[derive(PartialEq, Default)]
pub struct Viewport {
    pub offset: Vec2,
    pub size: Vec2,
    pub zoom_level: f32,
}

impl Viewport {
    pub const ZERO: Self = Self {
        offset: Vec2::ZERO,
        size: Vec2::ZERO,
        zoom_level: 0.0,
    };
    
    pub fn new() -> Self {
        Self::ZERO
    }
    
    pub fn sized(mut self, size: impl Into<Vec2>) -> Self {
        self.size = size.into();
        self
    }
}