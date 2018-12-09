use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use zip::write::{FileOptions, ZipWriter};

use crate::cargo::CargoConfig;
use crate::core::*;
use crate::error::Result;
use crate::launcher::LauncherConfig;
use crate::tpl::{self, Param};

const INFO_PLIST: &[u8] = include_bytes!("asset/alfred/info.plist");
const EXTENSION: &str = "alfredworkflow";

#[cfg(target_os = "macos")]
pub fn run(cargo_conf: &CargoConfig, launcher_conf: &LauncherConfig) -> Result<()> {
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

    install(&[workflow_path.as_ref()])?;
    Ok(())
}

#[cfg(not(target_os = "macos"))]
pub fn run(config: &CargoConfig, work_path: &PathBuf) -> Result<()> {
    bail!("Alfred supported only macOS")
}

// TODO Install workflow via CUI or apple script.
fn install(paths: &[&Path]) -> Result<()> {
    let args = paths
        .iter()
        .map(|f| f.to_str().unwrap_or(""))
        .collect::<Vec<&str>>();
    let _ = command("open", Some(args))?;
    Ok(())
}

fn workflow_path(file_name: &str, dir_path: &PathBuf) -> PathBuf {
    let path = dir_path.to_str().unwrap_or("");
    PathBuf::from(format!("{}/{}.{}", path, file_name, EXTENSION))
}

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
