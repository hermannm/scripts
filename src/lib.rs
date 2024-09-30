use std::{ops::Deref, process::ExitCode};

use tracing::error;

#[inline]
#[must_use] // must_use, so ExitCode is not accidentally discarded
pub fn run_script(script: impl FnOnce() -> anyhow::Result<()>) -> ExitCode {
    devlog_tracing::subscriber().with_target(false).init();

    let result = script();
    match result {
        Ok(_) => ExitCode::SUCCESS,
        Err(error) => {
            error!(cause = error.deref());
            ExitCode::FAILURE
        }
    }
}
