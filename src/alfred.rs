#[cfg(target_os = "macos")]
use std::fs::File;
#[cfg(target_os = "macos")]
use std::io::Write;
#[cfg(target_os = "macos")]
use std::path::{Path, PathBuf};
#[cfg(target_os = "macos")]
use zip::write::{FileOptions, ZipWriter};

use crate::cargo::CargoConfig;
#[cfg(target_os = "macos")]
use crate::core::*;
use crate::error::Result;
use crate::launcher::LauncherConfig;
#[cfg(target_os = "macos")]
use crate::tpl::{self, Param};

#[cfg(target_os = "macos")]
const INFO_PLIST: &[u8] = include_bytes!("asset/alfred/info.plist");
#[cfg(target_os = "macos")]
const EXTENSION: &str = "alfredworkflow";

#[cfg(target_os = "macos")]
pub fn install(cargo_conf: &CargoConfig, launcher_conf: &LauncherConfig) -> Result<()> {
    let workflow_path = make(cargo_conf, launcher_conf)?;
    open(&[workflow_path.as_ref()])?;
    Ok(())
}

#[cfg(target_os = "macos")]
fn make(cargo_conf: &CargoConfig, launcher_conf: &LauncherConfig) -> Result<PathBuf> {
    let workflow_path = workflow_path(cargo_conf.name(), &launcher_conf.work_dir);
    let zip = File::create(&workflow_path)?;
    let mut writer = ZipWriter::new(zip);
    let options = FileOptions::default();

    writer.start_file("info.plist", options)?;
    let info_plist = info_plist(&cargo_conf)?;
    writer.write_all(info_plist.as_bytes())?;

    writer.start_file("icon.png", options)?;
    writer.write_all(&launcher_conf.icon(cargo_conf)?)?;

    writer.finish()?;
    Ok(workflow_path)
}

// TODO Install workflow via CUI or apple script.
#[cfg(target_os = "macos")]
fn open(paths: &[&Path]) -> Result<()> {
    let args = paths
        .iter()
        .map(|f| f.to_str().unwrap_or(""))
        .collect::<Vec<&str>>();
    let _ = command("open", Some(args))?;
    Ok(())
}

#[cfg(target_os = "macos")]
fn workflow_path(file_name: &str, dir_path: &PathBuf) -> PathBuf {
    let path = dir_path.to_str().unwrap_or("");
    PathBuf::from(format!("{}/{}.{}", path, file_name, EXTENSION))
}

#[cfg(target_os = "macos")]
fn info_plist(config: &CargoConfig) -> Result<String> {
    let mut params = Param::new();
    params.insert("name", config.name());
    params.insert("description", config.description());
    params.insert("createdby", &config.author());
    params.insert("buildid", &config.build_id());

    let tpl = String::from_utf8_lossy(INFO_PLIST).into_owned();
    let info_plist = tpl::render(&tpl, &params)?;

    Ok(info_plist)
}

#[cfg(not(target_os = "macos"))]
pub fn run(_cargo_conf: &CargoConfig, _launcher_conf: &LauncherConfig) -> Result<()> {
    failure::bail!("Alfred supported only macOS")
}
