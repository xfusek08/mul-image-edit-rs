
use std::{path::{Path, PathBuf}, ffi::OsStr};
use file_format::FileFormat;
use std::fs::File;

pub const SUPPORTED_FORMATS : &'static [FileFormat] = &[
    FileFormat::MpegAudioLayer3,
    FileFormat::WaveformAudio,
];

pub enum Track {
    Valid {
        path: PathBuf,
        format: FileFormat,
        file: File,
    },
    Invalid {
        path: PathBuf,
        message: String
    }
}

impl Track {
    pub fn from_file<S>(filename : &S) -> Track
        where S: AsRef<OsStr> + ?Sized
    {
        let path = Path::new(filename);
        let path_buf = PathBuf::from(path);
        
        let format = FileFormat::from_file(path);
        if let Err(why) = format {
            return Track::Invalid {
                path: path_buf,
                message: format!("couldn't open {}: {}", path.display(), why),
            }
        }
        
        let format = format.unwrap();
        if !SUPPORTED_FORMATS.contains(&format) {
            return Track::Invalid {
                path: path_buf,
                message: format!("File \"{}\" has unsupported format of \"{}\":", path.display(), format),
            }
        }
        
        Track::Valid {
            path: path_buf,
            file: File::open(path).unwrap(),
            format: format
        }
    }
}
