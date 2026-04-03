def which(cmd: str) -> str | None:
    """Check if a command exists in PATH. Returns the path or None."""
    ...

def require(cmd: str) -> str:
    """Require a command to exist. Returns the path or raises RuntimeError."""
    ...

def run(
    program: str,
    args: list[str],
    env: list[tuple[str, str]] | None = None,
) -> str:
    """Run a command and return its stdout."""
    ...

def tmux_has_session(name: str) -> bool:
    """Check if a tmux session exists."""
    ...

def tmux_new_session(name: str, cmd: str) -> None:
    """Create a new detached tmux session and run a command in it."""
    ...

def tmux_kill_session(name: str) -> None:
    """Kill a tmux session. Idempotent."""
    ...

def tmux_attach(name: str) -> None:
    """Attach to a tmux session (takes over the terminal)."""
    ...

def tmux_send_keys(name: str, keys: str) -> None:
    """Send keys followed by Enter to a tmux session."""
    ...

def tmux_capture_pane(name: str, lines: int | None = None) -> str:
    """Capture output from a tmux pane."""
    ...

def git_cmd(dir: str, args: list[str]) -> str:
    """Run a git command in a directory and return stdout."""
    ...

def git_cmd_with_env(
    dir: str,
    args: list[str],
    env: list[tuple[str, str]] | None = None,
) -> str:
    """Run a git command with extra environment variables."""
    ...

def git_clone_shallow(url: str, dest: str, branch: str) -> None:
    """Shallow-clone a repo (single branch, depth 1)."""
    ...

def git_clone_local(source: str, dest: str, branch: str) -> None:
    """Clone from a local repo directory (fast, shares objects via hardlinks)."""
    ...

def git_checkout_new_branch(dir: str, branch: str) -> None:
    """Create and switch to a new branch."""
    ...

def git_config_set(dir: str, key: str, value: str) -> None:
    """Set a git config key in a repo."""
    ...
