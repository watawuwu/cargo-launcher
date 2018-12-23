use failure::*;
use log::*;
use std::fs;
use std::path::PathBuf;

use crate::cargo::CargoConfig;
use crate::error::Result;
use crate::fs::write_file;
use crate::launcher::{LauncherConfig, LauncherLike};
use crate::tpl::{self, Param};
const MODULE_TEMPLATE: &[u8] = include_bytes!("asset/albert/__init__.py");

pub struct Albert<'a> {
    cargo_config: &'a CargoConfig,
    launcher_config: &'a LauncherConfig,
}

impl<'a> Albert<'a> {
    pub fn new(cargo_config: &'a CargoConfig, launcher_config: &'a LauncherConfig) -> Albert<'a> {
        Albert {
            cargo_config,
            launcher_config,
        }
    }

    fn application_config(&self) -> Result<PathBuf> {
        let mut path = dirs::home_dir().ok_or_else(|| err_msg("Notfound home dir"))?;
        path.push(".local/share/albert/org.albert.extension.python/modules");

        path.push(self.cargo_config.name());
        Ok(path)
    }

    // todo string to bytes
    fn module_bin(&self) -> Result<String> {
        let conf = self.cargo_config;
        let mut params = Param::new();
        params.insert("prettyname", conf.name());
        params.insert("version", conf.version());
        params.insert("trigger", conf.name());
        params.insert("author", &conf.author());

        let tpl = String::from_utf8_lossy(MODULE_TEMPLATE).into_owned();
        let contents = tpl::render(&tpl, &params)?;

        Ok(contents)
    }

    fn module_path(&self) -> PathBuf {
        let mut buf = self.launcher_config.work_dir.clone();
        buf.push("__init__.py");
        buf
    }

    fn icon_path(&self) -> PathBuf {
        let mut buf = self.launcher_config.work_dir.clone();
        buf.push("icon.png");
        buf
    }

    fn icon(&self) -> Result<Vec<u8>> {
        self.launcher_config.icon(self.cargo_config)
    }
}

impl<'a> LauncherLike for Albert<'a> {
    fn before_check(&self) -> Result<()> {
        if cfg!(not(target_os = "linux")) {
            bail!("Albert supported only linux")
        }
        Ok(())
    }

    fn gen(&self) -> Result<Vec<PathBuf>> {
        let module = self.module_path();
        write_file(&module, self.module_bin()?.as_bytes())?;

        let icon = self.icon_path();
        write_file(&icon, &self.icon()?[..])?;

        Ok(vec![module, icon])
    }

    fn deploy(&self, paths: Vec<PathBuf>) -> Result<()> {
        let sink_dir = self.application_config()?;
        fs::create_dir_all(&sink_dir)?;
        for path in paths {
            debug!("path: {:?}", &path);
            debug!("sink: {:?}", &sink_dir);
            let name = path.file_name().ok_or_else(|| err_msg("Not file type"))?;
            let mut sink = sink_dir.clone();
            sink.push(name);
            fs::copy(&path, sink)?;
        }
        Ok(())
    }

    fn show_help(&self) -> Result<()> {
        let msg = r#"
Install completed!!
Please check the checkbox of the python extension list and activate the setting.

Installed path: "#;

        let path = self.application_config()?;
        println!("{}{}", msg, path.to_string_lossy());
        Ok(())
    }
}
