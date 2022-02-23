
use super::Track;

pub struct TrackSegment {
    pub title: String,
    pub start_sample: u64,
    pub length: u64,
    pub start_offset: u64,
    pub end_offset: u64,
}

pub struct TrackAnalyzer {
    pub track: Track,
    pub segments: Vec<TrackSegment>,
    running: bool,
}

impl TrackAnalyzer {
    pub fn new(track : Track) -> Self {
        Self {
            track: track,
            segments: vec![],
            running: false,
        }
    }
    
    pub fn get_track(&self) -> &Track {
        &self.track
    }
    
    /// Starts the analysis if all conditions are met
    pub fn start(&mut self) {
        self.running = true;
        self.segments.clear();
        
        for i in 0..50 {
            self.segments.push(TrackSegment {
                title: format!("Segment {}", i),
                start_sample: i * 10,
                length: 10,
                start_offset: 0,
                end_offset: 0,
            });
        }
        
        self.running = false;
    }
}
