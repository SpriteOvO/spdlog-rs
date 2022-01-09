//! Provides stuff related to source location

use std::path;

/// Represents a location in source code.
#[derive(Clone, Hash, Debug)]
pub struct SourceLocation {
    module_path: &'static str,
    file: &'static str,
    line: u32,
    column: u32,
}

impl SourceLocation {
    /// Constructs a `SourceLocation`.
    ///
    /// Users should usually use macro [`source_location_current`] to construct
    /// it.
    pub fn new(module_path: &'static str, file: &'static str, line: u32, column: u32) -> Self {
        Self {
            module_path,
            file,
            line,
            column,
        }
    }

    /// The module path.
    pub fn module_path(&self) -> &'static str {
        self.module_path
    }

    /// The source file.
    ///
    /// It returns a string slice like this: `src/main.rs`
    pub fn file(&self) -> &'static str {
        self.file
    }

    /// The source file name.
    ///
    /// It returns a string slice like this: `main.rs`
    pub fn file_name(&self) -> &'static str {
        if let Some(index) = self.file.rfind(path::MAIN_SEPARATOR) {
            &self.file[index + 1..]
        } else {
            self.file
        }
    }

    /// The line number in the source file.
    pub fn line(&self) -> u32 {
        self.line
    }

    /// The column number in the source file.
    pub fn column(&self) -> u32 {
        self.column
    }
}

/// Constructs a [`SourceLocation`] with current source location.
///
/// Returns `None` if the feature `source_location` is not enabled.
///
/// # Example
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
        Some($crate::SourceLocation::new(
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
