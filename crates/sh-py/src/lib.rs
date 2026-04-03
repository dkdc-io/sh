use std::path::PathBuf;

use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

fn to_py_err(e: dkdc_sh::Error) -> PyErr {
    PyErr::new::<PyRuntimeError, _>(e.to_string())
}

// -- Root functions -----------------------------------------------------------

#[pyfunction]
fn which(cmd: &str) -> Option<String> {
    dkdc_sh::which(cmd).map(|p| p.to_string_lossy().into_owned())
}

#[pyfunction]
fn require(cmd: &str) -> PyResult<String> {
    dkdc_sh::require(cmd)
        .map(|p| p.to_string_lossy().into_owned())
        .map_err(to_py_err)
}

#[pyfunction]
#[pyo3(signature = (program, args, env=None))]
fn run(program: &str, args: Vec<String>, env: Option<Vec<(String, String)>>) -> PyResult<String> {
    let args_ref: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    match env {
        Some(env_pairs) => {
            let env_ref: Vec<(&str, &str)> = env_pairs
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect();
            dkdc_sh::run_with_env(program, &args_ref, &env_ref).map_err(to_py_err)
        }
        None => dkdc_sh::run(program, &args_ref).map_err(to_py_err),
    }
}

// -- Tmux functions -----------------------------------------------------------

#[pyfunction]
fn tmux_has_session(name: &str) -> bool {
    dkdc_sh::tmux::has_session(name)
}

#[pyfunction]
fn tmux_new_session(name: &str, cmd: &str) -> PyResult<()> {
    dkdc_sh::tmux::new_session(name, cmd).map_err(to_py_err)
}

#[pyfunction]
fn tmux_kill_session(name: &str) -> PyResult<()> {
    dkdc_sh::tmux::kill_session(name).map_err(to_py_err)
}

#[pyfunction]
fn tmux_attach(name: &str) -> PyResult<()> {
    dkdc_sh::tmux::attach(name).map_err(to_py_err)
}

#[pyfunction]
fn tmux_send_keys(name: &str, keys: &str) -> PyResult<()> {
    dkdc_sh::tmux::send_keys(name, keys).map_err(to_py_err)
}

#[pyfunction]
#[pyo3(signature = (name, lines=None))]
fn tmux_capture_pane(name: &str, lines: Option<usize>) -> PyResult<String> {
    dkdc_sh::tmux::capture_pane(name, lines).map_err(to_py_err)
}

// -- Git functions ------------------------------------------------------------

#[pyfunction]
fn git_cmd(dir: &str, args: Vec<String>) -> PyResult<String> {
    let args_ref: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    dkdc_sh::git::cmd(&PathBuf::from(dir), &args_ref).map_err(to_py_err)
}

#[pyfunction]
#[pyo3(signature = (dir, args, env=None))]
fn git_cmd_with_env(
    dir: &str,
    args: Vec<String>,
    env: Option<Vec<(String, String)>>,
) -> PyResult<String> {
    let args_ref: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let env_pairs = env.unwrap_or_default();
    let env_ref: Vec<(&str, &str)> = env_pairs
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    dkdc_sh::git::cmd_with_env(&PathBuf::from(dir), &args_ref, &env_ref).map_err(to_py_err)
}

#[pyfunction]
fn git_clone_shallow(url: &str, dest: &str, branch: &str) -> PyResult<()> {
    dkdc_sh::git::clone_shallow(url, &PathBuf::from(dest), branch).map_err(to_py_err)
}

#[pyfunction]
fn git_clone_local(source: &str, dest: &str, branch: &str) -> PyResult<()> {
    dkdc_sh::git::clone_local(&PathBuf::from(source), &PathBuf::from(dest), branch)
        .map_err(to_py_err)
}

#[pyfunction]
fn git_checkout_new_branch(dir: &str, branch: &str) -> PyResult<()> {
    dkdc_sh::git::checkout_new_branch(&PathBuf::from(dir), branch).map_err(to_py_err)
}

#[pyfunction]
fn git_config_set(dir: &str, key: &str, value: &str) -> PyResult<()> {
    dkdc_sh::git::config_set(&PathBuf::from(dir), key, value).map_err(to_py_err)
}

// -- Module -------------------------------------------------------------------

#[pymodule]
mod core {
    use super::*;

    #[pymodule_init]
    fn module_init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        // Root
        m.add_function(wrap_pyfunction!(which, m)?)?;
        m.add_function(wrap_pyfunction!(require, m)?)?;
        m.add_function(wrap_pyfunction!(run, m)?)?;
        // Tmux
        m.add_function(wrap_pyfunction!(tmux_has_session, m)?)?;
        m.add_function(wrap_pyfunction!(tmux_new_session, m)?)?;
        m.add_function(wrap_pyfunction!(tmux_kill_session, m)?)?;
        m.add_function(wrap_pyfunction!(tmux_attach, m)?)?;
        m.add_function(wrap_pyfunction!(tmux_send_keys, m)?)?;
        m.add_function(wrap_pyfunction!(tmux_capture_pane, m)?)?;
        // Git
        m.add_function(wrap_pyfunction!(git_cmd, m)?)?;
        m.add_function(wrap_pyfunction!(git_cmd_with_env, m)?)?;
        m.add_function(wrap_pyfunction!(git_clone_shallow, m)?)?;
        m.add_function(wrap_pyfunction!(git_clone_local, m)?)?;
        m.add_function(wrap_pyfunction!(git_checkout_new_branch, m)?)?;
        m.add_function(wrap_pyfunction!(git_config_set, m)?)?;
        Ok(())
    }
}
