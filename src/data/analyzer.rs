
use super::Track;

pub struct TrackAnalyzer {
    pub track: Track,
}

impl TrackAnalyzer {
    pub fn new(track : Track) -> Self {
        Self { track: track, }
    }
}
