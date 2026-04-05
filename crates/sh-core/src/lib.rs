//! Shell utilities for tmux, git, and command management.
//!
//! Minimal, synchronous shell abstractions. No async runtime required.

use std::fmt;
use std::path::PathBuf;
use std::process::Command;

pub mod git;
pub mod tmux;

/// Shell operation errors.
#[derive(Debug)]
pub enum Error {
    CommandNotFound(String),
    CommandFailed { cmd: String, detail: String },
    Tmux(String),
    Io(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CommandNotFound(cmd) => write!(f, "command not found: {cmd}"),
            Error::CommandFailed { cmd, detail } => write!(f, "command failed: {cmd} — {detail}"),
            Error::Tmux(msg) => write!(f, "tmux error: {msg}"),
            Error::Io(err) => write!(f, "io error: {err}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

/// Check if a command exists in PATH.
pub fn which(cmd: &str) -> Option<PathBuf> {
    ::which::which(cmd).ok()
}

/// Require a command to exist, returning an error if not found.
pub fn require(cmd: &str) -> Result<PathBuf, Error> {
    ::which::which(cmd).map_err(|_| Error::CommandNotFound(cmd.to_string()))
}

/// Run a command and return its stdout.
pub fn run(program: &str, args: &[&str]) -> Result<String, Error> {
    run_with_env(program, args, &[])
}

/// Run a command with extra environment variables and return its stdout.
pub fn run_with_env(program: &str, args: &[&str], env: &[(&str, &str)]) -> Result<String, Error> {
    require(program)?;

    let mut command = Command::new(program);
    command.args(args);
    for (k, v) in env {
        command.env(k, v);
    }
    let output = command.output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        return Err(Error::CommandFailed {
            cmd: format!("{program} {}", args.first().unwrap_or(&"")),
            detail: stderr,
        });
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_which_exists() {
        assert!(which("ls").is_some());
    }

    #[test]
    fn test_which_not_exists() {
        assert!(which("nonexistent_command_12345").is_none());
    }

    #[test]
    fn test_require_exists() {
        assert!(require("ls").is_ok());
    }

    #[test]
    fn test_require_not_exists() {
        let result = require("nonexistent_command_12345");
        assert!(result.is_err());
        assert!(matches!(result, Err(Error::CommandNotFound(_))));
    }

    #[test]
    fn test_run() {
        let output = run("echo", &["hello"]).unwrap();
        assert_eq!(output.trim(), "hello");
    }

    #[test]
    fn test_run_not_found() {
        let result = run("nonexistent_command_12345", &[]);
        assert!(matches!(result, Err(Error::CommandNotFound(_))));
    }

    #[test]
    fn test_run_failed_command() {
        let result = run("ls", &["/nonexistent_path_12345"]);
        assert!(matches!(result, Err(Error::CommandFailed { .. })));
    }

    #[test]
    fn test_run_with_env() {
        let output = run_with_env("env", &[], &[("DKDC_SH_TEST_VAR", "hello123")]).unwrap();
        assert!(output.contains("DKDC_SH_TEST_VAR=hello123"));
    }

    #[test]
    fn test_error_display() {
        let err = Error::CommandNotFound("foo".to_string());
        assert_eq!(err.to_string(), "command not found: foo");

        let err = Error::CommandFailed {
            cmd: "bar".to_string(),
            detail: "oops".to_string(),
        };
        assert!(err.to_string().contains("bar"));

        let err = Error::Tmux("bad".to_string());
        assert!(err.to_string().contains("bad"));
    }
}
