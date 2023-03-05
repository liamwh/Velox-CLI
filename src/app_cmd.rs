use clap::{crate_version, Arg, Command};

pub const VERSION: &str = crate_version!();

pub fn app_cmd() -> Command {
    Command::new("veloxide")
        .version(VERSION)
        .about("Veloxide App Scaffolder")
        .subcommand(sub_new())
}

fn sub_new() -> Command {
    Command::new("init")
        .about("Initialise a new Veloxide app")
        .arg(Arg::new("name").help("App/project name (no space)"))
}
