use std::borrow::Cow;

pub mod error;
mod helper;
pub mod parse;
mod registry;

pub use error::{Error, Result};
pub use registry::{check_custom_pattern_names, PatternRegistry};

macro_rules! for_builtin_formatters {
    ( ( $($macro_params:tt)* ) => { $($macro_impl:tt)* }; ) => {
        mod __private {
            macro_rules! __callback {
                ( $($macro_params)* ) => { $($macro_impl)* };
            }
            pub(crate) use __callback;
        }
        __private::__callback! {
            AbbrWeekdayName => "weekday_name",
            WeekdayName => "weekday_name_full",
            AbbrMonthName => "month_name",
            MonthName => "month_name_full",
            FullDateTime => "datetime",
            ShortYear => "year_short",
            Year => "year",
            ShortDate => "date_short",
            Date => "date",
            Month => "month",
            Day => "day",
            Hour => "hour",
            Hour12 => "hour_12",
            Minute => "minute",
            Second => "second",
            Millisecond => "millisecond",
            Microsecond => "microsecond",
            Nanosecond => "nanosecond",
            AmPm => "am_pm",
            Time12 => "time_12",
            ShortTime => "time_short",
            Time => "time",
            TzOffset => "tz_offset",
            UnixTimestamp => "unix_timestamp",
            Full => "full",
            Level => "level",
            ShortLevel => "level_short",
            Source => "source",
            SourceFilename => "file_name",
            SourceFile => "file",
            SourceLine => "line",
            SourceColumn => "column",
            SourceModulePath => "module_path",
            LoggerName => "logger",
            Payload => "payload",
            KV => "kv",
            ProcessId => "pid",
            ThreadId => "tid",
            Eol => "eol",
        }
    };
}

for_builtin_formatters! {
    ( $( $variant:ident => $serialize:literal ),+ $(,)? ) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum BuiltInFormatter {
            $( $variant ),+
        }

        impl BuiltInFormatter {
            pub fn iter() -> impl Iterator<Item = Self> {
                [ $( Self::$variant ),+ ].into_iter()
            }

            #[must_use]
            pub fn struct_name(&self) -> &'static str {
                match self {
                    $( Self::$variant => stringify!($variant), )+
                }
            }

            #[must_use]
            pub fn placeholder(&self) -> &'static str {
                match self {
                    $( Self::$variant => $serialize, )+
                }
            }
        }
    };
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PatternKind<F> {
    BuiltIn(BuiltInFormatter),
    Custom {
        placeholder: Cow<'static, str>,
        factory: F,
    },
}

impl<F> PatternKind<F> {
    pub(crate) fn placeholder(&self) -> &str {
        match self {
            PatternKind::BuiltIn(f) => f.placeholder(),
            PatternKind::Custom { placeholder, .. } => placeholder,
        }
    }

    pub(crate) fn to_factory_erased(&self) -> PatternKind<()> {
        match self {
            PatternKind::BuiltIn(b) => PatternKind::BuiltIn(*b),
            PatternKind::Custom { placeholder, .. } => PatternKind::Custom {
                placeholder: placeholder.clone(),
                factory: (),
            },
        }
    }
}
