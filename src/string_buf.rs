//! Provides a string buffer type for sinks and formatters.

/// A string buffer type.
///
/// Used in [`Sink`] s and [`Formatter`] s.
///
/// # Todo
///
/// It is currently just an alias of [`String`], which can improve performance
/// if we implement it as a flexible stack + heap string buffer.
///
/// [`Sink`]: crate::sink::Sink
/// [`Formatter`]: crate::formatter::Formatter
pub type StringBuf = String;
