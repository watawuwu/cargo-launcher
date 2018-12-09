use pretty_env_logger;
use std::path::PathBuf;
use structopt::clap::*;

use crate::alfred;
use crate::args::Args;
use crate::cargo::CargoConfig;
use crate::error::Result;
use crate::file::*;
use crate::hain;
use crate::io;

// @FIXME from metadata and rel2abs
const WORK_PATH: &str = "target/launcher";
const ICON_BIN: &[u8] = include_bytes!("asset/icon.png");

arg_enum! {
    #[derive(Debug)]
    pub enum Launcher {
        Alfred,
        Hain,
    }
}

pub struct LauncherConfig {
    pub work_dir: PathBuf,
    pub icon_bin: &'static [u8],
}

impl LauncherConfig {
    pub fn icon(&self, cargo_conf: &CargoConfig) -> Result<Vec<u8>> {
        let r = match cargo_conf.icon_path() {
            Some(ref path) => io::read_file(&path)?,
            None => self.icon_bin.to_vec(),
        };
        Ok(r)
    }
}

pub fn launch(args: &Args, cargo_config: &CargoConfig) -> Result<()> {
    let launcher_config = LauncherConfig {
        work_dir: PathBuf::from(WORK_PATH),
        icon_bin: ICON_BIN,
    };
    mk_dir(&launcher_config.work_dir)?;
    match args.launcher {
        Launcher::Alfred => alfred::run(&cargo_config, &launcher_config)?,
        Launcher::Hain => hain::run(&cargo_config, &launcher_config)?,
    }
    Ok(())
}
