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

#[cfg(test)]
mod tests {

    use crate::core::*;

    #[test]
    #[cfg(target_os = "macos")]
    fn hash_bore_ok() {
        let args = "test";
        let expected = 16183295663280961421u64;
        let actual = hash(args);

        assert_eq!(expected, actual);
    }

    #[test]
    fn command_echo_ok() {
        let cmd = "echo";
        let args = vec!["-n", "test"];
        let expected = "test";
        let actual = command(cmd, Some(args)).unwrap();

        assert_eq!(expected, actual.as_str());
    }

    #[test]
    fn command_none_args_ok() {
        let cmd = "echo";
        let actual = command(cmd, None);
        assert!(actual.is_ok());
    }

    #[test]
    fn command_invalid_cmd_ng() {
        let cmd = "ls";
        let actual = command(cmd, None);
        assert!(actual.is_ok());
    }
}
