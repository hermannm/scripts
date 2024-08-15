use std::{
    fs::OpenOptions,
    io::{Read, Seek, SeekFrom, Write},
};

use anyhow::{Context, Result};
use clap::Parser;

/// Prepends the contents of a given file with a newline IF it already starts with a newline.
///
/// Used to add an extra newline to the start of the Git commit message template.
#[derive(Parser, Debug)]
#[command(
    about,
    trailing_var_arg = true,
    allow_hyphen_values = true,
    ignore_errors = true
)]
struct Args {
    /// Path of the file to prepend a newline to.
    #[arg()]
    file_path: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(args.file_path)
        .context("Failed to open file")?;

    const DEFAULT_COMMIT_MESSAGE_FILE_SIZE: usize = 437;
    let mut contents = String::with_capacity(DEFAULT_COMMIT_MESSAGE_FILE_SIZE + 1); // +1 for the newline we're prepending

    file.read_to_string(&mut contents)
        .context("Failed to read file")?;

    if contents.starts_with('\n') {
        contents.insert(0, '\n');

        file.seek(SeekFrom::Start(0))
            .context("Failed to seek to beginning of file")?;
        file.write_all(contents.as_bytes())
            .context("Failed to write to file")?;
    }

    Ok(())
}
