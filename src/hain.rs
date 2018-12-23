use dirs;
use failure::*;
use log::*;
use std::fs;
use std::path::PathBuf;

use crate::cargo::CargoConfig;
use crate::error::Result;
use crate::fs::write_file;
use crate::launcher::{LauncherConfig, LauncherLike};
use crate::tpl::{self, Param};

const INDEX_JS_BIN: &[u8] = include_bytes!("asset/hain/index.js");
const PACKAGE_JSON_BIN: &[u8] = include_bytes!("asset/hain/package.json");

pub struct Hain<'a> {
    cargo_config: &'a CargoConfig,
    launcher_config: &'a LauncherConfig,
}

impl<'a> Hain<'a> {
    pub fn new(cargo_config: &'a CargoConfig, launcher_config: &'a LauncherConfig) -> Hain<'a> {
        Hain {
            cargo_config,
            launcher_config,
        }
    }

    #[cfg(target_os = "macos")]
    fn application_config(&self) -> Result<PathBuf> {
        let mut path = dirs::home_dir().ok_or_else(|| err_msg("Notfound home dir"))?;
        path.push("Library");
        path.push("Application Support");
        path.push("hain-user");
        path.push("devplugins");
        path.push(self.plugin_name());
        Ok(path)
    }

    #[cfg(target_os = "linux")]
    fn application_config(&self) -> Result<PathBuf> {
        let mut path = dirs::config_dir().ok_or_else(|| err_msg("Notfound home dir"))?;
        if let Some(home) = std::env::var_os("XDG_CONFIG_HOME") {
            path = PathBuf::from(home);
        };
        path.push("hain-user/devplugins");
        path.push(self.plugin_name());
        Ok(path)
    }

    #[cfg(target_os = "windows")]
    fn application_config(&self) -> Result<PathBuf> {
        let local = std::env::var_os("LOCALAPPDATA");
        let user = std::env::var_os("USERPROFILE");

        let mut path = match (local, user) {
            (Some(l), _) => PathBuf::from(l),
            (None, Some(u)) => PathBuf::from(format!(
                "{}/Local Settings/Application Data",
                u.to_str().unwrap()
            )),
            _ => bail!("Notfound home dir"),
        };
        path.push("hain-user/devplugins");
        path.push(self.plugin_name());
        Ok(path)
    }

    fn index_js_path(&self) -> PathBuf {
        let mut buf = self.launcher_config.work_dir.clone();
        buf.push("index.js");
        buf
    }

    fn package_json_path(&self) -> PathBuf {
        let mut buf = self.launcher_config.work_dir.clone();
        buf.push("package.json");
        buf
    }

    fn icon_path(&self) -> PathBuf {
        let mut buf = self.launcher_config.work_dir.clone();
        buf.push("icon.png");
        buf
    }

    fn index_js(&self) -> Result<String> {
        let mut params = Param::new();
        params.insert("name", self.cargo_config.name());
        let tpl = String::from_utf8_lossy(INDEX_JS_BIN).into_owned();
        let contents = tpl::render(&tpl, &params)?;

        Ok(contents)
    }

    fn package_json(&self) -> Result<String> {
        let conf = self.cargo_config;
        let mut params = Param::new();
        params.insert("name", conf.name());
        params.insert("version", conf.version());
        params.insert("description", conf.description());
        params.insert("author", &conf.author());

        let tpl = String::from_utf8_lossy(PACKAGE_JSON_BIN).into_owned();
        let contents = tpl::render(&tpl, &params)?;

        Ok(contents)
    }

    fn plugin_name(&self) -> String {
        format!("hain-plugin-{}", self.cargo_config.name())
    }

    fn icon(&self) -> Result<Vec<u8>> {
        self.launcher_config.icon(self.cargo_config)
    }
}

impl<'a> LauncherLike for Hain<'a> {
    fn before_check(&self) -> Result<()> {
        Ok(())
    }

    fn gen(&self) -> Result<Vec<PathBuf>> {
        let index = self.index_js_path();
        write_file(&index, self.index_js()?.as_bytes())?;

        let package = self.package_json_path();
        write_file(&package, self.package_json()?.as_bytes())?;

        let icon = self.icon_path();
        write_file(&icon, &self.icon()?[..])?;

        Ok(vec![index, package, icon])
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
Restart of the hain is required.

Installed path: "#;
        let path = self.application_config()?;
        println!("{}{}", msg, path.to_string_lossy());
        Ok(())
    }
}
