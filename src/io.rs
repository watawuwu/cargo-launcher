use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

use crate::error::Result;

pub fn read_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn write_file<P: AsRef<Path>>(path: P, contents: &[u8]) -> Result<()> {
    let file = File::create(path)?;
    let mut buf = BufWriter::new(file);
    buf.write_all(contents)?;
    Ok(())
}
