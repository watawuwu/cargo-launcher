use std::fs;
use std::path::Path;

use crate::error::Result;

pub fn mk_dir<P: AsRef<Path>>(path: P) -> Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
}
