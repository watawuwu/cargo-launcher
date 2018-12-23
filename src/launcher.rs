use pretty_env_logger;
use std::path::PathBuf;
use structopt::clap::*;

use crate::albert::Albert;
use crate::alfred::Alfred;
use crate::args::Args;
use crate::cargo::CargoConfig;
use crate::error::Result;
use crate::fs::*;
use crate::hain::Hain;

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
    fn install(&self) -> Result<String> {
        self.before_check()?;
        let artifacts = self.gen()?;
        self.deploy(artifacts)?;
        self.completed_message()
    }
    fn before_check(&self) -> Result<()>;
    fn gen(&self) -> Result<Vec<PathBuf>>;
    fn deploy(&self, paths: Vec<PathBuf>) -> Result<()>;
    fn completed_message(&self) -> Result<String>;
}

pub fn launch(args: &Args, cargo_config: &CargoConfig) -> Result<String> {
    let launcher_config = LauncherConfig {
        work_dir: PathBuf::from(WORK_PATH),
        icon_bin: ICON_BIN,
    };
    launcher_config.mk_dir()?;

    let launcher = args.launcher.instance(cargo_config, &launcher_config);
    Ok(launcher.install()?)
}
