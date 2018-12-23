use pretty_env_logger;
use std::path::PathBuf;
use structopt::clap::*;

use crate::albert::{self, Albert};
use crate::alfred::{self, Alfred};
use crate::args::Args;
use crate::cargo::CargoConfig;
use crate::error::Result;
use crate::fs::*;
use crate::hain::{self, Hain};

// @FIXME from metadata and rel2abs
const WORK_PATH: &str = "target/launcher";
const ICON_BIN: &[u8] = include_bytes!("asset/icon.png");

arg_enum! {
    #[derive(Debug)]
    pub enum Launcher {
        Alfred,
        Hain,
        Albert,
    }
}
impl Launcher {
    fn instance<'a>(
        &self,
        cargo_config: &'a CargoConfig,
        launcher_config: &'a LauncherConfig,
    ) -> Box<dyn LauncherLike + 'a> {
        match *self {
            Launcher::Alfred => Box::new(Alfred::new(cargo_config, launcher_config)),
            Launcher::Hain => Box::new(Hain::new(cargo_config, launcher_config)),
            Launcher::Albert => Box::new(Albert::new(cargo_config, launcher_config)),
        }
    }
}

pub struct LauncherConfig {
    pub work_dir: PathBuf,
    icon_bin: &'static [u8],
}

impl LauncherConfig {
    pub fn icon(&self, cargo_conf: &CargoConfig) -> Result<Vec<u8>> {
        let r = match cargo_conf.icon_path() {
            Some(ref path) => read_file(&path)?,
            None => self.icon_bin.to_vec(),
        };
        Ok(r)
    }

    fn mk_dir(&self) -> Result<()> {
        mk_dir(&self.work_dir)?;
        Ok(())
    }
}

pub trait LauncherLike {
    // TODO Result<()> => Result<String>
    fn install(&self) -> Result<()>;
}

pub fn launch(args: &Args, cargo_config: &CargoConfig) -> Result<()> {
    let launcher_config = LauncherConfig {
        work_dir: PathBuf::from(WORK_PATH),
        icon_bin: ICON_BIN,
    };
    launcher_config.mk_dir()?;

    let r = args.launcher.instance(cargo_config, &launcher_config);
    r.install()?;

    match args.launcher {
        Launcher::Alfred => alfred::install(&cargo_config, &launcher_config)?,
        Launcher::Hain => hain::install(&cargo_config, &launcher_config)?,
        Launcher::Albert => albert::install(&cargo_config, &launcher_config)?,
    }
    Ok(())
}
