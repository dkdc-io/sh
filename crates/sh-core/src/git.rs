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
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        return Err(Error::CommandFailed {
            cmd: format!("git {}", args.first().unwrap_or(&"")),
            detail: stderr,
        });
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
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
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
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
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    /// Create a temporary git repo for testing.
    fn temp_repo() -> tempfile::TempDir {
        let dir = tempfile::tempdir().unwrap();
        Command::new("git")
            .args(["init"])
            .current_dir(dir.path())
            .output()
            .unwrap();
        Command::new("git")
            .args(["config", "user.email", "test@test.com"])
            .current_dir(dir.path())
            .output()
            .unwrap();
        Command::new("git")
            .args(["config", "user.name", "Test"])
            .current_dir(dir.path())
            .output()
            .unwrap();
        // Create an initial commit so HEAD exists
        fs::write(dir.path().join("README.md"), "# test").unwrap();
        Command::new("git")
            .args(["add", "."])
            .current_dir(dir.path())
            .output()
            .unwrap();
        Command::new("git")
            .args(["commit", "-m", "init"])
            .current_dir(dir.path())
            .output()
            .unwrap();
        dir
    }

    #[test]
    fn test_cmd_status() {
        let repo = temp_repo();
        let output = cmd(repo.path(), &["status", "--short"]).unwrap();
        assert!(output.is_empty()); // clean repo
    }

    #[test]
    fn test_cmd_invalid_dir() {
        let result = cmd(Path::new("/nonexistent_dir_12345"), &["status"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_cmd_invalid_subcommand() {
        let repo = temp_repo();
        let result = cmd(repo.path(), &["not-a-real-subcommand"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_config_set_and_read() {
        let repo = temp_repo();
        config_set(repo.path(), "user.name", "TestUser").unwrap();
        let output = cmd(repo.path(), &["config", "user.name"]).unwrap();
        assert_eq!(output.trim(), "TestUser");
    }

    #[test]
    fn test_checkout_new_branch() {
        let repo = temp_repo();
        checkout_new_branch(repo.path(), "feature-test").unwrap();
        let output = cmd(repo.path(), &["branch", "--show-current"]).unwrap();
        assert_eq!(output.trim(), "feature-test");
    }

    #[test]
    fn test_clone_local() {
        let repo = temp_repo();
        let dest = tempfile::tempdir().unwrap();
        let dest_path = dest.path().join("cloned");
        // Get current branch name
        let branch = cmd(repo.path(), &["branch", "--show-current"])
            .unwrap()
            .trim()
            .to_string();
        clone_local(repo.path(), &dest_path, &branch).unwrap();
        assert!(dest_path.join(".git").exists());
    }

    #[test]
    fn test_cmd_with_env() {
        let repo = temp_repo();
        // GIT_AUTHOR_NAME env var doesn't affect `status`, but we verify the call succeeds
        let output = cmd_with_env(
            repo.path(),
            &["status", "--short"],
            &[("GIT_AUTHOR_NAME", "X")],
        )
        .unwrap();
        assert!(output.is_empty());
    }
}
