use crate::launcher::Launcher;
use std::path::PathBuf;
use structopt::*;

#[derive(StructOpt)]
#[structopt(bin_name = "cargo")]
enum Command {
    #[structopt(name = "launcher")]
    SubCommand(Args),
}

#[derive(StructOpt, Debug)]
pub struct Args {
    #[structopt(short = "b", long = "bin")]
    pub bin_name: Option<String>,
    #[structopt(short = "i", long = "icon", parse(from_os_str))]
    pub icon_path: Option<PathBuf>,
    #[structopt(name = "launcher")]
    pub launcher: Launcher,
}

pub fn args() -> Args {
    let Command::SubCommand(args) = Command::from_args();
    args
}
