# dkdc-sh

[![crates.io](https://img.shields.io/crates/v/dkdc-sh?color=blue)](https://crates.io/crates/dkdc-sh)
[![PyPI](https://img.shields.io/pypi/v/dkdc-sh?color=blue)](https://pypi.org/project/dkdc-sh/)
[![CI](https://img.shields.io/github/actions/workflow/status/dkdc-io/sh/ci.yml?branch=main&label=CI)](https://github.com/dkdc-io/sh/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-8A2BE2.svg)](https://github.com/dkdc-io/sh/blob/main/LICENSE)

Shell utilities for tmux, git, and command management.

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
use dkdc_sh::{which, require, run, tmux, git};

// Command checking
if which("tmux").is_some() { /* ... */ }
require("git")?;

// Run arbitrary commands
let output = run("echo", &["hello"])?;

// Tmux session management
tmux::new_session("my-service", "python server.py")?;
tmux::send_keys("my-service", "reload")?;
let logs = tmux::capture_pane("my-service", Some(50))?;
tmux::kill_session("my-service")?;

// Git operations
git::clone_shallow("https://github.com/org/repo.git", &dest, "main")?;
git::checkout_new_branch(&repo_dir, "feature/branch")?;
git::config_set(&repo_dir, "user.email", "bot@example.com")?;
```

### Python

```python
import dkdc_sh

# Command checking
path = dkdc_sh.which("tmux")
dkdc_sh.require("git")

# Run commands
output = dkdc_sh.run("echo", ["hello"])

# Tmux
dkdc_sh.tmux_new_session("my-service", "python server.py")
logs = dkdc_sh.tmux_capture_pane("my-service", lines=50)
dkdc_sh.tmux_kill_session("my-service")

# Git
dkdc_sh.git_clone_shallow("https://github.com/org/repo.git", "./dest", "main")
dkdc_sh.git_checkout_new_branch("./repo", "feature/branch")
```
