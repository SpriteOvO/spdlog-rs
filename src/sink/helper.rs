use cfg_if::cfg_if;

use crate::{
    formatter::{Formatter, FullFormatter},
    prelude::*,
    sync::*,
    Error, ErrorHandler,
};

pub(crate) type SinkErrorHandler = Atomic<Option<ErrorHandler>>;

cfg_if! {
    if #[cfg(test)] {
        use static_assertions::const_assert;
        const_assert!(Atomic::<SinkErrorHandler>::is_lock_free());
    }
}

pub(crate) struct CommonImpl {
    pub(crate) level_filter: Atomic<LevelFilter>,
    pub(crate) formatter: SpinRwLock<Box<dyn Formatter>>,
    pub(crate) error_handler: SinkErrorHandler,
}

impl CommonImpl {
    pub(crate) fn new() -> Self {
        Self {
            level_filter: Atomic::new(LevelFilter::All),
            formatter: SpinRwLock::new(Box::new(FullFormatter::new())),
            error_handler: Atomic::new(None),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn with_formatter(formatter: Box<dyn Formatter>) -> Self {
        Self {
            level_filter: Atomic::new(LevelFilter::All),
            formatter: SpinRwLock::new(formatter),
            error_handler: Atomic::new(None),
        }
    }

    pub(crate) fn non_throwable_error(&self, from: impl AsRef<str>, err: Error) {
        match self.error_handler.load(Ordering::Relaxed) {
            Some(handler) => handler(err),
            None => crate::default_error_handler(from, err),
        }
    }
}

macro_rules! common_impl {
    ( @Sink: $($field:ident).+ ) => {
        fn level_filter(&self) -> $crate::LevelFilter {
            self.$($field).+.level_filter.load($crate::sync::Ordering::Relaxed)
        }

        fn set_level_filter(&self, level_filter: $crate::LevelFilter) {
            self.$($field).+
                .level_filter
                .store(level_filter, $crate::sync::Ordering::Relaxed);
        }

        fn set_formatter(&self, formatter: Box<dyn $crate::formatter::Formatter>) {
            *self.$($field).+.formatter.write() = formatter;
        }

        fn set_error_handler(&self, handler: Option<$crate::ErrorHandler>) {
            self.$($field).+.error_handler.store(handler, $crate::sync::Ordering::Relaxed);
        }
    };
}
pub(crate) use common_impl;
