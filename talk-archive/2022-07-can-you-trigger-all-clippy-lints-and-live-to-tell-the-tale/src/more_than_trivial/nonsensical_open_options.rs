use std::fs::{File, OpenOptions};
use std::io;
use std::path::Path;

pub fn open_config_at(path: impl AsRef<Path>) -> io::Result<File> {
    OpenOptions::new().read(true).truncate(true).open(path)
}
