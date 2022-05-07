use crate::{
    formatter::{Formatter, FullFormatter},
    prelude::*,
    sync::*,
    Error,
};

pub(crate) struct CommonImpl {
    pub(crate) level_filter: Atomic<LevelFilter>,
    pub(crate) formatter: SpinRwLock<Box<dyn Formatter>>,
}

impl CommonImpl {
    pub(crate) fn new() -> Self {
        Self {
            level_filter: Atomic::new(LevelFilter::All),
            formatter: SpinRwLock::new(Box::new(FullFormatter::new())),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn with_formatter(formatter: Box<dyn Formatter>) -> Self {
        Self {
            level_filter: Atomic::new(LevelFilter::All),
            formatter: SpinRwLock::new(formatter),
        }
    }

    pub(crate) fn non_throwable_error(&self, from: impl AsRef<str>, err: Error) {
        // Sinks do not have an error handler, because it would increase complexity and
        // the error is not common. So currently users cannot handle this error by
        // themselves.
        crate::default_error_handler(from, err);
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
    };
}
pub(crate) use common_impl;
