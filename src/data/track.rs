
use std::{path::Path, error::Error};
use file_format::FileFormat;

pub const SUPPORTED_FORMATS : &'static [FileFormat] = &[
    FileFormat::MpegAudioLayer3,
    FileFormat::WaveformAudio,
];

pub struct Track {
    path: Path
}

impl Track {
    fn from_file<S>(filename : S) -> Result<Self, Error> {
        let path =
        
        Ok(Self {
            path: path
        })
    }
}
