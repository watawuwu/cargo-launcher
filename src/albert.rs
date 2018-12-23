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
}

impl<'a> LauncherLike for Albert<'a> {
    fn install(&self) -> Result<()> {
        if cfg!(not(target_os = "linux")) {
            bail!("Albert supported only linux")
        }
        let workflow_path = make(self.cargo_config, self.launcher_config)?;
        copy(self.cargo_config, workflow_path)?;
        Ok(())
    }
}

pub fn install(cargo_conf: &CargoConfig, launcher_conf: &LauncherConfig) -> Result<()> {
    let workflow_path = make(cargo_conf, launcher_conf)?;
    copy(cargo_conf, workflow_path)?;
    Ok(())
}

fn make(cargo_conf: &CargoConfig, launcher_conf: &LauncherConfig) -> Result<Vec<PathBuf>> {
    let module = module_path(&launcher_conf.work_dir)?;
    write_file(&module, module_bin(cargo_conf)?.as_bytes())?;

    let icon = icon_path(&launcher_conf.work_dir)?;
    write_file(&icon, &launcher_conf.icon(cargo_conf)?[..])?;

    Ok(vec![module, icon])
}

fn module_bin(config: &CargoConfig) -> Result<String> {
    let mut params = Param::new();
    params.insert("prettyname", config.name());
    params.insert("version", config.version());
    params.insert("trigger", config.name());
    params.insert("author", &config.author());

    let tpl = String::from_utf8_lossy(MODULE_TEMPLATE).into_owned();
    let contents = tpl::render(&tpl, &params)?;

    Ok(contents)
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
Please check the checkbox of the python extension list and activate the setting.

Installed path: "#;
    println!("{}{}", msg, path.to_string_lossy());
}

fn application_config(cargo_conf: &CargoConfig) -> Result<PathBuf> {
    let mut path = dirs::home_dir().ok_or_else(|| err_msg("Notfound home dir"))?;
    path.push(".local/share/albert/org.albert.extension.python/modules");
    path.push(cargo_conf.name());
    Ok(path)
}

fn module_path(dir: &PathBuf) -> Result<PathBuf> {
    path(dir, "__init__.py")
}

fn icon_path(dir: &PathBuf) -> Result<PathBuf> {
    path(dir, "icon.png")
}

fn path(dir: &PathBuf, name: &str) -> Result<PathBuf> {
    let dir_s = dir.to_str().ok_or_else(|| err_msg("NotFound dir path"))?;
    Ok(PathBuf::from(format!("{}/{}", dir_s, name)))
}
