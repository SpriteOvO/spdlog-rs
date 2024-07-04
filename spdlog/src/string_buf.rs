/// A string buffer type.
///
/// Used at [`Formatter`].
///
/// By default, it is an alias for [`String`], if feature `flexible-string` is
/// enabled, an internal type `FlexibleString` will be used.
///
/// `FlexibleString` has a fixed stack buffer of 250 bytes, and upgrades to
/// [`String`] when more space is needed. It provides APIs that are as
/// consistent as possible with [`String`], but some APIs are not yet
/// implemented or cannot be implemented.
///
/// # Warning
///
/// `FlexibleString` can improve performance as it avoids memory allocation when
/// formatting records as much as possible, however it contains unsafe code.
///
/// [`Sink`]: crate::sink::Sink
/// [`Formatter`]: crate::formatter::Formatter
pub type StringBuf = StringBufInner;

use cfg_if::cfg_if;

// Users should not use the following types directly.

cfg_if! {
    if #[cfg(feature = "flexible-string")] {
        // pub for hide type alias in doc
        #[doc(hidden)]
        pub type StringBufInner = flexible_string::FlexibleString<STACK_SIZE>;
    } else {
        // same as above
        #[doc(hidden)]
        pub type StringBufInner = String;
    }
}

#[allow(dead_code)]
pub(crate) const STACK_SIZE: usize = 256;
#[allow(dead_code)]
pub(crate) const RESERVE_SIZE: usize = STACK_SIZE / 2;
