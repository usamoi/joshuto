use std::io::{BufWriter, Write};
use std::process::{Command, Stdio};
use std::str::from_utf8;

use crate::error::{AppError, AppResult};
use crate::types::option::search::CaseSensitivity;
use crate::types::state::remove_external_preview;
use crate::types::state::AppState;
use crate::ui::AppBackend;

pub fn fzf(
    app_state: &mut AppState,
    backend: &mut AppBackend,
    items: Vec<String>,
) -> AppResult<String> {
    let mut args = Vec::new();

    let case_sensitivity = app_state.config.search_options.fzf_case_sensitivity;

    match case_sensitivity {
        CaseSensitivity::Insensitive => args.push("-i".to_owned()),
        CaseSensitivity::Sensitive => args.push("+i".to_owned()),
        CaseSensitivity::Smart => {}
    }

    fzf_impl(app_state, backend, items, args)
}

pub fn fzf_multi(
    app_state: &mut AppState,
    backend: &mut AppBackend,
    items: Vec<String>,
) -> AppResult<String> {
    let mut args = Vec::new();

    let case_sensitivity = app_state.config.search_options.fzf_case_sensitivity;

    match case_sensitivity {
        CaseSensitivity::Insensitive => args.push("-i".to_owned()),
        CaseSensitivity::Sensitive => args.push("+i".to_owned()),
        CaseSensitivity::Smart => {}
    }

    args.push("-m".to_owned());
    fzf_impl(app_state, backend, items, args)
}

fn fzf_impl(
    app_state: &mut AppState,
    backend: &mut AppBackend,
    items: Vec<String>,
    args: Vec<String>,
) -> AppResult<String> {
    remove_external_preview(app_state);
    backend.terminal_drop();

    let mut cmd = Command::new("fzf");
    cmd.stdout(Stdio::piped()).args(&args);

    if !items.is_empty() {
        cmd.stdin(Stdio::piped());
    }

    let mut fzf = match cmd.spawn() {
        Ok(child) => child,
        Err(e) => {
            backend.terminal_restore()?;
            return Err(AppError::from(e));
        }
    };

    if let Some(fzf_stdin) = fzf.stdin.as_mut() {
        let mut writer = BufWriter::new(fzf_stdin);

        for item in items {
            writer.write_all(item.as_bytes())?;
        }
    }

    let fzf_output = fzf.wait_with_output();
    backend.terminal_restore()?;

    if let Ok(output) = fzf_output {
        if output.status.success() {
            if let Ok(output) = from_utf8(&output.stdout) {
                return Ok(output.to_owned());
            }
        }
    }

    Ok(String::new())
}
