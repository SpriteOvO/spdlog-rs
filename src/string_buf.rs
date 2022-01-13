//! Provides a string buffer type for sinks and formatters.

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
/// # Warnings
///
/// `FlexibleString` can improve performance as it avoids memory allocation when
/// formatting records (most log messages do not exceed 250 bytes), however it
/// contains unsafe code.
///
/// [`Sink`]: crate::sink::Sink
/// [`Formatter`]: crate::formatter::Formatter
pub type StringBuf = StringBufInner;

use cfg_if::cfg_if;

// Users should not use the following types directly.

cfg_if! {
    if #[cfg(feature = "flexible-string")] {
        // for integration tests
        #[doc(hidden)]
        pub use inner::FlexibleString;
        // for hide type alias in doc
        #[doc(hidden)]
        pub type StringBufInner = inner::FlexibleString<250>;
    } else {
        // same as above
        #[doc(hidden)]
        pub type StringBufInner = String;
    }
}

mod inner {
    use super::*;

    cfg_if! {
        if #[cfg(feature = "flexible-string")] {
            use std::{fmt, mem::MaybeUninit, ops, ptr, slice, str, string};

            // The following implementations are referenced from :
            // https://github.com/rust-lang/rust/blob/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/alloc/src/string.rs

            /// A possible error value when converting a `FlexibleString` from a UTF-8 byte vector.
            ///
            /// This type is the error type for the [`from_utf8`] method on [`FlexibleString`]. It
            /// is designed in such a way to carefully avoid reallocations: the
            /// [`into_bytes`] method will give back the byte vector that was used in the
            /// conversion attempt.
            ///
            /// [`from_utf8`]: FlexibleString::from_utf8
            /// [`into_bytes`]: FromUtf8Error::into_bytes
            ///
            /// The [`Utf8Error`] type provided by [`std::str`] represents an error that may
            /// occur when converting a slice of [`u8`]s to a [`&str`]. In this sense, it's
            /// an analogue to `FromUtf8Error`, and you can get one from a `FromUtf8Error`
            /// through the [`utf8_error`] method.
            ///
            /// [`Utf8Error`]: str::Utf8Error "std::str::Utf8Error"
            /// [`std::str`]: core::str "std::str"
            /// [`&str`]: prim@str "&str"
            /// [`utf8_error`]: FromUtf8Error::utf8_error
            ///
            /// # Examples
            ///
            /// Basic usage:
            ///
            /// ```
            /// use spdlog::string_buf::FlexibleString;
            ///
            /// // some invalid bytes, in a vector
            /// let bytes = vec![0, 159];
            ///
            /// let value = FlexibleString::<200>::from_utf8(bytes);
            ///
            /// assert!(value.is_err());
            /// assert_eq!(vec![0, 159], value.unwrap_err().into_bytes());
            /// ```
            // We implement it manually because the fields in std are private.
            #[derive(Clone, Debug, PartialEq, Eq)]
            pub struct FromUtf8Error {
                bytes: Vec<u8>,
                error: str::Utf8Error,
            }

            /// A flexible string, which first uses a `CAPACITY` sized stack buffer and
            /// switches to a heap buffer when more space is needed.
            #[derive(Clone)]
            pub struct FlexibleString<const CAPACITY: usize>(FlexibleStringInner<CAPACITY>);

            #[derive(Clone)]
            enum FlexibleStringInner<const CAPACITY: usize> {
                Stack(StackString<CAPACITY>),
                Heap(String),
            }

            macro_rules! common_methods_inner {
                (NEVER_UPGRADE => fn_name:$fn_name:ident, generics:$(<$generics:tt>)?, self_qual:$([$($self_qual:tt)*])?, self:$self:ident, arg_name:[$($arg_name:ident),*]) => {
                    match $($($self_qual)*)? $self.0 {
                        FlexibleStringInner::Stack(s) => s.$fn_name $(::<$generics>)? ($($arg_name),*),
                        FlexibleStringInner::Heap(h) => h.$fn_name $(::<$generics>)? ($($arg_name),*),
                    }
                };
                (MAYBE_UPGRADE => fn_name:$fn_name:ident, generics:$(<$generics:tt>)?, self_qual:$([$($self_qual:tt)*])?, self:$self:ident, arg_name:[$($arg_name:ident),*]) => {
                    match $($($self_qual)*)? $self.0 {
                        FlexibleStringInner::Stack(s) => {
                            match s.$fn_name $(::<$generics>)? ($($arg_name),*) {
                                Err(capacity) => {
                                    let mut heap = s.to_heap(capacity);
                                    let res = heap.$fn_name($($arg_name),*);
                                    *$self = Self(FlexibleStringInner::Heap(heap));
                                    res
                                },
                                Ok(res) => res
                            }
                        },
                        FlexibleStringInner::Heap(h) => h.$fn_name $(::<$generics>)? ($($arg_name),*),
                    }
                };
            }

            macro_rules! common_methods {
                () => {};
                ( $(#[$attr:meta])*
                $upgrade:ident => $vis:vis $([$($qual:tt)*])? fn $fn_name:ident $(<$generics:tt>)? ( $([$($self_qual:tt)*])? $self:ident $(,)? $( $arg_name:ident: $arg_type:ty),* ) $(-> $ret:ty)?
                $(where $($where_ty:ty: $where_bound:path)*)?; $($tail:tt)* ) => {
                    $(#[$attr])*
                    #[inline]
                    $vis $($($qual)*)? fn $fn_name $(<$generics>)? ($($($self_qual)*)? $self, $($arg_name: $arg_type),*) $(-> $ret)?
                    $(where $($where_ty: $where_bound)*)? {
                        common_methods_inner!($upgrade => fn_name:$fn_name, generics:$(<$generics>)?, self_qual:$([$($self_qual)*])?, self:$self, arg_name:[$($arg_name),*])
                    }
                    common_methods!($($tail)*);
                };
            }

            impl<const CAPACITY: usize> FlexibleString<CAPACITY> {
                /// Creates a new empty `FlexibleString`.
                ///
                /// # Examples
                ///
                /// Basic usage:
                ///
                /// ```
                /// use spdlog::string_buf::FlexibleString;
                ///
                /// let s = FlexibleString::<200>::new();
                /// ```
                #[inline]
                #[must_use]
                pub fn new() -> Self {
                    Self(FlexibleStringInner::Stack(StackString::new()))
                }

                /// Creates a new empty `FlexibleString` with a particular capacity.
                ///
                /// # Examples
                ///
                /// Basic usage:
                ///
                /// ```
                /// use spdlog::string_buf::FlexibleString;
                ///
                /// let mut s = FlexibleString::<200>::with_capacity(10);
                ///
                /// // The FlexibleString contains no chars, even though it has capacity for more
                /// assert_eq!(s.len(), 0);
                ///
                /// // These are all done without reallocating...
                /// let cap = s.capacity();
                /// for _ in 0..10 {
                ///     s.push('a');
                /// }
                ///
                /// assert_eq!(s.capacity(), cap);
                ///
                /// // ...but this may make the string reallocate
                /// s.push('a');
                /// ```
                #[inline]
                #[must_use]
                pub fn with_capacity(capacity: usize) -> Self {
                    if capacity > CAPACITY {
                        Self(FlexibleStringInner::Heap(String::with_capacity(capacity)))
                    } else {
                        Self(FlexibleStringInner::Stack(StackString::new()))
                    }
                }

                /// Converts a vector of bytes to a `FlexibleString`.
                ///
                /// # Examples
                ///
                /// Basic usage:
                ///
                /// ```
                /// use spdlog::string_buf::FlexibleString;
                ///
                /// // some bytes, in a vector
                /// let sparkle_heart = vec![240, 159, 146, 150];
                ///
                /// // We know these bytes are valid, so we'll use `unwrap()`.
                /// let sparkle_heart = FlexibleString::<200>::from_utf8(sparkle_heart).unwrap();
                ///
                /// assert_eq!("💖", sparkle_heart);
                /// ```
                ///
                /// Incorrect bytes:
                ///
                /// ```
                /// use spdlog::string_buf::FlexibleString;
                ///
                /// // some invalid bytes, in a vector
                /// let sparkle_heart = vec![0, 159, 146, 150];
                ///
                /// assert!(FlexibleString::<200>::from_utf8(sparkle_heart).is_err());
                /// ```
                #[inline]
                pub fn from_utf8(vec: Vec<u8>) -> Result<Self, FromUtf8Error> {
                    if vec.len() > CAPACITY {
                        String::from_utf8(vec)
                            .map(|s| Self(FlexibleStringInner::Heap(s)))
                            .map_err(|err| err.into())
                    } else {
                        unsafe { StackString::from_utf8_only_len_unchecked(vec) }
                            .map(|s| Self(FlexibleStringInner::Stack(s)))
                    }
                }

                // Unimplemented.
                // pub fn from_utf8_lossy(v: &[u8]) -> Cow<'_, str> {}

                // Unimplemented.
                // pub fn from_utf16(v: &[u16]) -> Result<Self, FromUtf16Error> {}

                // Unimplemented.
                // pub fn from_utf16_lossy(v: &[u16]) -> Self {}

                // Unimplemented since unstable.
                // pub fn into_raw_parts(self) -> (*mut u8, usize, usize)

                // Unimplemented since unstable.
                // pub unsafe fn from_raw_parts(buf: *mut u8, length: usize, capacity: usize) -> String

                /// Converts a vector of bytes to a `FlexibleString` without checking that the
                /// string contains valid UTF-8.
                ///
                /// # Safety
                ///
                /// See the documentation of [`String::from_utf8_unchecked`].
                ///
                /// # Examples
                ///
                /// Basic usage:
                ///
                /// ```
                /// use spdlog::string_buf::FlexibleString;
                ///
                /// // some bytes, in a vector
                /// let sparkle_heart = vec![240, 159, 146, 150];
                ///
                /// let sparkle_heart = unsafe {
                ///     FlexibleString::<200>::from_utf8_unchecked(sparkle_heart)
                /// };
                ///
                /// assert_eq!("💖", sparkle_heart);
                /// ```
                #[inline]
                #[must_use]
                pub unsafe fn from_utf8_unchecked(bytes: Vec<u8>) -> Self {
                    if bytes.len() > CAPACITY {
                        Self(FlexibleStringInner::Heap(String::from_utf8_unchecked(
                            bytes,
                        )))
                    } else {
                        Self(FlexibleStringInner::Stack(
                            StackString::from_utf8_unchecked(bytes),
                        ))
                    }
                }

                common_methods! {
                    /// Converts a `FlexibleString` into a byte vector.
                    ///
                    /// # Examples
                    ///
                    /// Basic usage:
                    ///
                    /// ```
                    /// use spdlog::string_buf::FlexibleString;
                    ///
                    /// let s = FlexibleString::<200>::from("hello");
                    /// let bytes = s.into_bytes();
                    ///
                    /// assert_eq!(&[104, 101, 108, 108, 111][..], &bytes[..]);
                    /// ```
                    #[must_use = "`self` will be dropped if the result is not used"]
                    NEVER_UPGRADE => pub fn into_bytes(self) -> Vec<u8>;

                    /// Extracts a string slice containing the entire `FlexibleString`.
                    ///
                    /// # Examples
                    ///
                    /// Basic usage:
                    ///
                    /// ```
                    /// use spdlog::string_buf::FlexibleString;
                    ///
                    /// let s = FlexibleString::<200>::from("foo");
                    ///
                    /// assert_eq!("foo", s.as_str());
                    /// ```
                    #[must_use]
                    NEVER_UPGRADE => pub fn as_str([&]self) -> &str;

                    /// Converts a `FlexibleString` into a mutable string slice.
                    ///
                    /// # Examples
                    ///
                    /// Basic usage:
                    ///
                    /// ```
                    /// use spdlog::string_buf::FlexibleString;
                    ///
                    /// let mut s = FlexibleString::<200>::from("foobar");
                    /// let s_mut_str = s.as_mut_str();
                    ///
                    /// s_mut_str.make_ascii_uppercase();
                    ///
                    /// assert_eq!("FOOBAR", s_mut_str);
                    /// ```
                    #[must_use]
                    NEVER_UPGRADE => pub fn as_mut_str([&mut]self) -> &mut str;

                    /// Appends a given string slice onto the end of this `FlexibleString`.
                    ///
                    /// # Examples
                    ///
                    /// Basic usage:
                    ///
                    /// ```
                    /// use spdlog::string_buf::FlexibleString;
                    ///
                    /// let mut s = FlexibleString::<200>::from("foo");
                    ///
                    /// s.push_str("bar");
                    ///
                    /// assert_eq!("foobar", s);
                    /// ```
                    MAYBE_UPGRADE => pub fn push_str([&mut]self, string: &str);

                    // Unimplemented since unstable.
                    // pub fn extend_from_within<R>(&mut self, src: R)

                    /// Returns this `FlexibleString`'s capacity, in bytes.
                    ///
                    /// # Examples
                    ///
                    /// Basic usage:
                    ///
                    /// ```
                    /// use spdlog::string_buf::FlexibleString;
                    ///
                    /// let s = FlexibleString::<200>::with_capacity(10);
                    ///
                    /// assert!(s.capacity() >= 10);
                    /// ```
                    NEVER_UPGRADE => pub fn capacity([&]self) -> usize;

                    /// Ensures that this `FlexibleString`'s capacity is at least `additional` bytes
                    /// larger than its length.
                    ///
                    /// # Examples
                    ///
                    /// Basic usage:
                    ///
                    /// ```
                    /// use spdlog::string_buf::FlexibleString;
                    ///
                    /// let mut s = FlexibleString::<200>::new();
                    ///
                    /// s.reserve(10);
                    ///
                    /// assert!(s.capacity() >= 10);
                    /// ```
                    ///
                    /// This might not actually increase the capacity:
                    ///
                    /// ```
                    /// use spdlog::string_buf::FlexibleString;
                    ///
                    /// let mut s = FlexibleString::<200>::with_capacity(10);
                    /// s.push('a');
                    /// s.push('b');
                    ///
                    /// // s now has a length of 2 and a capacity of 10
                    /// assert_eq!(2, s.len());
                    /// assert!(s.capacity() >= 10);
                    ///
                    /// // Since we already have an extra 8 capacity, calling this...
                    /// s.reserve(8);
                    ///
                    /// // ... doesn't actually increase.
                    /// assert!(s.capacity() >= 10);
                    /// ```
                    MAYBE_UPGRADE => pub fn reserve([&mut]self, additional: usize);

                    /// Ensures that this `FlexibleString`'s capacity is `additional` bytes
                    /// larger than its length.
                    ///
                    /// # Examples
                    ///
                    /// Basic usage:
                    ///
                    /// ```
                    /// use spdlog::string_buf::FlexibleString;
                    ///
                    /// let mut s = FlexibleString::<200>::new();
                    ///
                    /// s.reserve_exact(10);
                    ///
                    /// assert!(s.capacity() >= 10);
                    /// ```
                    ///
                    /// This might not actually increase the capacity:
                    ///
                    /// ```
                    /// use spdlog::string_buf::FlexibleString;
                    ///
                    /// let mut s = FlexibleString::<200>::with_capacity(10);
                    /// s.push('a');
                    /// s.push('b');
                    ///
                    /// // s now has a length of 2 and a capacity of 10
                    /// assert_eq!(2, s.len());
                    /// assert!(s.capacity() >= 10);
                    ///
                    /// // Since we already have an extra 8 capacity, calling this...
                    /// s.reserve_exact(8);
                    ///
                    /// // ... doesn't actually increase.
                    /// assert!(s.capacity() >= 10);
                    /// ```
                    MAYBE_UPGRADE => pub fn reserve_exact([&mut]self, additional: usize);

                    // Unimplemented since `TryReserveErrorKind` is currently unstable.
                    //
                    // /// Tries to reserve capacity for at least `additional` more elements to be inserted
                    // /// in the given `FlexibleString`. The collection may reserve more space to avoid
                    // /// frequent reallocations. After calling `reserve`, capacity will be
                    // /// greater than or equal to `self.len() + additional`. Does nothing if
                    // /// capacity is already sufficient.
                    // ///
                    // /// # Examples
                    // ///
                    // /// ```
                    // /// use spdlog::string_buf::FlexibleString;
                    // ///
                    // /// use std::collections::TryReserveError;
                    // ///
                    // /// fn process_data(data: &str) -> Result<FlexibleString::<200>, TryReserveError> {
                    // ///     let mut output = FlexibleString::<200>::new();
                    // ///
                    // ///     // Pre-reserve the memory, exiting if we can't
                    // ///     output.try_reserve(data.len())?;
                    // ///
                    // ///     // Now we know this can't OOM in the middle of our complex work
                    // ///     output.push_str(data);
                    // ///
                    // ///     Ok(output)
                    // /// }
                    // /// # process_data("rust").expect("why is the test harness OOMing on 4 bytes?");
                    // /// ```
                    // MAYBE_UPGRADE => pub fn try_reserve([&mut]self, additional: usize) -> Result<(), TryReserveError>;
                    //
                    // /// Tries to reserve the minimum capacity for exactly `additional` more elements to
                    // /// be inserted in the given `FlexibleString`. After calling `reserve_exact`,
                    // /// capacity will be greater than or equal to `self.len() + additional`.
                    // /// Does nothing if the capacity is already sufficient.
                    // ///
                    // /// # Examples
                    // ///
                    // /// ```
                    // /// use spdlog::string_buf::FlexibleString;
                    // ///
                    // /// use std::collections::TryReserveError;
                    // ///
                    // /// fn process_data(data: &str) -> Result<FlexibleString::<200>, TryReserveError> {
                    // ///     let mut output = FlexibleString::<200>::new();
                    // ///
                    // ///     // Pre-reserve the memory, exiting if we can't
                    // ///     output.try_reserve_exact(data.len())?;
                    // ///
                    // ///     // Now we know this can't OOM in the middle of our complex work
                    // ///     output.push_str(data);
                    // ///
                    // ///     Ok(output)
                    // /// }
                    // /// # process_data("rust").expect("why is the test harness OOMing on 4 bytes?");
                    // /// ```
                    // MAYBE_UPGRADE => pub fn try_reserve_exact([&mut]self, additional: usize) -> Result<(), TryReserveError>;

                    // Cannot be implemented because the stack string cannot ensure the requirement.
                    // pub fn shrink_to_fit(&mut self);

                    /// Shrinks the capacity of this `FlexibleString` with a lower bound.
                    ///
                    /// # Examples
                    ///
                    /// ```
                    /// use spdlog::string_buf::FlexibleString;
                    ///
                    /// let mut s = FlexibleString::<200>::from("foo");
                    ///
                    /// s.reserve(100);
                    /// assert!(s.capacity() >= 100);
                    ///
                    /// s.shrink_to(10);
                    /// assert!(s.capacity() >= 10);
                    /// s.shrink_to(0);
                    /// assert!(s.capacity() >= 3);
                    /// ```
                    NEVER_UPGRADE => pub fn shrink_to([&mut]self, min_capacity: usize);

                    /// Appends the given [`char`] to the end of this `FlexibleString`.
                    ///
                    /// # Examples
                    ///
                    /// Basic usage:
                    ///
                    /// ```
                    /// use spdlog::string_buf::FlexibleString;
                    ///
                    /// let mut s = FlexibleString::<200>::from("abc");
                    ///
                    /// s.push('1');
                    /// s.push('2');
                    /// s.push('3');
                    ///
                    /// assert_eq!("abc123", s);
                    /// ```
                    MAYBE_UPGRADE => pub fn push([&mut]self, ch: char);

                    /// Returns a byte slice of this `FlexibleString`'s contents.
                    ///
                    /// # Examples
                    ///
                    /// Basic usage:
                    ///
                    /// ```
                    /// use spdlog::string_buf::FlexibleString;
                    ///
                    /// let s = FlexibleString::<200>::from("hello");
                    ///
                    /// assert_eq!(&[104, 101, 108, 108, 111], s.as_bytes());
                    /// ```
                    #[must_use]
                    NEVER_UPGRADE => pub fn as_bytes([&]self) -> &[u8];

                    /// Shortens this `FlexibleString` to the specified length.
                    ///
                    /// If `new_len` is greater than the string's current length, this has no
                    /// effect.
                    ///
                    /// Note that this method has no effect on the allocated capacity
                    /// of the string
                    ///
                    /// # Panics
                    ///
                    /// Panics if `new_len` does not lie on a [`char`] boundary.
                    ///
                    /// # Examples
                    ///
                    /// Basic usage:
                    ///
                    /// ```
                    /// use spdlog::string_buf::FlexibleString;
                    ///
                    /// let mut s = FlexibleString::<200>::from("hello");
                    ///
                    /// s.truncate(2);
                    ///
                    /// assert_eq!("he", s);
                    /// ```
                    NEVER_UPGRADE => pub fn truncate([&mut]self, new_len: usize);

                    /// Removes the last character from the string buffer and returns it.
                    ///
                    /// Returns [`None`] if this `FlexibleString` is empty.
                    ///
                    /// # Examples
                    ///
                    /// Basic usage:
                    ///
                    /// ```
                    /// use spdlog::string_buf::FlexibleString;
                    ///
                    /// let mut s = FlexibleString::<200>::from("foo");
                    ///
                    /// assert_eq!(s.pop(), Some('o'));
                    /// assert_eq!(s.pop(), Some('o'));
                    /// assert_eq!(s.pop(), Some('f'));
                    ///
                    /// assert_eq!(s.pop(), None);
                    /// ```
                    NEVER_UPGRADE => pub fn pop([&mut]self) -> Option<char>;

                    /// Removes a [`char`] from this `FlexibleString` at a byte position and returns it.
                    ///
                    /// This is an *O*(*n*) operation, as it requires copying every element in the
                    /// buffer.
                    ///
                    /// # Panics
                    ///
                    /// Panics if `idx` is larger than or equal to the `FlexibleString`'s length,
                    /// or if it does not lie on a [`char`] boundary.
                    ///
                    /// # Examples
                    ///
                    /// Basic usage:
                    ///
                    /// ```
                    /// use spdlog::string_buf::FlexibleString;
                    ///
                    /// let mut s = FlexibleString::<200>::from("foo");
                    ///
                    /// assert_eq!(s.remove(0), 'f');
                    /// assert_eq!(s.remove(1), 'o');
                    /// assert_eq!(s.remove(0), 'o');
                    /// ```
                    NEVER_UPGRADE => pub fn remove([&mut]self, idx: usize) -> char;

                    // Unimplemented.
                    // ...

                    /// Inserts a character into this `FlexibleString` at a byte position.
                    ///
                    /// This is an *O*(*n*) operation as it requires copying every element in the
                    /// buffer.
                    ///
                    /// # Panics
                    ///
                    /// Panics if `idx` is larger than the `FlexibleString`'s length, or if it does not
                    /// lie on a [`char`] boundary.
                    ///
                    /// # Examples
                    ///
                    /// Basic usage:
                    ///
                    /// ```
                    /// use spdlog::string_buf::FlexibleString;
                    ///
                    /// let mut s = FlexibleString::<200>::with_capacity(3);
                    ///
                    /// s.insert(0, 'f');
                    /// s.insert(1, 'o');
                    /// s.insert(2, 'o');
                    ///
                    /// assert_eq!("foo", s);
                    /// ```
                    MAYBE_UPGRADE => pub fn insert([&mut]self, idx: usize, ch: char);

                    /// Inserts a string slice into this `FlexibleString` at a byte position.
                    ///
                    /// This is an *O*(*n*) operation as it requires copying every element in the
                    /// buffer.
                    ///
                    /// # Panics
                    ///
                    /// Panics if `idx` is larger than the `FlexibleString`'s length, or if it does not
                    /// lie on a [`char`] boundary.
                    ///
                    /// # Examples
                    ///
                    /// Basic usage:
                    ///
                    /// ```
                    /// use spdlog::string_buf::FlexibleString;
                    ///
                    /// let mut s = FlexibleString::<200>::from("bar");
                    ///
                    /// s.insert_str(0, "foo");
                    ///
                    /// assert_eq!("foobar", s);
                    /// ```
                    MAYBE_UPGRADE => pub fn insert_str([&mut]self, idx: usize, string: &str);

                    // Unimplemented.
                    // pub unsafe fn as_mut_vec(&mut self) -> &mut Vec<u8>

                    /// Returns the length of this `FlexibleString`, in bytes, not [`char`]s or
                    /// graphemes. In other words, it might not be what a human considers the
                    /// length of the string.
                    ///
                    /// # Examples
                    ///
                    /// Basic usage:
                    ///
                    /// ```
                    /// use spdlog::string_buf::FlexibleString;
                    ///
                    /// let a = FlexibleString::<200>::from("foo");
                    /// assert_eq!(a.len(), 3);
                    ///
                    /// let fancy_f = FlexibleString::<200>::from("ƒoo");
                    /// assert_eq!(fancy_f.len(), 4);
                    /// assert_eq!(fancy_f.chars().count(), 3);
                    /// ```
                    NEVER_UPGRADE => pub fn len([&]self) -> usize;

                    /// Returns `true` if this `FlexibleString` has a length of zero, and `false` otherwise.
                    ///
                    /// # Examples
                    ///
                    /// Basic usage:
                    ///
                    /// ```
                    /// use spdlog::string_buf::FlexibleString;
                    ///
                    /// let mut v = FlexibleString::<200>::new();
                    /// assert!(v.is_empty());
                    ///
                    /// v.push('a');
                    /// assert!(!v.is_empty());
                    /// ```
                    NEVER_UPGRADE => pub fn is_empty([&]self) -> bool;

                    // Unimplemented.
                    // pub fn split_off(&mut self, at: usize) -> String;

                    /// Truncates this `FlexibleString`, removing all contents.
                    ///
                    /// While this means the `FlexibleString` will have a length of zero, it does not
                    /// touch its capacity.
                    ///
                    /// # Examples
                    ///
                    /// Basic usage:
                    ///
                    /// ```
                    /// use spdlog::string_buf::FlexibleString;
                    ///
                    /// let mut s = FlexibleString::<200>::from("foo");
                    ///
                    /// s.clear();
                    ///
                    /// assert!(s.is_empty());
                    /// assert_eq!(0, s.len());
                    /// ```
                    NEVER_UPGRADE => pub fn clear([&mut]self);

                    // Unimplemented.
                    // pub fn drain<R>(&mut self, range: R) -> Drain<'_>;

                    // Unimplemented.
                    // ...
                }
            }

            impl<const CAPACITY: usize> Default for FlexibleString<CAPACITY> {
                #[inline]
                fn default() -> Self {
                    Self::new()
                }
            }

            macro_rules! impl_eq {
                ($lhs:ty, $rhs:ty) => {
                    #[allow(unused_lifetimes)]
                    impl<'a, 'b, const CAPACITY: usize> PartialEq<$rhs> for $lhs {
                        #[inline]
                        fn eq(&self, other: &$rhs) -> bool {
                            PartialEq::eq(&self[..], &other[..])
                        }
                    }

                    #[allow(unused_lifetimes)]
                    impl<'a, 'b, const CAPACITY: usize> PartialEq<$lhs> for $rhs {
                        #[inline]
                        fn eq(&self, other: &$lhs) -> bool {
                            PartialEq::eq(&self[..], &other[..])
                        }
                    }
                };
            }

            impl_eq! { FlexibleString<CAPACITY>, str }
            impl_eq! { FlexibleString<CAPACITY>, &'a str }
            impl_eq! { FlexibleString<CAPACITY>, String }

            #[allow(unused_lifetimes)]
            impl<const CAPACITY_LHS: usize, const CAPACITY_RHS: usize>
                PartialEq<FlexibleString<CAPACITY_RHS>> for FlexibleString<CAPACITY_LHS>
            {
                #[inline]
                fn eq(&self, other: &FlexibleString<CAPACITY_RHS>) -> bool {
                    PartialEq::eq(&self[..], &other[..])
                }
            }

            impl<const CAPACITY: usize> AsRef<str> for FlexibleString<CAPACITY> {
                #[inline]
                fn as_ref(&self) -> &str {
                    self
                }
            }

            impl<const CAPACITY: usize> AsMut<str> for FlexibleString<CAPACITY> {
                #[inline]
                fn as_mut(&mut self) -> &mut str {
                    self
                }
            }

            impl<const CAPACITY: usize> AsRef<[u8]> for FlexibleString<CAPACITY> {
                #[inline]
                fn as_ref(&self) -> &[u8] {
                    self.as_bytes()
                }
            }

            impl<const CAPACITY: usize> From<char> for FlexibleString<CAPACITY> {
                #[inline]
                fn from(c: char) -> Self {
                    FlexibleString::from(c.encode_utf8(&mut [0; 4]))
                }
            }

            impl<const CAPACITY: usize> From<&str> for FlexibleString<CAPACITY> {
                #[inline]
                fn from(s: &str) -> Self {
                    unsafe { FlexibleString::from_utf8_unchecked(s.as_bytes().to_owned()) }
                }
            }

            impl<const CAPACITY: usize> From<&mut str> for FlexibleString<CAPACITY> {
                #[inline]
                fn from(s: &mut str) -> Self {
                    (s as &str).into()
                }
            }

            impl<const CAPACITY: usize> From<String> for FlexibleString<CAPACITY> {
                #[inline]
                fn from(s: String) -> Self {
                    s.as_str().into()
                }
            }

            impl<'a, const CAPACITY: usize> FromIterator<&'a char> for FlexibleString<CAPACITY> {
                fn from_iter<I: IntoIterator<Item = &'a char>>(iter: I) -> Self {
                    let mut buf = FlexibleString::new();
                    buf.extend(iter);
                    buf
                }
            }

            impl<'a, const CAPACITY: usize> FromIterator<&'a str> for FlexibleString<CAPACITY> {
                fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
                    let mut buf = FlexibleString::new();
                    buf.extend(iter);
                    buf
                }
            }

            impl<const CAPACITY_LHS: usize, const CAPACITY_RHS: usize>
                FromIterator<FlexibleString<CAPACITY_RHS>> for FlexibleString<CAPACITY_LHS>
            {
                fn from_iter<I: IntoIterator<Item = FlexibleString<CAPACITY_RHS>>>(
                    iter: I,
                ) -> FlexibleString<CAPACITY_LHS> {
                    let mut buf = FlexibleString::new();
                    buf.extend(iter);
                    buf
                }
            }

            impl<const CAPACITY: usize> FromIterator<char> for FlexibleString<CAPACITY> {
                fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
                    let mut buf = FlexibleString::new();
                    buf.extend(iter);
                    buf
                }
            }

            impl<const CAPACITY: usize> FromIterator<Box<str>> for FlexibleString<CAPACITY> {
                fn from_iter<I: IntoIterator<Item = Box<str>>>(iter: I) -> Self {
                    let mut buf = FlexibleString::new();
                    buf.extend(iter);
                    buf
                }
            }

            impl<const CAPACITY: usize> Extend<char> for FlexibleString<CAPACITY> {
                fn extend<I: IntoIterator<Item = char>>(&mut self, iter: I) {
                    let iterator = iter.into_iter();
                    let (lower_bound, _) = iterator.size_hint();
                    self.reserve(lower_bound);
                    iterator.for_each(move |c| self.push(c));
                }

                // comment out these 2 functions since they are currently unstable.

                // #[inline]
                // fn extend_one(&mut self, c: char) {
                //     self.push(c);
                // }

                // #[inline]
                // fn extend_reserve(&mut self, additional: usize) {
                //     self.reserve(additional);
                // }
            }

            impl<'a, const CAPACITY: usize> Extend<&'a char> for FlexibleString<CAPACITY> {
                fn extend<I: IntoIterator<Item = &'a char>>(&mut self, iter: I) {
                    self.extend(iter.into_iter().cloned());
                }

                // comment out these 2 functions since they are currently unstable.

                // #[inline]
                // fn extend_one(&mut self, &c: &'a char) {
                //     self.push(c);
                // }

                // #[inline]
                // fn extend_reserve(&mut self, additional: usize) {
                //     self.reserve(additional);
                // }
            }

            impl<'a, const CAPACITY: usize> Extend<&'a str> for FlexibleString<CAPACITY> {
                fn extend<I: IntoIterator<Item = &'a str>>(&mut self, iter: I) {
                    iter.into_iter().for_each(move |s| self.push_str(s));
                }

                // comment out this function since it is currently unstable.

                // #[inline]
                // fn extend_one(&mut self, s: &'a str) {
                //     self.push_str(s);
                // }
            }

            impl<'a, const CAPACITY: usize> Extend<Box<str>> for FlexibleString<CAPACITY> {
                fn extend<I: IntoIterator<Item = Box<str>>>(&mut self, iter: I) {
                    iter.into_iter().for_each(move |s| self.push_str(&s));
                }
            }

            impl<const CAPACITY_LHS: usize, const CAPACITY_RHS: usize> Extend<FlexibleString<CAPACITY_RHS>>
                for FlexibleString<CAPACITY_LHS>
            {
                fn extend<I: IntoIterator<Item = FlexibleString<CAPACITY_RHS>>>(&mut self, iter: I) {
                    iter.into_iter().for_each(move |s| self.push_str(&s));
                }

                // comment out this function since it is currently unstable.

                // #[inline]
                // fn extend_one(&mut self, s: FlexibleString<CAPACITY_RHS>) {
                //     self.push_str(&s);
                // }
            }

            impl<const CAPACITY: usize> str::FromStr for FlexibleString<CAPACITY> {
                type Err = core::convert::Infallible;

                #[inline]
                fn from_str(s: &str) -> Result<FlexibleString<CAPACITY>, Self::Err> {
                    Ok(FlexibleString::from(s))
                }
            }

            impl<const CAPACITY: usize> ops::Add<&str> for FlexibleString<CAPACITY> {
                type Output = FlexibleString<CAPACITY>;

                #[inline]
                fn add(mut self, other: &str) -> Self {
                    self.push_str(other);
                    self
                }
            }

            impl<const CAPACITY: usize> ops::AddAssign<&str> for FlexibleString<CAPACITY> {
                #[inline]
                fn add_assign(&mut self, other: &str) {
                    self.push_str(other);
                }
            }

            impl<const CAPACITY: usize> ops::Deref for FlexibleString<CAPACITY> {
                type Target = str;

                common_methods! {
                    NEVER_UPGRADE => fn deref([&]self) -> &str;
                }
            }

            impl<const CAPACITY: usize> ops::DerefMut for FlexibleString<CAPACITY> {
                common_methods! {
                    NEVER_UPGRADE => fn deref_mut([&mut]self) -> &mut str;
                }
            }

            impl<const CAPACITY: usize> fmt::Write for FlexibleString<CAPACITY> {
                #[inline]
                fn write_str(&mut self, s: &str) -> fmt::Result {
                    self.push_str(s);
                    Ok(())
                }

                #[inline]
                fn write_char(&mut self, c: char) -> fmt::Result {
                    self.push(c);
                    Ok(())
                }
            }

            impl<const CAPACITY: usize> fmt::Display for FlexibleString<CAPACITY> {
                #[inline]
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    fmt::Display::fmt(&**self, f)
                }
            }

            impl<const CAPACITY: usize> fmt::Debug for FlexibleString<CAPACITY> {
                #[inline]
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    fmt::Debug::fmt(&**self, f)
                }
            }

            #[derive(Clone)]
            struct StackString<const CAPACITY: usize> {
                data: [MaybeUninit<u8>; CAPACITY],
                len: usize,
            }

            // Err(EstimatedCapacity) if the stack size is not enough, we must switch to use the heap.
            type StackStringResult<T> = Result<T, Option<usize>>;

            impl<const CAPACITY: usize> StackString<CAPACITY> {
                #[inline]
                fn new() -> Self {
                    Self {
                        data: unsafe { MaybeUninit::uninit().assume_init() },
                        len: 0,
                    }
                }

                #[inline]
                unsafe fn from_utf8_only_len_unchecked(vec: Vec<u8>) -> Result<Self, FromUtf8Error> {
                    match str::from_utf8(&vec) {
                        Ok(..) => Ok(Self::from_utf8_unchecked(vec)),
                        Err(e) => Err(FromUtf8Error {
                            bytes: vec,
                            error: e,
                        }),
                    }
                }

                #[inline]
                unsafe fn from_utf8_unchecked(bytes: Vec<u8>) -> Self {
                    let mut res = Self::new();
                    res.copy_append_unchecked(bytes.as_ptr(), bytes.len());
                    res
                }

                #[inline]
                fn into_bytes(self) -> Vec<u8> {
                    self.as_bytes().into()
                }

                #[inline]
                fn as_str(&self) -> &str {
                    self
                }

                #[inline]
                fn as_mut_str(&mut self) -> &mut str {
                    self
                }

                #[inline]
                fn push_str(&mut self, string: &str) -> StackStringResult<()> {
                    let len_needed = self.len + string.len();
                    if len_needed > CAPACITY {
                        Err(Some(len_needed))
                    } else {
                        unsafe {
                            self.copy_append_unchecked(string.as_ptr(), string.len());
                        }
                        Ok(())
                    }
                }

                #[inline]
                const fn capacity(&self) -> usize {
                    CAPACITY
                }

                #[inline]
                fn reserve(&mut self, additional: usize) -> StackStringResult<()> {
                    let cap_at_least = self.len + additional;
                    if cap_at_least > CAPACITY {
                        Err(Some(cap_at_least))
                    } else {
                        Ok(())
                    }
                }

                #[inline]
                fn reserve_exact(&mut self, additional: usize) -> StackStringResult<()> {
                    self.reserve(additional)
                }

                // comment out these 2 functions since `TryReserveErrorKind` is currently unstable.
                //
                // fn try_reserve(&mut self, additional: usize) -> StackStringResult<Result<(), TryReserveError>> {
                //     let cap_at_least = self.len + additional;
                //     if cap_at_least > CAPACITY {
                //         Err(Some(cap_at_least))
                //     } else {
                //         Ok(Ok(()))
                //     }
                // }
                //
                // fn try_reserve_exact(
                //     &mut self,
                //     additional: usize,
                // ) -> StackStringResult<Result<(), TryReserveError>> {
                //     self.try_reserve(additional)
                // }

                #[inline]
                fn shrink_to(&mut self, _min_capacity: usize) {
                    // just do nothing :)
                }

                #[inline]
                fn push(&mut self, ch: char) -> StackStringResult<()> {
                    let len_utf8 = ch.len_utf8();
                    let len_needed = self.len + len_utf8;

                    if len_needed > CAPACITY {
                        Err(Some(len_needed))
                    } else {
                        match len_utf8 {
                            1 => {
                                unsafe { *self.data[self.len].as_mut_ptr() = ch as u8 }
                                self.len += 1;
                            }
                            test_len_utf8 => {
                                let mut buf = [0; 4];
                                let bytes = ch.encode_utf8(&mut buf).as_bytes();
                                debug_assert_eq!(test_len_utf8, bytes.len());
                                unsafe { self.copy_append_unchecked(bytes.as_ptr(), bytes.len()) }
                            }
                        }
                        Ok(())
                    }
                }

                #[inline]
                fn as_bytes(&self) -> &[u8] {
                    unsafe { slice::from_raw_parts(self.as_ptr(), self.len) }
                }

                #[inline]
                fn as_bytes_mut(&mut self) -> &mut [u8] {
                    unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), self.len) }
                }

                #[inline]
                fn truncate(&mut self, new_len: usize) {
                    if new_len <= self.len {
                        assert!(self.is_char_boundary(new_len));
                        unsafe {
                            self.set_len(new_len);
                        }
                    }
                }

                #[inline]
                fn pop(&mut self) -> Option<char> {
                    let ch = self.chars().rev().next()?;
                    let newlen = self.len() - ch.len_utf8();
                    unsafe {
                        self.set_len(newlen);
                    }
                    Some(ch)
                }

                #[inline]
                fn remove(&mut self, idx: usize) -> char {
                    let ch = match self[idx..].chars().next() {
                        Some(ch) => ch,
                        None => panic!("cannot remove a char from the end of a string"),
                    };

                    let next = idx + ch.len_utf8();
                    let len = self.len();
                    unsafe {
                        ptr::copy(
                            self.as_ptr().add(next),
                            self.as_mut_ptr().add(idx),
                            len - next,
                        );
                        self.set_len(len - (next - idx));
                    }
                    ch
                }

                #[inline]
                fn insert(&mut self, idx: usize, ch: char) -> StackStringResult<()> {
                    assert!(self.is_char_boundary(idx));
                    let mut bits = [0; 4];
                    let bits = ch.encode_utf8(&mut bits).as_bytes();

                    unsafe { self.insert_bytes(idx, bits) }
                }

                #[inline]
                fn insert_str(&mut self, idx: usize, string: &str) -> StackStringResult<()> {
                    assert!(self.is_char_boundary(idx));

                    unsafe { self.insert_bytes(idx, string.as_bytes()) }
                }

                #[inline]
                fn len(&self) -> usize {
                    self.len
                }

                #[inline]
                fn is_empty(&self) -> bool {
                    self.len == 0
                }

                #[inline]
                fn clear(&mut self) {
                    self.truncate(0)
                }

                #[inline]
                unsafe fn set_len(&mut self, new_len: usize) {
                    debug_assert!(new_len <= CAPACITY);
                    self.len = new_len;
                }

                //////////

                #[inline]
                fn as_ptr(&self) -> *const u8 {
                    self.data.as_ptr() as *const u8
                }

                #[inline]
                fn as_mut_ptr(&mut self) -> *mut u8 {
                    self.data.as_mut_ptr() as *mut u8
                }

                #[inline]
                unsafe fn copy_append_unchecked(&mut self, src: *const u8, len: usize) {
                    let dst = self.as_mut_ptr().add(self.len);
                    ptr::copy_nonoverlapping(src, dst, len);
                    self.len += len;
                }

                #[inline]
                fn to_heap(&self, estimated_capacity: Option<usize>) -> String {
                    let bytes = self.as_bytes();

                    if let Some(capacity) = estimated_capacity {
                        let mut vec = Vec::with_capacity(capacity);
                        vec.extend_from_slice(bytes);
                        unsafe { String::from_utf8_unchecked(vec) }
                    } else {
                        unsafe { String::from_utf8_unchecked(bytes.into()) }
                    }
                }

                #[inline]
                unsafe fn insert_bytes(&mut self, idx: usize, bytes: &[u8]) -> StackStringResult<()> {
                    let len = self.len();
                    let amt = bytes.len();

                    let len_needed = len + amt;
                    if len_needed > CAPACITY {
                        return Err(Some(len_needed));
                    }

                    ptr::copy(
                        self.as_ptr().add(idx),
                        self.as_mut_ptr().add(idx + amt),
                        len - idx,
                    );
                    ptr::copy_nonoverlapping(bytes.as_ptr(), self.as_mut_ptr().add(idx), amt);
                    self.set_len(len_needed);

                    Ok(())
                }
            }

            impl<const CAPACITY: usize> ops::Deref for StackString<CAPACITY> {
                type Target = str;

                #[inline]
                fn deref(&self) -> &str {
                    let bytes = self.as_bytes();
                    unsafe { str::from_utf8_unchecked(bytes) }
                }
            }

            impl<const CAPACITY: usize> ops::DerefMut for StackString<CAPACITY> {
                #[inline]
                fn deref_mut(&mut self) -> &mut str {
                    let bytes = self.as_bytes_mut();
                    unsafe { str::from_utf8_unchecked_mut(bytes) }
                }
            }

            impl<const CAPACITY: usize> fmt::Debug for StackString<CAPACITY> {
                #[inline]
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    fmt::Debug::fmt(&**self, f)
                }
            }

            //////////////////////////////////////////////////

            impl FromUtf8Error {
                /// Returns a slice of [`u8`]s bytes that were attempted to convert to a `FlexibleString`.
                ///
                /// # Examples
                ///
                /// Basic usage:
                ///
                /// ```
                /// use spdlog::string_buf::FlexibleString;
                ///
                /// // some invalid bytes, in a vector
                /// let bytes = vec![0, 159];
                ///
                /// let value = FlexibleString::<200>::from_utf8(bytes);
                ///
                /// assert_eq!(&[0, 159], value.unwrap_err().as_bytes());
                /// ```
                #[inline]
                pub fn as_bytes(&self) -> &[u8] {
                    &self.bytes[..]
                }

                /// Returns the bytes that were attempted to convert to a `FlexibleString`.
                ///
                /// This method is carefully constructed to avoid allocation. It will
                /// consume the error, moving out the bytes, so that a copy of the bytes
                /// does not need to be made.
                ///
                /// # Examples
                ///
                /// Basic usage:
                ///
                /// ```
                /// use spdlog::string_buf::FlexibleString;
                ///
                /// // some invalid bytes, in a vector
                /// let bytes = vec![0, 159];
                ///
                /// let value = FlexibleString::<200>::from_utf8(bytes);
                ///
                /// assert_eq!(vec![0, 159], value.unwrap_err().into_bytes());
                /// ```
                #[inline]
                pub fn into_bytes(self) -> Vec<u8> {
                    self.bytes
                }

                /// Fetch a `Utf8Error` to get more details about the conversion failure.
                ///
                /// The [`std::str::Utf8Error`] type represents an error that may
                /// occur when converting a slice of [`u8`]s to a [`&str`]. In this sense, it's
                /// an analogue to `FromUtf8Error`. See its documentation for more details
                /// on using it.
                ///
                /// [`std::str`]: core::str "std::str"
                /// [`&str`]: prim@str "&str"
                ///
                /// # Examples
                ///
                /// Basic usage:
                ///
                /// ```
                /// use spdlog::string_buf::FlexibleString;
                ///
                /// // some invalid bytes, in a vector
                /// let bytes = vec![0, 159];
                ///
                /// let error = FlexibleString::<200>::from_utf8(bytes).unwrap_err().utf8_error();
                ///
                /// // the first byte is invalid here
                /// assert_eq!(1, error.valid_up_to());
                /// ```
                #[inline]
                pub fn utf8_error(&self) -> str::Utf8Error {
                    self.error
                }
            }

            impl From<string::FromUtf8Error> for FromUtf8Error {
                fn from(std_err: string::FromUtf8Error) -> Self {
                    let err = std_err.utf8_error();
                    Self {
                        bytes: std_err.into_bytes(),
                        error: err,
                    }
                }
            }

            impl fmt::Display for FromUtf8Error {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    fmt::Display::fmt(&self.error, f)
                }
            }
        }
    }
}
