use std::{borrow::Cow, fmt};

// Avoid performance cost due to the branch of matching variants.
//
// The idea is borrowed from https://github.com/emit-rs/emit/blob/097f52542220aeb04127705141c6ff84ab96d58d/core/src/str.rs
#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) struct RefStr<'a> {
    ptr: *const str,
    inner: RefStrInner<'a>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum RefStrInner<'a> {
    Borrowed(&'a str),
    Static(&'static str),
}

impl RefStr<'static> {
    pub(crate) fn new_static(s: &'static str) -> Self {
        Self {
            ptr: s,
            inner: RefStrInner::Static(s),
        }
    }
}

impl<'a> RefStr<'a> {
    pub(crate) fn new_ref(s: &'a str) -> Self {
        Self {
            ptr: s,
            inner: RefStrInner::Borrowed(s),
        }
    }

    pub(crate) fn get(&self) -> &str {
        // SAFETY: `self.ptr` is guaranteed to outlive the borrow of `self`.
        unsafe { &*self.ptr }
    }

    pub(crate) fn to_cow_static(self) -> Cow<'static, str> {
        match self.inner {
            RefStrInner::Static(s) => Cow::Borrowed(s),
            RefStrInner::Borrowed(s) => Cow::Owned(s.to_owned()),
        }
    }
}

impl fmt::Debug for RefStr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(f)
    }
}

impl PartialEq<str> for RefStr<'_> {
    fn eq(&self, other: &str) -> bool {
        self.get() == other
    }
}

impl PartialEq<RefStr<'_>> for str {
    fn eq(&self, other: &RefStr<'_>) -> bool {
        self == other.get()
    }
}

impl PartialEq<&str> for RefStr<'_> {
    fn eq(&self, other: &&str) -> bool {
        self.get() == *other
    }
}

impl PartialEq<RefStr<'_>> for &str {
    fn eq(&self, other: &RefStr<'_>) -> bool {
        *self == other.get()
    }
}
