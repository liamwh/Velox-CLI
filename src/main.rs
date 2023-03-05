#![forbid(unsafe_code)]
#![allow(clippy::pedantic)]
#![warn(clippy::all)]

use crate::prelude::*;
use app_cmd::app_cmd;
use run_new::run_veloxide_init;

mod app_cmd;
mod error;
mod exec;
mod prelude;
mod run_new;
mod utils;

pub use app_cmd::VERSION;

fn main() {
    match cmd_run() {
        Ok(_) => (),
        Err(err) => println!("FAIL - {err}"),
    }
}

fn cmd_run() -> Result<()> {
    let app = app_cmd().get_matches();

    match app.subcommand() {
        Some(("init", sub_cmd)) => run_veloxide_init(sub_cmd)?,
        _ => {
            app_cmd().print_long_help()?;
            println!("\n");
        }
    }

    Ok(())
}
