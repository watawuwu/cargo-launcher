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

pub struct LauncherConfig<'a> {
    pub work_dir: PathBuf,
    icon_path: Option<&'a PathBuf>,
}

impl<'a> LauncherConfig<'a> {
    pub fn icon(&self) -> Result<Vec<u8>> {
        let r = match self.icon_path {
            Some(path) => read_file(path)?,
            None => ICON_BIN.to_vec(),
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
        icon_path: args.icon_path.as_ref(),
    };
    launcher_config.mk_dir()?;

    let launcher = args.launcher.instance(cargo_config, &launcher_config);
    Ok(launcher.install()?)
}

#[cfg(test)]
mod tests {

    use crate::fs::write_file;
    use crate::launcher::*;
    use std::fs;
    use std::path::PathBuf;
    use tempdir::TempDir;

    #[test]
    fn mk_dir_bore_ok() {
        let tmp_dir = TempDir::new("mk_dir_bore_ok").unwrap();
        let dir = tmp_dir.path().join("work_dir");
        let conf = LauncherConfig {
            work_dir: dir.clone(),
            icon_path: None,
        };

        let r = conf.mk_dir();
        assert!(r.is_ok());
        assert!(dir.exists());
    }

    #[test]
    fn icon_none_ok() {
        let tmp_dir = TempDir::new("icon_none_ok").unwrap();
        let dir = tmp_dir.path().join("work_dir");
        let conf = LauncherConfig {
            work_dir: dir.clone(),
            icon_path: None,
        };

        let r = conf.icon();
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), ICON_BIN.to_vec());
    }

    #[test]
    fn icon_some_ok() {
        let tmp_dir = TempDir::new("icon_some_ok").unwrap();
        let dir = tmp_dir.path().join("work_dir");
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("test-icon.png");
        write_file(path, vec![1u8].as_slice()).unwrap();
        let conf = LauncherConfig {
            work_dir: dir.clone(),
            icon_path: None,
        };

        let r = conf.icon();
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), ICON_BIN.to_vec());
    }

    #[test]
    fn icon_notfound_ng() {
        let tmp_dir = TempDir::new("icon_notfound_ng").unwrap();
        let dir = tmp_dir.path().join("work_dir");
        let path = PathBuf::from("notfound-icon-path");
        let conf = LauncherConfig {
            work_dir: dir.clone(),
            icon_path: Some(&path),
        };

        let r = conf.icon();
        assert!(r.is_err());
    }

}
