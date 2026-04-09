//! Open files in the user's preferred editor.

use crate::{Error, require};
use std::path::Path;
use std::process::Command;

const DEFAULT_EDITOR: &str = "vim";

/// Open a file in `$EDITOR` (falls back to vim).
pub fn open(path: &Path) -> Result<(), Error> {
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| DEFAULT_EDITOR.to_string());
    require(&editor)?;

    let status = Command::new(&editor).arg(path).status()?;

    if !status.success() {
        return Err(Error::CommandFailed {
            cmd: editor,
            detail: format!("failed to open {}", path.display()),
        });
    }

    Ok(())
}
