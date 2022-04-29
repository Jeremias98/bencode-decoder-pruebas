use std::{fs::{self, File}, io::Read};

pub struct FileReader;

impl FileReader {

    pub fn read(filename: &String) -> Result<Vec<u8>, String> {
        let mut f = File::open(&filename).expect("no file found");
        let metadata = fs::metadata(&filename).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");
    
        Ok(buffer)
    }

}
