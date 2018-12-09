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

pub fn config() -> Result<CargoConfig> {
    let args = vec!["read-manifest"];
    let output = cargo_exec(args)?;
    let config: CargoConfig = serde_json::from_str(output.as_str())?;
    Ok(config)
}
