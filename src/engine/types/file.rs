use std::fs::File as StdFile;
use std::{fs, io::{Write, Read}};
use bincode::{
    enc::write::Writer, de::read::Reader,
    error::{DecodeError, EncodeError}
};
pub struct File {
    f: StdFile
}

impl File {
    pub fn open(path: &str, append: bool) -> Result<Self, std::io::Error> {
        match  fs::OpenOptions::new().append(append).create(true).write(true).read(true).open(path) {
            Ok(f) => Ok(Self{f: f}),
            Err(e) => Err(e),
        }
    }
}

impl Writer for File {
    fn write(&mut self, bytes: &[u8]) -> Result<(), bincode::error::EncodeError> {
        match self.f.write(bytes) {
            Ok(_) => Ok(()),
            Err(e) => Err(EncodeError::Other("Failed to write bytes to file")),
        }
    }
}

impl Reader for File {
    fn read(&mut self, bytes: &mut [u8]) -> Result<(), bincode::error::DecodeError> {
        match self.f.read(bytes) {
            Ok(_) => Ok(()),
            Err(e) => Err(DecodeError::Io{ inner: e, additional: 0}),
        }
    }
}