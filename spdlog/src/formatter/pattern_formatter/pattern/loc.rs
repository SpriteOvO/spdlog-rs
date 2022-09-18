use std::fmt::Write;

use crate::{
    formatter::pattern_formatter::{Pattern, PatternContext},
    Error, Record, StringBuf,
};

/// A pattern that writes the source file, line and column of a log record into
/// the output. Example: `main.rs:30:20`.
#[derive(Default, Clone)]
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
            write!(dest, "{}:{}", loc.file_name(), loc.line()).map_err(Error::FormatRecord)?;
        }
        Ok(())
    }
}

/// A pattern that writes the source file basename into the output. Example:
/// `main.rs`.
#[derive(Default, Clone)]
pub struct SourceFilename;

impl SourceFilename {
    /// Create a new `SourceFilename` pattern.
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for SourceFilename {
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
#[derive(Default, Clone)]
pub struct SourceFile;

impl SourceFile {
    /// Create a new `SourceFile` pattern.
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for SourceFile {
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
#[derive(Default, Clone)]
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
#[derive(Default, Clone)]
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
