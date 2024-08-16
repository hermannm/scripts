use tracing::error;
use tracing_subscriber::{
    field::MakeExt,
    fmt::{format, FormatFields},
};

pub(crate) fn init_logger() {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .fmt_fields(make_prettified_log_field_formatter())
        .init();
}

/// Creates a log field formatter for `tracing`, with a prettified, newline-delimited format. This
/// aims to improve readability over the default log field format, which appends log fields on the
/// same line, making it hard to read when multiple fields are appended.
///
/// ### Example
///
/// This example log:
/// ```rust
/// error!(reason = "Bad things", severity = "BAD", "Something went wrong");
/// ```
/// ...gets printed like this:
/// ```text
/// ERROR Something went wrong
///   reason: "Bad things"
///   severity: "BAD"
/// ```
/// If your terminal supports ASCII color codes, the log field names ("reason" and "severity") above
/// will be colored, to distinguish them from field values.
fn make_prettified_log_field_formatter() -> impl for<'writer> FormatFields<'writer> {
    format::debug_fn(|writer, field, value| {
        // "message" is the main log message, we don't want to show field name for that
        if field.name() == "message" {
            write!(writer, "{value:?}")
        } else if writer.has_ansi_escapes() {
            write!(
                writer,
                "  {CYAN_COLOR}{field}{GRAY_COLOR}:{RESET_COLOR} {value:?}"
            )
        } else {
            write!(writer, "  {field}: {value:?}")
        }
    })
    .delimited("\n")
}

pub(crate) fn log_error(error: anyhow::Error) {
    // If the error has no cause, we just log the error
    let Some(mut cause) = error.source() else {
        error!("{error}");
        return;
    };

    let mut cause_string = cause.to_string();
    // If there are multiple levels of error causes, we append them, separated by ': '
    while let Some(child_cause) = cause.source() {
        cause_string.push_str(": ");
        cause_string.push_str(&child_cause.to_string());
        cause = child_cause;
    }

    error!(cause = cause_string, "{error}");
}

const CYAN_COLOR: &str = "\x1b[36m";
const GRAY_COLOR: &str = "\x1b[37m";
const RESET_COLOR: &str = "\x1b[0m";
