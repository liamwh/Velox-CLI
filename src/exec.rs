use crate::prelude::*;
use crate::Error;
use indicatif::ProgressBar;
use std::path::Path;
use std::process::{Command, ExitStatus, Stdio};

fn build_cmd(
    cwd: Option<&Path>,
    cmd_str: &str,
    args: &[&str],
    stdout: Stdio,
    stderr: Stdio,
) -> Command {
    let mut cmd = Command::new(cmd_str);
    if let Some(cwd) = cwd {
        cmd.current_dir(cwd);
    }
    cmd.args(args);
    cmd.stdout(stdout);
    cmd.stderr(stderr);
    cmd
}

fn log_execution(
    print_exec: bool,
    progress_bar: Option<&mut ProgressBar>,
    cmd_str: &str,
    args: &[&str],
) {
    if print_exec {
        let msg = format!("> executing: {} {}", cmd_str, args.join(" "));
        match progress_bar {
            Some(pb) => pb.set_message(msg),
            None => println!("{}", msg),
        }
    }
}

pub fn spawn_and_wait(
    cwd: Option<&Path>,
    cmd_str: &str,
    args: &[&str],
    print_exec: bool,
    progress_bar: Option<&mut ProgressBar>,
) -> Result<ExitStatus> {
    let mut cmd = build_cmd(cwd, cmd_str, args, Stdio::null(), Stdio::null());
    log_execution(print_exec, progress_bar, cmd_str, args);
    let status = cmd
        .spawn()
        .map_err(|e| Error::Exec(cmd_str.to_string(), e.to_string()))?
        .wait()?;
    if !status.success() {
        Err(Error::Exec(
            cmd_str.to_string(),
            format!("Command failed with status {}", status),
        ))
    } else {
        Ok(status)
    }
}
pub fn spawn_output(
    cwd: Option<&Path>,
    cmd_str: &str,
    args: &[&str],
    print_exec: bool,
) -> Result<String> {
    log_execution(print_exec, None, cmd_str, args);

    let mut cmd = build_cmd(cwd, cmd_str, args, Stdio::piped(), Stdio::piped());
    let output = cmd
        .output()
        .map_err(|e| Error::Exec(cmd_str.to_string(), e.to_string()))?;

    let txt = if output.status.success() {
        String::from_utf8(output.stdout)
            .map_err(|_| Error::Exec(cmd_str.to_string(), "Failed to decode output".to_string()))?
    } else {
        String::from_utf8(output.stderr).map_err(|_| {
            Error::Exec(
                cmd_str.to_string(),
                "Failed to decode error output".to_string(),
            )
        })?
    };

    Ok(txt)
}
