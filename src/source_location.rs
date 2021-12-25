//! Provides stuff related to source location

/// Represents a location in source code.
#[derive(Clone, Hash, Debug)]
pub struct SourceLocation {
    module_path: &'static str,
    file: &'static str,
    line: u32,
}

impl SourceLocation {
    /// Constructs a `SourceLocation`.
    ///
    /// Users should usually use macro [`source_location_current`] to construct
    /// it.
    pub fn new(module_path: &'static str, file: &'static str, line: u32) -> Self {
        Self {
            module_path,
            file,
            line,
        }
    }

    /// The module path.
    pub fn module_path(&self) -> &'static str {
        self.module_path
    }

    /// The source file.
    pub fn file(&self) -> &'static str {
        self.file
    }

    /// The line in the source file.
    pub fn line(&self) -> u32 {
        self.line
    }
}

/// Constructs a [`SourceLocation`] with current source location.
///
/// # Example
///
/// ```
/// use spdlog::{SourceLocation, source_location_current};
///
/// let source_location: SourceLocation = source_location_current!();
/// ```
#[macro_export]
macro_rules! source_location_current {
    () => {
        $crate::SourceLocation::new(module_path!(), file!(), line!())
    };
}
