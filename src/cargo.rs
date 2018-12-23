use serde_derive::*;
use std::path::PathBuf;

use crate::core::*;
use crate::error::Result;

#[derive(Serialize, Deserialize)]
pub struct CargoConfig {
    name: String,
    version: String,
    description: Option<String>,
    metadata: Option<Metadata>,
    authors: Option<Vec<String>>,
}

impl CargoConfig {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn version(&self) -> &str {
        self.version.as_str()
    }
    pub fn description(&self) -> &str {
        self.description.as_ref().map(|d| d.as_str()).unwrap_or("")
    }

    pub fn author(&self) -> String {
        self.authors
            .as_ref()
            .map(|f| f.join(", "))
            .unwrap_or_else(|| String::from(""))
    }

    pub fn build_id(&self) -> String {
        format!("{}-{}", self.name(), hash(self.name()))
    }

    pub fn icon_path(&self) -> Option<PathBuf> {
        if let Some(ref metadata) = self.metadata {
            metadata.launcher.icon.clone()
        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    launcher: LauncherConfig,
}

#[derive(Serialize, Deserialize)]
pub struct LauncherConfig {
    icon: Option<PathBuf>,
}

fn cargo_exec(sub: Vec<&str>) -> Result<String> {
    let r = command("cargo", Some(sub))?;
    Ok(r)
}

pub fn config(opt_path: &Option<PathBuf>) -> Result<CargoConfig> {
    let mut args = vec!["read-manifest"];

    if let Some(path) = opt_path {
        if let Some(ref s) = path.to_str() {
            args.push("--manifest-path");
            args.push(s);
        }
    }

    let output = cargo_exec(args)?;
    let config: CargoConfig = serde_json::from_str(output.as_str())?;
    Ok(config)
}

#[cfg(test)]
#[cfg(not(target_os = "windows"))]
mod tests {

    use crate::cargo::*;
    use crate::fs::write_file;
    use std::env;
    use std::fs;
    use std::path::PathBuf;
    use tempdir::TempDir;

    const DUMMY_CARGO: &str = r##"
[package]
name        = "test-cargo"
edition     = "2018"
version     = "0.1.0"
authors     = ["mozilla", "watawuwu"]
license     = "MIT"
description = "Test description"
repository  = "https://github.com/watawuwu/cargo-launcher"
readme      = "README.md"
"##;

    const DUMMY_MAIN: &str = r##"fn main() { println!("test"); }"##;

    fn create_tmp_project(tmp_dir: &TempDir, toml: &str) -> PathBuf {
        env::set_current_dir(&tmp_dir).unwrap();
        fs::create_dir("src").unwrap();
        let cargo_file = tmp_dir.path().join("Cargo.toml");
        write_file(&cargo_file, toml.as_bytes()).unwrap();
        let dummy_main = tmp_dir.path().join("src/main.rs");
        write_file(&dummy_main, DUMMY_MAIN.as_bytes()).unwrap();
        cargo_file
    }

    #[test]
    fn config_bore_ok() {
        let tmp_dir = TempDir::new("").unwrap();
        let cargo_file = create_tmp_project(&tmp_dir, DUMMY_CARGO);

        let cargo = config(&Some(cargo_file)).unwrap();

        assert_eq!(cargo.name(), "test-cargo");
        assert_eq!(cargo.version(), "0.1.0");
        assert_eq!(cargo.description(), "Test description");
        assert_eq!(cargo.author(), "mozilla, watawuwu");
        #[cfg(target_os = "macos")]
        assert_eq!(cargo.build_id(), "test-cargo-5484037434785666097");
        assert!(cargo.icon_path().is_none());
    }

    #[test]
    fn config_icon_ok() {
        let tmp_dir = TempDir::new("").unwrap();
        let dummy = format!(
            "{}{}",
            DUMMY_CARGO, "[package.metadata.launcher]\nicon=\"test.png\"\n"
        );
        let cargo_file = create_tmp_project(&tmp_dir, &dummy);
        let cargo = config(&Some(cargo_file)).unwrap();
        let icon_path = cargo.icon_path();
        assert!(icon_path.is_some());
        assert_eq!(icon_path.unwrap().to_string_lossy().as_ref(), "test.png");
    }
}
