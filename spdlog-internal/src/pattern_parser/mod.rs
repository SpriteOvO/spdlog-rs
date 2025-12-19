use std::borrow::Cow;

use strum::IntoEnumIterator as _;
use strum_macros::{EnumDiscriminants, EnumIter, EnumString, IntoStaticStr};

pub mod error;
mod helper;
pub mod parse;
mod registry;

pub use error::{Error, Result};
pub use registry::{check_custom_pattern_names, PatternRegistry};

#[derive(
    Clone, Copy, Debug, Eq, PartialEq, IntoStaticStr, EnumDiscriminants, EnumIter, EnumString,
)]
#[strum_discriminants(derive(IntoStaticStr))]
pub enum BuiltInFormatterInner {
    #[strum(serialize = "weekday_name")]
    AbbrWeekdayName,
    #[strum(serialize = "weekday_name_full")]
    WeekdayName,
    #[strum(serialize = "month_name")]
    AbbrMonthName,
    #[strum(serialize = "month_name_full")]
    MonthName,
    #[strum(serialize = "datetime")]
    FullDateTime,
    #[strum(serialize = "year_short")]
    ShortYear,
    #[strum(serialize = "year")]
    Year,
    #[strum(serialize = "date_short")]
    ShortDate,
    #[strum(serialize = "date")]
    Date,
    #[strum(serialize = "month")]
    Month,
    #[strum(serialize = "day")]
    Day,
    #[strum(serialize = "hour")]
    Hour,
    #[strum(serialize = "hour_12")]
    Hour12,
    #[strum(serialize = "minute")]
    Minute,
    #[strum(serialize = "second")]
    Second,
    #[strum(serialize = "millisecond")]
    Millisecond,
    #[strum(serialize = "microsecond")]
    Microsecond,
    #[strum(serialize = "nanosecond")]
    Nanosecond,
    #[strum(serialize = "am_pm")]
    AmPm,
    #[strum(serialize = "time_12")]
    Time12,
    #[strum(serialize = "time_short")]
    ShortTime,
    #[strum(serialize = "time")]
    Time,
    #[strum(serialize = "tz_offset")]
    TzOffset,
    #[strum(serialize = "unix_timestamp")]
    UnixTimestamp,
    #[strum(serialize = "full")]
    Full,
    #[strum(serialize = "level")]
    Level,
    #[strum(serialize = "level_short")]
    ShortLevel,
    #[strum(serialize = "source")]
    Source,
    #[strum(serialize = "file_name")]
    SourceFilename,
    #[strum(serialize = "file")]
    SourceFile,
    #[strum(serialize = "line")]
    SourceLine,
    #[strum(serialize = "column")]
    SourceColumn,
    #[strum(serialize = "module_path")]
    SourceModulePath,
    #[strum(serialize = "logger")]
    LoggerName,
    #[strum(serialize = "payload")]
    Payload,
    #[strum(serialize = "kv")]
    KV,
    #[strum(serialize = "pid")]
    ProcessId,
    #[strum(serialize = "tid")]
    ThreadId,
    #[strum(serialize = "eol")]
    Eol,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BuiltInFormatter(BuiltInFormatterInner);

impl BuiltInFormatter {
    pub fn iter() -> impl Iterator<Item = BuiltInFormatter> {
        BuiltInFormatterInner::iter().map(BuiltInFormatter)
    }

    pub fn struct_name(&self) -> &'static str {
        BuiltInFormatterInnerDiscriminants::from(self.0).into()
    }

    pub fn placeholder(&self) -> &'static str {
        self.0.into()
    }

    pub fn inner(&self) -> BuiltInFormatterInner {
        self.0
    }
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
            PatternKind::BuiltIn(b) => PatternKind::BuiltIn(b.clone()),
            PatternKind::Custom { placeholder, .. } => PatternKind::Custom {
                placeholder: placeholder.clone(),
                factory: (),
            },
        }
    }
}
