use core::fmt;
use std::{error::Error, fmt::Debug};

use tracing::{
    error,
    field::{Field, Visit},
};
use tracing_subscriber::{
    field::{MakeVisitor, VisitFmt, VisitOutput},
    fmt::format::Writer,
};

pub(crate) fn init_logger() {
    tracing_subscriber::fmt()
        .fmt_fields(DevLogFieldFormatter::new())
        .without_time()
        .with_target(false)
        .init();
}

pub(crate) fn log_error(error: &anyhow::Error) {
    error!(cause = error.source(), "{}", error.to_string());
}

/// A log field formatter for `tracing`, with a prettified, newline-delimited format. This
/// aims to improve readability over the default log field format, which appends log fields on the
/// same line, making it hard to read when multiple fields are appended.
///
/// ### Usage
///
/// ```rust
/// tracing_subscriber::fmt()
///     .fmt_fields(DevLogFieldFormatter::new())
///     .init();
/// ```
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
struct DevLogFieldFormatter {
    // To prevent direct struct initialization, so we can add fields here later as a non-breaking
    // change.
    _private: (),
}

impl DevLogFieldFormatter {
    fn new() -> Self {
        DevLogFieldFormatter { _private: () }
    }
}

impl<'a> MakeVisitor<Writer<'a>> for DevLogFieldFormatter {
    type Visitor = DevLogFieldVisitor<'a>;

    fn make_visitor(&self, writer: Writer<'a>) -> Self::Visitor {
        DevLogFieldVisitor {
            writer,
            result: Ok(()),
            first_visit: true,
        }
    }
}

struct DevLogFieldVisitor<'a> {
    writer: Writer<'a>,
    result: fmt::Result,
    first_visit: bool,
}

impl<'a> DevLogFieldVisitor<'a> {
    fn write_field(&mut self, field: &Field, value: &dyn Debug) {
        self.write_field_name(field);
        if self.result.is_err() {
            return;
        }
        self.result = write!(self.writer, " {value:?}");
    }

    fn write_string_field(&mut self, field: &Field, value: &str) {
        self.write_field_name(field);
        if self.result.is_err() {
            return;
        }
        self.result = write!(self.writer, " {value}")
    }

    fn write_field_name(&mut self, field: &Field) {
        self.result = if self.writer.has_ansi_escapes() {
            write!(self.writer, "{CYAN_COLOR}{field}{GRAY_COLOR}:{RESET_COLOR}")
        } else {
            write!(self.writer, "{field}:")
        };
    }

    fn write_string_list_item(&mut self, value: &str) {
        if self.result.is_err() {
            return;
        }

        self.result = if self.writer.has_ansi_escapes() {
            write!(
                self.writer,
                "{LIST_ITEM_DELIMITER}{GRAY_COLOR}-{RESET_COLOR} {value}"
            )
        } else {
            write!(self.writer, "{LIST_ITEM_DELIMITER}- {value}")
        }
    }

    fn delimit(&mut self) {
        if self.result.is_err() {
            return;
        }

        // The first field is the main log message, which we don't want to delimit
        if !self.first_visit {
            self.result = self.writer().write_str(LOG_FIELD_DELIMITER);
        }
    }
}

impl<'a> Visit for DevLogFieldVisitor<'a> {
    fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
        self.delimit();
        if self.result.is_err() {
            return;
        }

        // The first field is the main log message, which we want to write without field name
        if self.first_visit {
            self.first_visit = false;
            self.result = write!(self.writer, "{value:?}")
        } else {
            self.write_field(field, value)
        }
    }

    fn record_error(&mut self, field: &Field, error: &(dyn Error + 'static)) {
        self.delimit();
        if self.result.is_err() {
            return;
        }

        // If the error has no cause, we just log the error
        let Some(cause) = error.source() else {
            self.write_string_field(field, &error.to_string());
            return;
        };

        self.write_field_name(field);
        self.write_string_list_item(&error.to_string());
        self.write_string_list_item(&cause.to_string());
        while let Some(cause) = cause.source() {
            self.write_string_list_item(&cause.to_string());
        }
    }
}

impl<'a> VisitOutput<fmt::Result> for DevLogFieldVisitor<'a> {
    fn finish(self) -> fmt::Result {
        self.result
    }
}

impl<'a> VisitFmt for DevLogFieldVisitor<'a> {
    fn writer(&mut self) -> &mut dyn fmt::Write {
        &mut self.writer
    }
}

const LOG_FIELD_DELIMITER: &str = "\n  ";
const LIST_ITEM_DELIMITER: &str = "\n    ";

const CYAN_COLOR: &str = "\x1b[36m";
const GRAY_COLOR: &str = "\x1b[37m";
const RESET_COLOR: &str = "\x1b[0m";
