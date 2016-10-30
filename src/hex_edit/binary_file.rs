use std::fs::File;
use std::io;
use std::io::prelude::*;

pub struct BinaryFile {
    _data: Vec<u8>,
}

#[derive(Debug)]
pub enum BinaryFileErrorCode {
    CouldNotOpenFile { reason: io::Error },
    CouldNotReadFile { reason: io::Error }
}

impl BinaryFile {
    pub fn open(file_path: &String) -> Result<BinaryFile, BinaryFileErrorCode> {
        match BinaryFile::load_file(&file_path) {
            Ok(x) => Ok(BinaryFile {_data: x}),
            Err(x) => Err(x)
        } 
    }

    fn load_file(file_path: &String) -> Result<Vec<u8>, BinaryFileErrorCode> {
        let mut bytes: Vec<u8> = Vec::new();
        
        let mut file: File = match File::open(file_path) {
            Ok(x) => x,
            Err(reason) => { return Err(BinaryFileErrorCode::CouldNotOpenFile {reason: reason}) }
        };
        
        match file.read_to_end(&mut bytes) {
            Ok(_) => {},
            Err(reason) => { return Err(BinaryFileErrorCode::CouldNotReadFile {reason: reason}) }
        };

        Ok(bytes)
    }

    pub fn length(&self) -> u64 {
        self._data.len() as u64
    }

    pub fn as_slice(&self) -> &[u8] {
        self._data.as_slice()
    } 

}