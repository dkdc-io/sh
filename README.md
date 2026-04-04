# dkdc-sh

[![GitHub Release](https://img.shields.io/github/v/release/dkdc-io/sh?color=blue)](https://github.com/dkdc-io/sh/releases)
[![crates.io](https://img.shields.io/crates/v/dkdc-sh?color=blue)](https://crates.io/crates/dkdc-sh)
[![PyPI](https://img.shields.io/pypi/v/dkdc-sh?color=blue)](https://pypi.org/project/dkdc-sh/)
[![CI](https://img.shields.io/github/actions/workflow/status/dkdc-io/sh/ci.yml?branch=main&label=CI)](https://github.com/dkdc-io/sh/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-8A2BE2.svg)](https://github.com/dkdc-io/sh/blob/main/LICENSE)

Shell library.

## Install

```bash
cargo add dkdc-sh
```

```bash
uv add dkdc-sh
```

## Usage

### Rust

```rust
use dkdc_sh::{which, require, run, run_with_env, tmux, git};

// Command checking
if which("tmux").is_some() { /* ... */ }
require("git")?;

// Run arbitrary commands
let output = run("echo", &["hello"])?;

// Run with custom environment variables
let output = run_with_env("cargo", &["build"], &[("RUSTFLAGS", "-D warnings")])?;

// Tmux session management
tmux::new_session("my-service", "python server.py")?;
tmux::send_keys("my-service", "reload")?;
let logs = tmux::capture_pane("my-service", Some(50))?;
tmux::kill_session("my-service")?;

// Git operations
git::clone_shallow("https://github.com/org/repo.git", &dest, "main")?;
git::clone_shallow_with_env(
    "https://github.com/org/repo.git", &dest, "main",
    &[("GIT_SSH_COMMAND", "ssh -i ~/.ssh/deploy_key")],
)?;
git::checkout_new_branch(&repo_dir, "feature/branch")?;
git::config_set(&repo_dir, "user.email", "bot@example.com")?;
let log = git::cmd_with_env(&repo_dir, &["log", "--oneline"], &[("GIT_PAGER", "cat")])?;
```

### Python

```python
import dkdc_sh

# Command checking
path = dkdc_sh.which("tmux")
dkdc_sh.require("git")

# Run commands
output = dkdc_sh.run("echo", ["hello"])

# Run with custom environment variables
output = dkdc_sh.run("cargo", ["build"], env=[("RUSTFLAGS", "-D warnings")])

# Tmux
dkdc_sh.tmux_new_session("my-service", "python server.py")
dkdc_sh.tmux_send_keys("my-service", "reload")
logs = dkdc_sh.tmux_capture_pane("my-service", lines=50)
dkdc_sh.tmux_kill_session("my-service")

# Git
dkdc_sh.git_clone_shallow("https://github.com/org/repo.git", "./dest", "main")
dkdc_sh.git_checkout_new_branch("./repo", "feature/branch")
output = dkdc_sh.git_cmd("./repo", ["log", "--oneline"])
```

## Error handling

### Rust

The library uses a unified `Error` enum:

| Variant | Description |
|---------|-------------|
| `CommandNotFound` | Command not found in PATH |
| `CommandFailed` | Non-zero exit code (includes stderr) |
| `Tmux` | Tmux-specific errors |
| `Io` | IO errors |

### Python

All errors are raised as `RuntimeError`.
