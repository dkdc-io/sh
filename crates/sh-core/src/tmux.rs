//! Tmux session management.

use std::process::{Command, Stdio};

use crate::{Error, require};

/// Check if a tmux session exists.
pub fn has_session(name: &str) -> bool {
    Command::new("tmux")
        .args(["has-session", "-t", name])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Create a new detached tmux session and run a command in it.
///
/// The session is created with `remain-on-exit on` so it persists
/// even after the command exits.
pub fn new_session(name: &str, cmd: &str) -> Result<(), Error> {
    require("tmux")?;

    if has_session(name) {
        return Err(Error::Tmux(format!("session '{name}' already exists")));
    }

    let status = Command::new("tmux")
        .args(["new-session", "-d", "-s", name])
        .status()?;

    if !status.success() {
        return Err(Error::Tmux(format!("failed to create session '{name}'")));
    }

    let status = Command::new("tmux")
        .args(["set-option", "-t", name, "remain-on-exit", "on"])
        .status()?;

    if !status.success() {
        return Err(Error::Tmux(
            "failed to set remain-on-exit option".to_string(),
        ));
    }

    send_keys(name, cmd)?;

    Ok(())
}

/// Kill a tmux session. Idempotent — returns Ok if already gone.
pub fn kill_session(name: &str) -> Result<(), Error> {
    require("tmux")?;

    if !has_session(name) {
        return Ok(());
    }

    let status = Command::new("tmux")
        .args(["kill-session", "-t", name])
        .status()?;

    if !status.success() {
        return Err(Error::Tmux(format!("failed to kill session '{name}'")));
    }

    Ok(())
}

/// Attach to a tmux session (takes over the terminal).
pub fn attach(name: &str) -> Result<(), Error> {
    require("tmux")?;

    if !has_session(name) {
        return Err(Error::Tmux(format!("session '{name}' does not exist")));
    }

    let status = Command::new("tmux").args(["attach", "-t", name]).status()?;

    if !status.success() {
        return Err(Error::Tmux(format!("failed to attach to session '{name}'")));
    }

    Ok(())
}

/// Send keys followed by Enter to a tmux session.
pub fn send_keys(name: &str, keys: &str) -> Result<(), Error> {
    require("tmux")?;

    if !has_session(name) {
        return Err(Error::Tmux(format!("session '{name}' does not exist")));
    }

    let status = Command::new("tmux")
        .args(["send-keys", "-t", name, keys, "Enter"])
        .status()?;

    if !status.success() {
        return Err(Error::Tmux(format!(
            "failed to send keys to session '{name}'"
        )));
    }

    Ok(())
}

/// Capture output from a tmux pane.
///
/// Returns the last `lines` lines of output, or all visible lines if None.
pub fn capture_pane(name: &str, lines: Option<usize>) -> Result<String, Error> {
    require("tmux")?;

    if !has_session(name) {
        return Err(Error::Tmux(format!("session '{name}' does not exist")));
    }

    let mut args = vec!["capture-pane", "-t", name, "-p"];

    let start_arg;
    if let Some(n) = lines {
        start_arg = format!("-{n}");
        args.extend(["-S", &start_arg]);
    }

    let output = Command::new("tmux").args(&args).output()?;

    if !output.status.success() {
        return Err(Error::Tmux(format!(
            "failed to capture pane from session '{name}'"
        )));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    static COUNTER: AtomicU32 = AtomicU32::new(0);

    fn unique_session() -> String {
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        format!("dkdc_sh_test_{id}")
    }

    #[test]
    fn test_has_session_nonexistent() {
        assert!(!has_session("dkdc_sh_nonexistent_99999"));
    }

    #[test]
    fn test_new_kill_session() {
        let name = unique_session();

        new_session(&name, "echo hello").unwrap();
        assert!(has_session(&name));

        kill_session(&name).unwrap();
        assert!(!has_session(&name));
    }

    #[test]
    fn test_new_session_duplicate() {
        let name = unique_session();

        new_session(&name, "echo hello").unwrap();
        let result = new_session(&name, "echo again");
        assert!(result.is_err());

        let _ = kill_session(&name);
    }

    #[test]
    fn test_kill_session_idempotent() {
        kill_session("dkdc_sh_nonexistent_99999").unwrap();
    }

    #[test]
    fn test_send_keys_nonexistent() {
        let result = send_keys("dkdc_sh_nonexistent_99999", "echo hi");
        assert!(result.is_err());
    }

    #[test]
    fn test_capture_pane_nonexistent() {
        let result = capture_pane("dkdc_sh_nonexistent_99999", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_attach_nonexistent() {
        let result = attach("dkdc_sh_nonexistent_99999");
        assert!(result.is_err());
    }

    #[test]
    fn test_send_keys_and_capture() {
        let name = unique_session();

        new_session(&name, "echo started").unwrap();

        std::thread::sleep(std::time::Duration::from_millis(200));
        let output = capture_pane(&name, Some(50)).unwrap();
        assert!(!output.is_empty());

        let output_all = capture_pane(&name, None).unwrap();
        assert!(!output_all.is_empty());

        let _ = kill_session(&name);
    }
}
