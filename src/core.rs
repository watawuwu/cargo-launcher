use crate::error::Result;
#[cfg(target_os = "macos")]
use std::collections::hash_map::DefaultHasher;
use std::ffi::OsStr;
#[cfg(target_os = "macos")]
use std::hash::Hasher;
use std::process::Command;

pub fn command<P: AsRef<OsStr>>(program: P, maybe_args: Option<Vec<P>>) -> Result<String> {
    let args = maybe_args.unwrap_or_else(|| vec![]);
    let output = Command::new(program).args(args).output()?;

    let result = String::from_utf8_lossy(&output.stdout).into_owned();
    Ok(result)
}

#[cfg(target_os = "macos")]
pub fn hash(input: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    hasher.write(input.as_bytes());
    hasher.finish()
}
