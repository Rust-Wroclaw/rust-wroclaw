use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

pub fn save_state(state: &[u8], path: impl AsRef<Path>) -> io::Result<()> {
    let mut file = File::create(path)?;

    file.write(state)?;

    Ok(())
}
