use std::fmt::Write;

use crate::{
    formatter::pattern_formatter::{Pattern, PatternContext},
    Error, Record, StringBuf,
};

/// A pattern that writes the source file, line and column of a log record into
/// the output. Example: `path/to/main.rs:30`.
#[derive(Clone, Default)]
pub struct Source;

impl Pattern for Source {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        if let Some(loc) = record.source_location() {
            (|| {
                dest.write_str(loc.file())?;
                dest.write_char(':')?;
                dest.write_str(&numtoa::BaseN::<10>::u32(loc.line()))
            })()
            .map_err(Error::FormatRecord)?;
        }
        Ok(())
    }
}

/// A pattern that writes the source file basename into the output. Example:
/// `main.rs`.
#[derive(Clone, Default)]
pub struct SourceFilename;

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
#[derive(Clone, Default)]
pub struct SourceFile;

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
#[derive(Clone, Default)]
pub struct SourceLine;

impl Pattern for SourceLine {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        if let Some(loc) = record.source_location() {
            dest.write_str(&numtoa::BaseN::<10>::u32(loc.line()))
                .map_err(Error::FormatRecord)?;
        }
        Ok(())
    }
}

/// A pattern that writes the source column into the output. Example: `20`.
#[derive(Clone, Default)]
pub struct SourceColumn;

impl Pattern for SourceColumn {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        if let Some(loc) = record.source_location() {
            dest.write_str(&numtoa::BaseN::<10>::u32(loc.column()))
                .map_err(Error::FormatRecord)?;
        }
        Ok(())
    }
}

/// A pattern that writes the source module path into the output. Example:
/// `mod::path`
#[derive(Clone, Default)]
pub struct SourceModulePath;

impl Pattern for SourceModulePath {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        if let Some(loc) = record.source_location() {
            dest.write_str(loc.module_path())
                .map_err(Error::FormatRecord)?;
        }
        Ok(())
    }
}
