use std::fmt::Write;

use crate::{
    formatter::pattern_formatter::{Pattern, PatternContext},
    Error, Record, StringBuf,
};

/// A pattern that writes the source file, line and column of a log record into
/// the output. Example: `main.rs:30:20`.
///
/// This pattern corresponds to `{@}` or `{loc}` in the pattern template string.
#[derive(Default)]
pub struct Loc;

impl Loc {
    /// Create a new `Loc` pattern.
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for Loc {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        if let Some(loc) = record.source_location() {
            dest.write_fmt(format_args!(
                "{}:{}:{}",
                loc.file_name(),
                loc.line(),
                loc.column()
            ))
            .map_err(Error::FormatRecord)?;
        }
        Ok(())
    }
}

/// A pattern that writes the source file basename into the output. Example:
/// `main.rs`.
///
/// This pattern corresponds to `{s}` or `{source-basename}` in the pattern
/// template string.
#[derive(Default)]
pub struct SourceBasename;

impl SourceBasename {
    /// Create a new `SourceBasename` pattern.
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for SourceBasename {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        if let Some(loc) = record.source_location() {
            dest.write_str(loc.file_name())
                .map_err(Error::FormatRecord)?;
        }
        Ok(())
    }
}

/// A pattern that writes the source file path into the output. Example:
/// `src/main.rs`.
///
/// This pattern corresponds to `{g}` or `{source}` in the pattern template
/// string.
#[derive(Default)]
pub struct SourcePath;

impl SourcePath {
    /// Create a new `SourcePath` pattern.
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for SourcePath {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        if let Some(loc) = record.source_location() {
            dest.write_str(loc.file()).map_err(Error::FormatRecord)?;
        }
        Ok(())
    }
}

/// A pattern that writes the source line into the output. Example: `20`.
///
/// This pattern corresponds to `{#}` or `{line}` in the pattern template
/// string.
#[derive(Default)]
pub struct SourceLine;

impl SourceLine {
    /// Create a new `SourceLine` pattern.
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for SourceLine {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        if let Some(loc) = record.source_location() {
            dest.write_fmt(format_args!("{}", loc.line()))
                .map_err(Error::FormatRecord)?;
        }
        Ok(())
    }
}

/// A pattern that writes the source column into the output. Example: `20`.
///
/// This pattern corresponds to `{%}` or `{column}` in the pattern template
/// string.
#[derive(Default)]
pub struct SourceColumn;

impl SourceColumn {
    /// Create a new `SourceColumn` pattern.
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for SourceColumn {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        if let Some(loc) = record.source_location() {
            dest.write_fmt(format_args!("{}", loc.column()))
                .map_err(Error::FormatRecord)?;
        }
        Ok(())
    }
}