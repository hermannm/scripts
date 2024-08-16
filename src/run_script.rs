use std::process::ExitCode;

use crate::logs::{init_logger, Log};

#[inline]
#[must_use] // must_use, so ExitCode is not accidentally discarded
pub fn run_script(script: impl FnOnce() -> anyhow::Result<()>) -> ExitCode {
    init_logger();

    let result = script();
    match result {
        Ok(_) => ExitCode::SUCCESS,
        Err(error) => {
            error.log();
            ExitCode::FAILURE
        }
    }
}
