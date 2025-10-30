/// A string buffer type.
///
/// Used at [`Formatter`].
///
/// By default, it is an alias for [`String`], if feature `flexible-string` is
/// enabled, an internal type `FlexibleString` will be used.
///
/// `FlexibleString` has a fixed stack buffer of 256 bytes, and upgrades to
/// [`String`] when more space is needed. It provides APIs that are as
/// consistent as possible with [`String`], but some APIs are not yet
/// implemented or not possible to be implemented.
///
/// <div class="warning">
///
/// `FlexibleString` can improve performance as it avoids memory allocation when
/// formatting records as much as possible, however it contains unsafe code that
/// has not been strictly reviewed.
///
/// </div>
///
/// [`Sink`]: crate::sink::Sink
/// [`Formatter`]: crate::formatter::Formatter
pub type StringBuf = StringBufInner;

// Users should not use the following types directly.

// pub for hide type alias in doc
#[doc(hidden)]
#[cfg(feature = "flexible-string")]
pub type StringBufInner = flexible_string::FlexibleString<STACK_SIZE>;
// same as above
#[doc(hidden)]
#[cfg(not(feature = "flexible-string"))]
pub type StringBufInner = String;

#[allow(dead_code)]
pub(crate) const STACK_SIZE: usize = 256;
#[allow(dead_code)]
pub(crate) const RESERVE_SIZE: usize = STACK_SIZE / 2;
