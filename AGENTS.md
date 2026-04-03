# sh

Shell utilities for tmux, git, and command management.

## Commands

```bash
bin/build          # Build all (Rust + Python)
bin/build-rs       # Build Rust crate
bin/build-py       # Build Python bindings (maturin develop)
bin/check          # Run all checks (format, lint, test)
bin/check-rs       # Rust checks (fmt, clippy, test)
bin/check-py       # Python checks (ruff, ty)
bin/test           # Run all tests
bin/test-rs        # Rust tests
bin/format         # Format all code
bin/bump-version   # Bump version (--patch, --minor (default), --major)
```

## Architecture

```
crates/sh-core/       # Core library (dkdc-sh on crates.io)
  src/lib.rs           # Error type, which, require, run
  src/tmux.rs          # Tmux session management
  src/git.rs           # Git command abstractions
crates/sh-py/         # PyO3 bindings (cdylib)
py/dkdc_sh/           # Python wrapper + type stubs (core.pyi, py.typed)
```

Library only — no binaries. Pure sync, no async runtime. Single dependency: `which`.
