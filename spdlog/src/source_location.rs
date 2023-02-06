//! Provides stuff related to source location.

use std::path;

/// Represents a location in source code.
///
/// Usually users don't need to construct it manually, but if you do, use macro
/// [`source_location_current`].
///
/// [`source_location_current`]: crate::source_location_current
#[derive(Clone, Hash, Debug)]
pub struct SourceLocation {
    module_path: &'static str,
    file: &'static str,
    line: u32,
    column: u32,
}

impl SourceLocation {
    // Usually users don't need to construct it manually, but if you do, use macro
    // [`source_location_current`].
    #[doc(hidden)]
    #[must_use]
    pub fn __new(module_path: &'static str, file: &'static str, line: u32, column: u32) -> Self {
        Self {
            module_path,
            file,
            line,
            column,
        }
    }

    /// Gets the module path.
    #[must_use]
    pub fn module_path(&self) -> &'static str {
        self.module_path
    }

    /// Gets the source file.
    ///
    /// It returns a string slice like this: `src/main.rs`
    #[must_use]
    pub fn file(&self) -> &'static str {
        self.file
    }

    /// Gets the source file name.
    ///
    /// It returns a string slice like this: `main.rs`
    #[must_use]
    pub fn file_name(&self) -> &'static str {
        if let Some(index) = self.file.rfind(path::MAIN_SEPARATOR) {
            &self.file[index + 1..]
        } else {
            self.file
        }
    }

    /// Gets the line number in the source file.
    #[must_use]
    pub fn line(&self) -> u32 {
        self.line
    }

    /// Gets the column number in the source file.
    #[must_use]
    pub fn column(&self) -> u32 {
        self.column
    }

    #[cfg(feature = "log")]
    #[must_use]
    pub(crate) fn from_log_crate_record(record: &log::Record) -> Option<Self> {
        let (module_path, file, line) = (
            record.module_path_static(),
            record.file_static(),
            record.line(),
        );

        match (module_path, file, line) {
            (None, None, None) => None,
            _ => Some(Self {
                module_path: module_path.unwrap_or(""),
                file: file.unwrap_or(""),
                line: line.unwrap_or(0),
                column: 0,
            }),
        }
    }
}

/// Constructs a [`SourceLocation`] with current source location.
///
/// Returns `None` if the feature `source-location` is not enabled.
///
/// # Examples
///
/// ```
/// use spdlog::{SourceLocation, source_location_current};
///
/// let source_location: Option<SourceLocation> = source_location_current!();
/// ```
#[macro_export]
macro_rules! source_location_current {
    () => {
        $crate::__private_source_location_current_inner!()
    };
}

#[macro_export]
#[doc(hidden)]
#[cfg(feature = "source-location")]
macro_rules! __private_source_location_current_inner {
    () => {
        Some($crate::SourceLocation::__new(
            module_path!(),
            file!(),
            line!(),
            column!(),
        ))
    };
}

#[macro_export]
#[doc(hidden)]
#[cfg(not(feature = "source-location"))]
macro_rules! __private_source_location_current_inner {
    () => {
        None
    };
}
