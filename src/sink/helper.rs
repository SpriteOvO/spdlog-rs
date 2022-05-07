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

pub(crate) const SINK_DEFAULT_LEVEL_FILTER: LevelFilter = LevelFilter::All;

pub(crate) struct CommonImpl {
    pub(crate) level_filter: Atomic<LevelFilter>,
    pub(crate) formatter: SpinRwLock<Box<dyn Formatter>>,
    pub(crate) error_handler: SinkErrorHandler,
}

impl CommonImpl {
    pub(crate) fn new() -> Self {
        Self {
            level_filter: Atomic::new(SINK_DEFAULT_LEVEL_FILTER),
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
        $crate::sink::helper::common_impl!(@SinkCustom {
            level_filter: $($field).+.level_filter,
            formatter: $($field).+.formatter,
            error_handler: $($field).+.error_handler,
        });
    };
    ( @SinkCustom {
        level_filter: $($level_filter:ident).+,
        formatter: $($formatter:ident).+,
        error_handler: $($error_handler:ident).+$(,)?
    } ) => {
        $crate::sink::helper::common_impl!(@SinkCustomInner@level_filter: $($level_filter).+);
        $crate::sink::helper::common_impl!(@SinkCustomInner@formatter: $($formatter).+);
        $crate::sink::helper::common_impl!(@SinkCustomInner@error_handler: $($error_handler).+);
    };
    ( @SinkCustomInner@level_filter: None ) => {};
    ( @SinkCustomInner@level_filter: $($field:ident).+ ) => {
        fn level_filter(&self) -> $crate::LevelFilter {
            self.$($field).+.load($crate::sync::Ordering::Relaxed)
        }

        fn set_level_filter(&self, level_filter: $crate::LevelFilter) {
            self.$($field).+.store(level_filter, $crate::sync::Ordering::Relaxed);
        }
    };
    ( @SinkCustomInner@formatter: None ) => {};
    ( @SinkCustomInner@formatter: $($field:ident).+ ) => {
        fn set_formatter(&self, formatter: Box<dyn $crate::formatter::Formatter>) {
            *self.$($field).+.write() = formatter;
        }
    };
    ( @SinkCustomInner@error_handler: None ) => {};
    ( @SinkCustomInner@error_handler: $($field:ident).+ ) => {
        fn set_error_handler(&self, handler: Option<$crate::ErrorHandler>) {
            self.$($field).+.store(handler, $crate::sync::Ordering::Relaxed);
        }
    };
}
pub(crate) use common_impl;
