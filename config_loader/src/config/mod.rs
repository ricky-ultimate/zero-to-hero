use std::io;
use std::fs;
use std::path::Path;

pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String, io::Error> {
    fs::read_to_string(path)
}
