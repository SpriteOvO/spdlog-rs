use std::result::Result as StdResult;

use cfg_if::cfg_if;
use serde::Deserialize;

use crate::{
    config::Configurable,
    formatter::{Formatter, FullFormatter},
    prelude::*,
    sync::*,
    Error, ErrorHandler,
};

pub(crate) type SinkErrorHandler = Atomic<Option<ErrorHandler>>;

cfg_if! {
    if #[cfg(test)] {
        crate::utils::const_assert!(Atomic::<SinkErrorHandler>::is_lock_free());
    }
}

pub(crate) const fn sink_default_level_filter() -> LevelFilter {
    LevelFilter::All
}

pub(crate) struct CommonImpl {
    pub(crate) level_filter: Atomic<LevelFilter>,
    pub(crate) formatter: SpinRwLock<Box<dyn Formatter>>,
    pub(crate) error_handler: SinkErrorHandler,
}

impl CommonImpl {
    #[must_use]
    pub(crate) fn from_builder(common_builder_impl: CommonBuilderImpl) -> Self {
        Self::from_builder_with_formatter(common_builder_impl, || Box::new(FullFormatter::new()))
    }

    #[must_use]
    pub(crate) fn from_builder_with_formatter(
        common_builder_impl: CommonBuilderImpl,
        fallback: impl FnOnce() -> Box<dyn Formatter>,
    ) -> Self {
        Self {
            level_filter: Atomic::new(common_builder_impl.level_filter),
            formatter: SpinRwLock::new(common_builder_impl.formatter.unwrap_or_else(fallback)),
            error_handler: Atomic::new(common_builder_impl.error_handler),
        }
    }

    #[allow(dead_code)]
    #[must_use]
    pub(crate) fn with_formatter(formatter: Box<dyn Formatter>) -> Self {
        Self {
            level_filter: Atomic::new(LevelFilter::All),
            formatter: SpinRwLock::new(formatter),
            error_handler: Atomic::new(None),
        }
    }

    pub(crate) fn non_returnable_error(&self, from: impl AsRef<str>, err: Error) {
        match self.error_handler.load(Ordering::Relaxed) {
            Some(handler) => handler(err),
            None => crate::default_error_handler(from, err),
        }
    }
}

#[derive(Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub(crate) struct CommonBuilderImpl {
    #[serde(default = "sink_default_level_filter")]
    pub(crate) level_filter: LevelFilter,
    #[serde(default, deserialize_with = "crate::config::deser::formatter")]
    pub(crate) formatter: Option<Box<dyn Formatter>>,
    #[serde(skip)] // Set `error_handler` from config is not supported
    pub(crate) error_handler: Option<ErrorHandler>,
}

impl Default for CommonBuilderImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl CommonBuilderImpl {
    #[must_use]
    pub(crate) fn new() -> Self {
        Self {
            level_filter: sink_default_level_filter(),
            formatter: None,
            error_handler: None,
        }
    }
}

macro_rules! common_impl {
    // Sink

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

    // SinkBuiler

    ( @SinkBuilder: $($field:ident).+ ) => {
        $crate::sink::helper::common_impl!(@SinkBuilderCustomInner@level_filter: $($field).+.level_filter);
        $crate::sink::helper::common_impl!(@SinkBuilderCustomInner@formatter: $($field).+.formatter);
        $crate::sink::helper::common_impl!(@SinkBuilderCustomInner@error_handler: $($field).+.error_handler);
    };
    ( @SinkBuilderCustom {
        level_filter: $($level_filter:ident).+,
        formatter: $($formatter:ident).+,
        error_handler: $($error_handler:ident).+$(,)?
    } ) => {
        $crate::sink::helper::common_impl!(@SinkBuilderCustomInner@level_filter: $($level_filter).+);
        $crate::sink::helper::common_impl!(@SinkBuilderCustomInner@formatter: $($formatter).+);
        $crate::sink::helper::common_impl!(@SinkBuilderCustomInner@error_handler: $($error_handler).+);
    };
    ( @SinkBuilderCustomInner@level_filter: None ) => {};
    ( @SinkBuilderCustomInner@level_filter: $($field:ident).+ ) => {
        $crate::sink::helper::common_impl! {
            /// Specifies a log level filter.
            ///
            /// This parameter is **optional**, and defaults to [`LevelFilter::All`].
            ///
            /// [`LevelFilter::All`]: crate::LevelFilter::All
            @SinkBuilderCustomInner@level_filter: $($field).+
        }
    };
    ( $(#[$attr:meta])* @SinkBuilderCustomInner@level_filter: $($field:ident).+ ) => {
        $(#[$attr])*
        #[must_use]
        pub fn level_filter(mut self, level_filter: $crate::LevelFilter) -> Self {
            self.$($field).+ = level_filter;
            self
        }
    };
    ( @SinkBuilderCustomInner@formatter: None ) => {};
    ( @SinkBuilderCustomInner@formatter: $($field:ident).+ ) => {
        $crate::sink::helper::common_impl! {
            /// Specifies a formatter.
            ///
            /// This parameter is **optional**, and defaults to [`FullFormatter`].
            ///
            /// [`FullFormatter`]: crate::formatter::FullFormatter
            @SinkBuilderCustomInner@formatter: $($field).+
        }
    };
    ( $(#[$attr:meta])* @SinkBuilderCustomInner@formatter: $($field:ident).+ ) => {
        $(#[$attr])*
        #[must_use]
        pub fn formatter(mut self, formatter: Box<dyn $crate::formatter::Formatter>) -> Self {
            self.$($field).+ = Some(formatter);
            self
        }
    };
    ( @SinkBuilderCustomInner@error_handler: None ) => {};
    ( @SinkBuilderCustomInner@error_handler: $($field:ident).+ ) => {
        $crate::sink::helper::common_impl! {
            /// Specifies an error handler.
            ///
            /// This parameter is **optional**, and defaults no handler, see [`Sink::set_error_handler`] for details.
            ///
            /// [`Sink::set_error_handler`]: crate::Sink::set_error_handler
            @SinkBuilderCustomInner@error_handler: $($field).+
        }
    };
    ( $(#[$attr:meta])* @SinkBuilderCustomInner@error_handler: $($field:ident).+ ) => {
        $(#[$attr])*
        #[must_use]
        pub fn error_handler(mut self, handler: $crate::ErrorHandler) -> Self {
            self.$($field).+ = Some(handler);
            self
        }
    };
}
pub(crate) use common_impl;
