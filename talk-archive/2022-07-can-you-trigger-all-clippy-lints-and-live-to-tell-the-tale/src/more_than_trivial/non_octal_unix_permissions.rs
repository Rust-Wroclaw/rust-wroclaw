use std::fs::{File, OpenOptions};
use std::io;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

pub fn create_save_file(path: impl AsRef<Path>) -> io::Result<File> {
    let mut options = OpenOptions::new();

    options.mode(444); // This is actually 674, should probably use 0x444 instead

    options.open(path)
}
