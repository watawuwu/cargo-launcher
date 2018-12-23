use failure::bail;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use zip::write::{FileOptions, ZipWriter};

use crate::cargo::CargoConfig;
use crate::core::*;
use crate::error::Result;
use crate::launcher::{LauncherConfig, LauncherLike};
use crate::tpl::{self, Param};

const INFO_PLIST: &[u8] = include_bytes!("asset/alfred/info.plist");
const EXTENSION: &str = "alfredworkflow";

pub struct Alfred<'a> {
    cargo_config: &'a CargoConfig,
    launcher_config: &'a LauncherConfig,
}

impl<'a> Alfred<'a> {
    pub fn new(cargo_config: &'a CargoConfig, launcher_config: &'a LauncherConfig) -> Alfred<'a> {
        Alfred {
            cargo_config,
            launcher_config,
        }
    }

    fn workflow_path(&self) -> PathBuf {
        let name = self.cargo_config.name();
        let dir = &self.launcher_config.work_dir;
        let path = dir.to_str().unwrap_or("");
        PathBuf::from(format!("{}/{}.{}", path, name, EXTENSION))
    }

    fn info_plist(&self) -> Result<String> {
        let conf = self.cargo_config;
        let mut params = Param::new();
        params.insert("name", conf.name());
        params.insert("description", conf.description());
        params.insert("createdby", &conf.author());
        params.insert("buildid", &conf.build_id());

        let tpl = String::from_utf8_lossy(INFO_PLIST).into_owned();
        let info_plist = tpl::render(&tpl, &params)?;

        Ok(info_plist)
    }

    fn icon(&self) -> Result<Vec<u8>> {
        self.launcher_config.icon(self.cargo_config)
    }
}

impl<'a> LauncherLike for Alfred<'a> {
    fn before_check(&self) -> Result<()> {
        if cfg!(not(target_os = "macos")) {
            bail!("Alfred supported only macOS")
        }
        Ok(())
    }

    fn gen(&self) -> Result<Vec<PathBuf>> {
        let workflow_path = self.workflow_path();
        let zip = File::create(&workflow_path)?;
        let mut writer = ZipWriter::new(zip);
        let options = FileOptions::default();

        writer.start_file("info.plist", options)?;
        let info_plist = self.info_plist()?;
        writer.write_all(info_plist.as_bytes())?;

        writer.start_file("icon.png", options)?;
        writer.write_all(&self.icon()?)?;

        writer.finish()?;
        Ok(vec![workflow_path])
    }

    fn deploy(&self, paths: Vec<PathBuf>) -> Result<()> {
        let args = paths
            .iter()
            .map(|f| f.to_str().unwrap_or(""))
            .collect::<Vec<&str>>();
        let _ = command("open", Some(args))?;
        Ok(())
    }

    fn show_help(&self) -> Result<()> {
        let msg = r#"
Install completed!!

Note:
Powerpack is necessary to use Alfred workflow.
"#;
        println!("{}", msg);
        Ok(())
    }
}
