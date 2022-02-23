
use super::Track;

pub struct TrackAnalyzer {
    track: Track,
}

impl TrackAnalyzer {
    pub fn new(track : Track) -> Self {
        Self { track: track, }
    }
    
    pub fn get_track(&self) -> &Track {
        &self.track
    }
    
    /// Starts the analysis if all conditions are met
    pub fn start(&mut self) {
        
    }
}
