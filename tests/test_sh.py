import dkdc_sh


def test_which_exists():
    assert dkdc_sh.which("ls") is not None


def test_which_not_exists():
    assert dkdc_sh.which("nonexistent_command_12345") is None


def test_require_exists():
    path = dkdc_sh.require("ls")
    assert "ls" in path


def test_require_not_exists():
    try:
        dkdc_sh.require("nonexistent_command_12345")
        assert False, "should have raised"
    except RuntimeError:
        pass


def test_run():
    output = dkdc_sh.run("echo", ["hello"])
    assert output.strip() == "hello"


def test_run_with_env():
    output = dkdc_sh.run("env", [], env=[("DKDC_SH_TEST_VAR", "hello123")])
    assert "DKDC_SH_TEST_VAR=hello123" in output


def test_run_not_found():
    try:
        dkdc_sh.run("nonexistent_command_12345", [])
        assert False, "should have raised"
    except RuntimeError:
        pass


def test_run_failed():
    try:
        dkdc_sh.run("ls", ["/nonexistent_path_12345"])
        assert False, "should have raised"
    except RuntimeError:
        pass


def test_tmux_has_session_nonexistent():
    assert dkdc_sh.tmux_has_session("dkdc_sh_py_test_nonexistent") is False


def test_tmux_kill_session_idempotent():
    dkdc_sh.tmux_kill_session("dkdc_sh_py_test_nonexistent")


def test_git_cmd(tmp_path):
    import subprocess

    subprocess.run(["git", "init"], cwd=tmp_path, check=True, capture_output=True)
    output = dkdc_sh.git_cmd(str(tmp_path), ["status", "--short"])
    assert isinstance(output, str)


def test_git_cmd_invalid_dir():
    try:
        dkdc_sh.git_cmd("/nonexistent_dir_12345", ["status"])
        assert False, "should have raised"
    except RuntimeError:
        pass
