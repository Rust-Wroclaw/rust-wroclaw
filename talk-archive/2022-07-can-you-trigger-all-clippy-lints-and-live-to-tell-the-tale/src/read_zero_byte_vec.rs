use std::io::{self, Read};

pub fn read_data<F: Read>(mut f: F) -> io::Result<Vec<u8>> {
    let mut buffer = Vec::with_capacity(1024);

    f.read_to_end(&mut buffer)?;

    Ok(buffer)
}
