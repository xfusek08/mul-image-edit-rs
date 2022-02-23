
use std::{path::{Path, PathBuf}, ffi::OsStr, fmt::{Display, self}, error::Error};
use file_format::FileFormat;
use std::fs::File;

pub const SUPPORTED_FORMATS : &'static [FileFormat] = &[
    FileFormat::MpegAudioLayer3,
    FileFormat::WaveformAudio,
];

pub struct Track {
    path: PathBuf,
    format: FileFormat,
    file: File,
}

impl Track {
    pub fn from_file<S>(filename : &S) -> Result<Track, TrackLoadingError>
        where S: AsRef<OsStr> + ?Sized
    {
        let path = Path::new(filename);
        let path_buf = PathBuf::from(path);
        
        let format = FileFormat::from_file(path);
        if let Err(why) = format {
            return Err(TrackLoadingError::FileError { path: path_buf, why: Box::new(why) });
        }
        
        let format = format.unwrap();
        if !SUPPORTED_FORMATS.contains(&format) {
            return Err(TrackLoadingError::UnsupportedFormat { path: path_buf, format: format });
        }
        
        Ok(Track {
            path: PathBuf::from(path),
            file: File::open(path).unwrap(),
            format: format
        })
    }
    
    pub fn file_name(&self) -> &str {
        match self.path.as_path().file_name() {
            Some(name) => name.to_str().unwrap_or("File name has invalid encoding"), // TODO: parse os_str into sanitized utf-8 string with byte encoder.
            None => "Undefine",
        }
    }
}

pub enum TrackLoadingError {
    FileError {
        path: PathBuf,
        why: Box<dyn Error>,
    },
    UnsupportedFormat {
        path: PathBuf,
        format: FileFormat,
    },
}

impl Display for TrackLoadingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TrackLoadingError::FileError { path , why } =>
                write!(f, "Couldn't open {}: {why}", path.display()),
                
            TrackLoadingError::UnsupportedFormat { path, format } =>
                write!(f, "File \"{}\" has unsupported format of \"{format}\":", path.display()),
        }
    }
}
