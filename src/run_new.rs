use crate::exec::{prompt, spawn_and_wait, spawn_output};
use crate::prelude::*;
use crate::utils::safer_remove_dir_all;
use crate::Error;
use clap::ArgMatches;
use regex::bytes::Regex;
use std::path::Path;
use std::str::from_utf8;

const DEFAULT_APP_NAME: &str = "my-veloxide-app";
const GIT_DIR: &str = ".git";
const GIT_TMPL_BASE: &str = "https://github.com/liamwh/Veloxide.git";

pub fn run_veloxide_init(sub_cmd: &ArgMatches) -> Result<()> {
    check_git()?;

    // --- Get the name
    let app_name = sub_cmd.get_one::<String>("name");

    let app_name = match app_name {
        Some(name) => name.to_string(),
        None => prompt(
            &f!("What is your app name? ({DEFAULT_APP_NAME}): "),
            Some(DEFAULT_APP_NAME),
        )?,
    };

    // --- Compute the app dir
    let re = Regex::new(r"[^A-Za-z0-9]").unwrap();
    let app_dir_name = re.replace_all(app_name.as_bytes(), "-".as_bytes());
    // remove the last '-' chars
    let app_dir_name = Regex::new(r"[-]+$")
        .unwrap()
        .replace_all(&app_dir_name, "".as_bytes())
        .into_owned();
    let app_dir_name = from_utf8(&app_dir_name).unwrap().to_lowercase();
    let app_dir = Path::new(&app_dir_name);

    // check if the dir already exist
    if app_dir.exists() {
        return Err(Error::DirAlreadyExist(s!(app_dir.to_string_lossy())));
    }

    // --- Do the git clone
    println!("Cloning the Veloxide template...");
    // git clone --depth 1 --branch <tag_name> <repo_url>
    spawn_and_wait(
        None,
        "git",
        &[
            "clone",
            "--depth",
            "1",
            "--branch",
            "main",
            GIT_TMPL_BASE,
            &app_dir_name,
        ],
        true,
    )?;

    // --- Remove the git folder
    let git_dir = app_dir.join(GIT_DIR);
    println!(
        "Delete template git directory ({})",
        git_dir.to_string_lossy()
    );
    safer_remove_dir_all(&git_dir)?;

    // --- Do the git init and initial
    spawn_and_wait(Some(app_dir), "git", &["init", "."], true)?;
    spawn_and_wait(Some(app_dir), "git", &["add", "-A", "."], true)?;
    spawn_and_wait(
        Some(app_dir),
        "git",
        &["commit", "-a", "-m", ". initial"],
        true,
    )?;

    println!(
        "
Next steps:

> cd {app_dir_name}

> make start

First compile takes a little while, but future compiles will be faster.

Open {app_dir_name} in your IDE.

Happy coding!
"
    );

    Ok(())
}

fn check_git() -> Result<()> {
    spawn_output(None, "git", &["--version"], false).map_err(|_| Error::GitNotPresent)?;
    Ok(())
}
