mod albert;
mod alfred;
mod args;
mod cargo;
mod core;
mod error;
mod fs;
mod hain;
mod launcher;
mod tpl;

use log::debug;
use pretty_env_logger;
use std::process::exit;

use crate::args::args;
use crate::cargo::config;
use crate::error::Result;
use crate::launcher::launch;

const SUCCESS_CODE: i32 = 0;
const FAILED_CODE: i32 = 1;

fn main() -> Result<()> {
    pretty_env_logger::init();
    let args = args();
    debug!("args: {:?}", args);
    let config = config(&None, args.bin_name.as_ref().map(String::as_str))?;

    match launch(&args, &config) {
        Ok(msg) => {
            println!("{}", msg);
            exit(SUCCESS_CODE)
        }
        Err(err) => {
            eprintln!("Error occurred: {}", err);
            exit(FAILED_CODE);
        }
    }
}
