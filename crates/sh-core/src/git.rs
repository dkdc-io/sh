//! Git command abstractions (sync).

use std::path::Path;
use std::process::Command;

use crate::{Error, require};

/// Run a git command in a directory and return stdout.
pub fn cmd(dir: &Path, args: &[&str]) -> Result<String, Error> {
    cmd_with_env(dir, args, &[])
}

/// Run a git command with extra environment variables.
///
/// When `GIT_ASKPASS` is present in `env`, credential helpers are disabled
/// to prevent interception by system keychains.
pub fn cmd_with_env(dir: &Path, args: &[&str], env: &[(&str, &str)]) -> Result<String, Error> {
    require("git")?;

    let has_askpass = env.iter().any(|(k, _)| *k == "GIT_ASKPASS");

    let mut command = Command::new("git");
    if has_askpass {
        command.args(["-c", "credential.helper="]);
    }
    command.args(args).current_dir(dir);
    for (k, v) in env {
        command.env(k, v);
    }
    let output = command.output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(Error::CommandFailed {
            cmd: format!("git {}", args.first().unwrap_or(&"")),
            detail: stderr,
        });
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Shallow-clone a repo (single branch, depth 1).
pub fn clone_shallow(url: &str, dest: &Path, branch: &str) -> Result<(), Error> {
    clone_shallow_with_env(url, dest, branch, &[])
}

/// Shallow-clone a repo with extra environment variables.
pub fn clone_shallow_with_env(
    url: &str,
    dest: &Path,
    branch: &str,
    env: &[(&str, &str)],
) -> Result<(), Error> {
    require("git")?;

    let has_askpass = env.iter().any(|(k, _)| *k == "GIT_ASKPASS");

    let mut command = Command::new("git");
    if has_askpass {
        command.args(["-c", "credential.helper="]);
    }
    command.args([
        "clone",
        "--depth",
        "1",
        "--branch",
        branch,
        url,
        &dest.to_string_lossy(),
    ]);
    for (k, v) in env {
        command.env(k, v);
    }
    let output = command.output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(Error::CommandFailed {
            cmd: "git clone".to_string(),
            detail: stderr,
        });
    }

    Ok(())
}

/// Clone from a local repo directory (fast, shares objects via hardlinks).
pub fn clone_local(source: &Path, dest: &Path, branch: &str) -> Result<(), Error> {
    require("git")?;

    let output = Command::new("git")
        .args([
            "clone",
            "--branch",
            branch,
            "--single-branch",
            &source.to_string_lossy(),
            &dest.to_string_lossy(),
        ])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(Error::CommandFailed {
            cmd: "git clone (local)".to_string(),
            detail: stderr,
        });
    }

    Ok(())
}

/// Create and switch to a new branch.
pub fn checkout_new_branch(dir: &Path, branch: &str) -> Result<(), Error> {
    cmd(dir, &["checkout", "-b", branch])?;
    Ok(())
}

/// Set a git config key in a repo.
pub fn config_set(dir: &Path, key: &str, value: &str) -> Result<(), Error> {
    cmd(dir, &["config", key, value])?;
    Ok(())
}
