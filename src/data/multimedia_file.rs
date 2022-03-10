
use std::{
    path::{ Path, PathBuf },
    fmt::{ Display, self },
    ffi::OsStr,
    fs::{File},
    error::Error, io::Read,
};
use file_format::FileFormat;
use indoc::indoc;

pub struct MultimediaFile {
    path: PathBuf,
    file_name: String,
    format: FileFormat,
    file: File,
}

/// static methods
impl MultimediaFile {
    
    pub fn from_file<S>(filename : &S) -> Result<MultimediaFile, MultimediaFileLoadingError>
        where S: AsRef<OsStr> + ?Sized
    {
        let path = Path::new(filename);
        let path_buf = PathBuf::from(path);
        
        let format = FileFormat::from_file(path);
        if let Err(why) = format {
            return Err(MultimediaFileLoadingError::InvalidFile { path: path_buf, why: Box::new(why) });
        }
        
        Ok(MultimediaFile {
            path: PathBuf::from(path),
            file_name: format!("{}", path.display()),
            file: File::open(path).unwrap(),
            format: format.unwrap(),
        })
    }
    
    pub fn get_path(&self) -> &Path {
        self.path.as_path()
    }
    
    pub fn file_name(&self) -> &str {
        self.file_name.as_str()
    }
    
    pub fn file_name_owned(&self) -> String {
        self.file_name.clone()
    }
    
    pub fn bytes(&mut self) -> Vec<u8> {
        let metadata = self.file.metadata().expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        self.file.read(&mut buffer).expect("buffer overflow");
        buffer
    }
}

impl Display for MultimediaFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            indoc!("
                {}:
                    format: {}
            "),
            self.get_path().display(),
            self.format
        )
    }
}

pub enum MultimediaFileLoadingError {
    InvalidFile {
        path: PathBuf,
        why: Box<dyn Error>,
    },
}

impl Display for MultimediaFileLoadingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MultimediaFileLoadingError::InvalidFile { path , why } =>
                write!(f, "Couldn't open {}: {why}", path.display()),
        }
    }
}
