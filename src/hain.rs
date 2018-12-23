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
}

impl<'a> LauncherLike for Hain<'a> {
    fn install(&self) -> Result<()> {
        let paths = make(self.cargo_config, self.launcher_config)?;
        copy(self.cargo_config, paths)?;
        Ok(())
    }
}

pub fn install(cargo_conf: &CargoConfig, launch_conf: &LauncherConfig) -> Result<()> {
    let paths = make(cargo_conf, launch_conf)?;
    copy(cargo_conf, paths)?;
    Ok(())
}

pub fn make(cargo_conf: &CargoConfig, launch_conf: &LauncherConfig) -> Result<Vec<PathBuf>> {
    let index = index_js_path(&launch_conf.work_dir)?;
    write_file(&index, index_js(cargo_conf)?.as_bytes())?;

    let package = package_json_path(&launch_conf.work_dir)?;
    write_file(&package, package_json(cargo_conf)?.as_bytes())?;

    let icon = icon_path(&launch_conf.work_dir)?;
    write_file(&icon, &launch_conf.icon(cargo_conf)?[..])?;

    Ok(vec![index, package, icon])
}

fn copy(conf: &CargoConfig, paths: Vec<PathBuf>) -> Result<()> {
    let sink_dir = application_config(conf)?;
    fs::create_dir_all(&sink_dir)?;
    for path in paths {
        debug!("path: {:?}", &path);
        debug!("sink: {:?}", &sink_dir);
        let name = path.file_name().ok_or_else(|| err_msg("Not file type"))?;
        let mut sink = sink_dir.clone();
        sink.push(name);
        fs::copy(&path, sink)?;
    }

    show_help(&sink_dir);
    Ok(())
}

fn show_help(path: &PathBuf) {
    let msg = r#"
Install completed!!
Restart of the hain is required.

Installed path: "#;
    println!("{}{}", msg, path.to_string_lossy());
}

// hain config dir spec
// https://github.com/LinusU/node-application-config-path/blob/master/index.js
#[cfg(target_os = "macos")]
fn application_config(cargo_conf: &CargoConfig) -> Result<PathBuf> {
    let mut path = dirs::home_dir().ok_or_else(|| err_msg("Notfound home dir"))?;
    path.push("Library");
    path.push("Application Support");
    path.push("hain-user");
    path.push("devplugins");
    path.push(plugin_name(cargo_conf.name()));
    Ok(path)
}

#[cfg(target_os = "linux")]
fn application_config(cargo_conf: &CargoConfig) -> Result<PathBuf> {
    let mut path = dirs::config_dir().ok_or_else(|| err_msg("Notfound home dir"))?;
    if let Some(home) = std::env::var_os("XDG_CONFIG_HOME") {
        path = PathBuf::from(home);
    };
    path.push("hain-user/devplugins");
    path.push(plugin_name(cargo_conf.name()));
    Ok(path)
}

#[cfg(target_os = "windows")]
fn application_config(cargo_conf: &CargoConfig) -> Result<PathBuf> {
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
    path.push(plugin_name(cargo_conf.name()));
    Ok(path)
}

fn index_js_path(dir: &PathBuf) -> Result<PathBuf> {
    path(dir, "index.js")
}

fn package_json_path(dir: &PathBuf) -> Result<PathBuf> {
    path(dir, "package.json")
}

fn icon_path(dir: &PathBuf) -> Result<PathBuf> {
    path(dir, "icon.png")
}

fn path(dir: &PathBuf, name: &str) -> Result<PathBuf> {
    let dir_s = dir.to_str().ok_or_else(|| err_msg("NotFound dir path"))?;
    Ok(PathBuf::from(format!("{}/{}", dir_s, name)))
}

fn index_js(config: &CargoConfig) -> Result<String> {
    let mut params = Param::new();
    params.insert("name", config.name());
    let tpl = String::from_utf8_lossy(INDEX_JS_BIN).into_owned();
    let contents = tpl::render(&tpl, &params)?;

    Ok(contents)
}

fn package_json(config: &CargoConfig) -> Result<String> {
    let mut params = Param::new();
    params.insert("name", config.name());
    params.insert("version", config.version());
    params.insert("description", config.description());
    params.insert("author", &config.author());

    let tpl = String::from_utf8_lossy(PACKAGE_JSON_BIN).into_owned();
    let contents = tpl::render(&tpl, &params)?;

    Ok(contents)
}

fn plugin_name(name: &str) -> String {
    format!("hain-plugin-{}", name)
}
