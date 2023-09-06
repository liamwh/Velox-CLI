use crate::exec::{spawn_and_wait, spawn_output};
use crate::prelude::*;
use crate::utils::safer_remove_dir_all;
use crate::Error;
use clap::ArgMatches;
use console::{style, Emoji, Term};
use core::time;
use dialoguer::{Input, Select};
use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use regex::bytes::Regex;
use std::path::Path;
use std::str::from_utf8;
use std::time::Instant;
use walkdir::WalkDir;

const DEFAULT_APP_NAME: &str = "my-veloxide-app";
const GIT_DIR: &str = ".git";
const GIT_TEMPLATE_BASE: &str = "https://github.com/liamwh/Veloxide.git";

static CHEEKY: Emoji<'_, '_> = Emoji("😜", "");
static X: Emoji<'_, '_> = Emoji("❌", "x");
static TICK: Emoji<'_, '_> = Emoji("✅", "");
static DOWN_ARROW: Emoji<'_, '_> = Emoji("⏬", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨", ":-)");

const FRONTEND_FRAMEWORK_SELECTION_SVELTEKIT: &str = "SvelteKit";
const FRONTEND_FRAMEWORK_SELECTION_REACT: &str = "React (Next)";

pub fn run_veloxide_init(sub_cmd: &ArgMatches) -> Result<()> {
    check_git_is_installed()?;
    let term = Term::stdout();
    let app_name = if let Some(name) = sub_cmd.get_one::<String>("name") {
        name.to_string()
    } else {
        Input::new()
            .with_prompt("What is your app name?")
            .default(DEFAULT_APP_NAME.to_string())
            .interact()?
    };

    // --- Compute the app dir
    let re = Regex::new(r"[^A-Za-z0-9]").unwrap();
    let app_dir_name = re.replace_all(app_name.as_bytes(), "-".as_bytes());
    let app_dir_name = Regex::new(r"[-]+$")
        .unwrap()
        .replace_all(&app_dir_name, "".as_bytes())
        .into_owned();
    let app_dir_name = from_utf8(&app_dir_name).unwrap().to_lowercase();
    let app_dir = Path::new(&app_dir_name);
    if app_dir.exists() {
        return Err(Error::DirAlreadyExist(s!(app_dir.to_string_lossy())));
    }
    let mut frontend = "";
    let confirm_selections = &["Yes", "No"];
    let wants_a_frontend: bool = Select::new()
        .with_prompt("Do you want a frontend?")
        .default(0)
        .items(&confirm_selections[..])
        .interact()?
        == 0;
    if wants_a_frontend {
        let selections = &[
            FRONTEND_FRAMEWORK_SELECTION_SVELTEKIT,
            FRONTEND_FRAMEWORK_SELECTION_REACT,
        ];
        loop {
            let selection = Select::new()
                .with_prompt("Select a frontend framework")
                .default(0)
                .items(&selections[..])
                .interact()
                .unwrap();

            match selections[selection] {
                "SvelteKit" => {
                    frontend = "sveltekit";
                    term.write_line(format!("\u{1b}[32mGreat choice! {}\u{1b}[0m", TICK).as_str())
                        .unwrap();
                    break;
                }
                "React (Next)" => {
                    term.write_line(
                        format!("\u{1b}[31m{} Wrong answer {}\u{1b}[0m", X, CHEEKY).as_str(),
                    )
                    .unwrap();
                }
                _ => {}
            }
        }
    }
    let started = Instant::now();

    println!("{} Cloning the Veloxide template...", DOWN_ARROW);
    let mut pb = ProgressBar::new_spinner();
    let spinner_style = ProgressStyle::with_template("{spinner} {wide_msg}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
    pb.set_style(spinner_style.clone());
    pb.set_message("Cloning repository...");
    pb.enable_steady_tick(time::Duration::from_millis(100));

    spawn_and_wait(
        None,
        "git",
        &[
            "clone",
            "--depth",
            "1",
            "--branch",
            "main",
            GIT_TEMPLATE_BASE,
            &app_dir_name,
        ],
        false,
        Some(&mut pb),
    )?;

    let git_dir = app_dir.join(GIT_DIR);
    let msg = format!(
        "Deleting template git directory ({})...",
        git_dir.to_string_lossy()
    );
    pb.set_message(msg);
    safer_remove_dir_all(&git_dir)?;
    let msg = "Initialing your git repo...".to_string();
    pb.set_message(msg);
    spawn_and_wait(Some(app_dir), "git", &["init", "."], false, Some(&mut pb))?;
    spawn_and_wait(
        Some(app_dir),
        "git",
        &["add", "-A", "."],
        false,
        Some(&mut pb),
    )?;
    spawn_and_wait(
        Some(app_dir),
        "git",
        &["commit", "-a", "-m", ". initial"],
        false,
        Some(&mut pb),
    )?;
    let msg = "Copying .env.example files to .env".to_string();
    pb.set_message(msg);
    for entry in WalkDir::new(app_dir) {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "example" {
                    let env_file = path.with_extension("");
                    std::fs::copy(path, &env_file)?;
                }
            }
        }
    }
    let frontends_dir = app_dir.join("frontends");
    for entry in WalkDir::new(&frontends_dir).min_depth(1).max_depth(1) {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(folder_name) = path.file_name() {
                if folder_name.to_string_lossy() == frontend {
                    continue;
                } else {
                    safer_remove_dir_all(path)?;
                }
            }
        }
    }
    let finished_style = ProgressStyle::default_bar()
        .template("{msg}")
        .expect("expected to be able to set the style");
    pb.set_style(finished_style);
    let msg = format!("{} Done in {}", SPARKLE, HumanDuration(started.elapsed()));
    pb.finish_with_message(msg);
    term.write_line("\n")?;
    term.write_line("Next steps:")?;
    let max_steps = 5;
    let mut current_step = 1;
    write_step(
        &term,
        current_step,
        max_steps,
        &format!("> cd {}", app_dir_name),
    )?;
    current_step += 1;

    write_step(&term, current_step, max_steps, "> just run-backend")?;
    current_step += 1;

    write_step(
        &term,
        current_step,
        max_steps,
        "First compile takes a little while, but future compiles will be faster.",
    )?;
    current_step += 1;

    write_step(
        &term,
        current_step,
        max_steps,
        &format!("Open {} in your IDE.", app_dir_name),
    )?;
    current_step += 1;

    write_step(&term, current_step, max_steps, "Happy coding!")?;
    Ok(())
}

fn check_git_is_installed() -> Result<()> {
    spawn_output(None, "git", &["--version"], false).map_err(|_| Error::GitNotPresent)?;
    Ok(())
}

fn write_step(term: &Term, current: usize, max: usize, msg: &str) -> Result<()> {
    let styled_msg = format!(
        "{} {}",
        style(format!("[{}/{}]", current, max)).bold().dim(),
        msg
    );
    term.write_line(&styled_msg)?;
    Ok(())
}
