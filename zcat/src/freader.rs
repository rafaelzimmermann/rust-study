use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str;

pub struct BufferedFileReader<const C: usize> {
    file_inst: File,
    buffer: [u8; C]
}

impl<const C: usize> BufferedFileReader<C> {
    pub fn new(file_path: &Path) -> Result<Self, Box<dyn Error>> {
        let f = File::open(file_path);
        return match f {
            Ok(_) => Ok(Self { 
                buffer: [0; C],
                file_inst: f.unwrap(),
            }),
            Err(e) =>  Err(Box::new(e)),
        }     
    }

    fn format_buffer(&mut self, size: usize) -> Option<String> {
        return match str::from_utf8(&self.buffer[0..size]) {
            Ok(v) => {
                Some(v.to_string())
            },
            Err(_) => None,
        };
    }
}

impl<const C: usize> Iterator for BufferedFileReader<C> {
    type Item = String;

    fn next(&mut self) -> Option<String>{
        return match self.file_inst.read(&mut self.buffer[..]) {
            Ok(s) => {
                if s <= 0 {
                    return None;
                }
                return self.format_buffer(s);
            },
            Err(_) => None,
        }
    }
}