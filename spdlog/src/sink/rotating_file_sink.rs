//! Provides a rotating file sink.

use std::{
    collections::LinkedList,
    convert::Infallible,
    ffi::OsString,
    fs::{self, File},
    hash::Hash,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    result::Result as StdResult,
    time::{Duration, SystemTime},
};

use chrono::prelude::*;

use crate::{
    error::InvalidArgumentError,
    formatter::FormatterContext,
    sink::{helper, Sink},
    sync::*,
    utils, Error, Record, Result, StringBuf,
};

/// Rotation policies for [`RotatingFileSink`].
///
/// Rotation policy defines when and how to split logs into multiple files,
/// during which new log files may be created and old log files may be deleted.
///
/// # Error
///
/// Note that some parameters have range requirements, functions that receive it
/// will return an error if the requirements are not met.
///
/// # Examples
///
/// ```
/// use spdlog::sink::RotationPolicy;
///
/// // Rotating every 10 MB file.
/// RotationPolicy::FileSize(1024 * 1024 * 10);
///
/// // Rotating every day at 15:30.
/// RotationPolicy::Daily { hour: 15, minute: 30 };
///
/// // Rotating every hour.
/// RotationPolicy::Hourly;
///
/// // Rotating every 6 hour.
/// # use std::time::Duration;
/// RotationPolicy::Period(Duration::from_secs(6 * 60 * 60));
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum RotationPolicy {
    /// Rotating to a new log file when the size of the current log file exceeds
    /// the given limit.
    FileSize(
        /// Maximum file size (in bytes). Range: (0, u64::MAX].
        u64,
    ),
    /// Rotating to a new log file at a specified time point within a day.
    Daily {
        /// Hour of the time point. Range: [0, 23].
        hour: u32,
        /// Minute of the time point. Range: [0, 59].
        minute: u32,
    },
    /// Rotating to a new log file at minute 0 of each hour.
    Hourly,
    /// Rotating to a new log file after given period (greater then 1 minute) is
    /// passed.
    Period(
        /// Period to the next rotation. Range: [1 minute, Duration::MAX].
        Duration,
    ),
}

const SECONDS_PER_MINUTE: u64 = 60;
const SECONDS_PER_HOUR: u64 = 60 * SECONDS_PER_MINUTE;
const SECONDS_PER_DAY: u64 = 24 * SECONDS_PER_HOUR;
const MINUTE_1: Duration = Duration::from_secs(SECONDS_PER_MINUTE);
const HOUR_1: Duration = Duration::from_secs(SECONDS_PER_HOUR);
const DAY_1: Duration = Duration::from_secs(SECONDS_PER_DAY);

trait Rotator {
    #[allow(clippy::ptr_arg)]
    fn log(&self, record: &Record, string_buf: &StringBuf) -> Result<()>;
    fn flush(&self) -> Result<()>;
    fn drop_flush(&mut self) -> Result<()> {
        self.flush()
    }
}

enum RotatorKind {
    FileSize(RotatorFileSize),
    TimePoint(RotatorTimePoint),
}

struct RotatorFileSize {
    base_path: PathBuf,
    max_size: u64,
    max_files: usize,
    inner: SpinMutex<RotatorFileSizeInner>,
}

struct RotatorFileSizeInner {
    file: Option<BufWriter<File>>,
    current_size: u64,
}

struct RotatorTimePoint {
    base_path: PathBuf,
    time_point: TimePoint,
    max_files: usize,
    inner: SpinMutex<RotatorTimePointInner>,
}

#[derive(Copy, Clone)]
enum TimePoint {
    Daily { hour: u32, minute: u32 },
    Hourly,
    Period(Duration),
}

struct RotatorTimePointInner {
    file: BufWriter<File>,
    rotation_time_point: SystemTime,
    file_paths: Option<LinkedList<PathBuf>>,
}

/// A sink with a file as the target, split files according to the rotation
/// policy.
///
/// A service program running for a long time may continuously write logs to a
/// single file, which makes the logs hard to view and manage.
/// `RotatingFileSink` is designed for this usage scenario. It automatically
/// splits logs into one or more files and can be configured to automatically
/// delete old files to save disk space. The operation of splitting logs into
/// multiple files and optionally deleting old files is called **rotation**. The
/// **rotation policy** determines when and how log files are created or
/// deleted.
///
/// # Examples
///
/// See [./examples] directory.
///
/// [./examples]: https://github.com/SpriteOvO/spdlog-rs/tree/main/spdlog/examples
pub struct RotatingFileSink {
    common_impl: helper::CommonImpl,
    rotator: RotatorKind,
}

/// #
#[doc = include_str!("../include/doc/generic-builder-note.md")]
pub struct RotatingFileSinkBuilder<ArgBP, ArgRP> {
    common_builder_impl: helper::CommonBuilderImpl,
    base_path: ArgBP,
    rotation_policy: ArgRP,
    max_files: usize,
    rotate_on_open: bool,
}

impl RotatingFileSink {
    /// Gets a builder of `RotatingFileSink` with default parameters:
    ///
    /// | Parameter         | Default Value           |
    /// |-------------------|-------------------------|
    /// | [level_filter]    | `All`                   |
    /// | [formatter]       | `FullFormatter`         |
    /// | [error_handler]   | [default error handler] |
    /// |                   |                         |
    /// | [base_path]       | *must be specified*     |
    /// | [rotation_policy] | *must be specified*     |
    /// | [max_files]       | `0`                     |
    /// | [rotate_on_open]  | `false`                 |
    ///
    /// [level_filter]: RotatingFileSinkBuilder::level_filter
    /// [formatter]: RotatingFileSinkBuilder::formatter
    /// [error_handler]: RotatingFileSinkBuilder::error_handler
    /// [default error handler]: error/index.html#default-error-handler
    /// [base_path]: RotatingFileSinkBuilder::base_path
    /// [rotation_policy]: RotatingFileSinkBuilder::rotation_policy
    /// [max_files]: RotatingFileSinkBuilder::max_files
    /// [rotate_on_open]: RotatingFileSinkBuilder::rotate_on_open
    #[must_use]
    pub fn builder() -> RotatingFileSinkBuilder<(), ()> {
        RotatingFileSinkBuilder {
            common_builder_impl: helper::CommonBuilderImpl::new(),
            base_path: (),
            rotation_policy: (),
            max_files: 0,
            rotate_on_open: false,
        }
    }

    /// Constructs a `RotatingFileSink`.
    ///
    /// The parameter `max_files` specifies the maximum number of files. If the
    /// number of existing files reaches this parameter, the oldest file will be
    /// deleted on the next rotation. Pass `0` for no limit.
    ///
    /// The parameter `rotate_on_open` specifies whether to rotate files once
    /// when constructing `RotatingFileSink`. For the [`RotationPolicy::Daily`],
    /// [`RotationPolicy::Hourly`], and [`RotationPolicy::Period`] rotation
    /// policies, it may truncate the contents of the existing file if the
    /// parameter is `true`, since the file name is a time point and not an
    /// index.
    ///
    /// # Error
    ///
    /// If an error occurs opening the file, [`Error::CreateDirectory`] or
    /// [`Error::OpenFile`] will be returned.
    ///
    /// # Panics
    ///
    /// Panics if the parameter `rotation_policy` is invalid. See the
    /// documentation of [`RotationPolicy`] for requirements.
    #[deprecated(
        since = "0.3.0",
        note = "it may be removed in the future, use `RotatingFileSink::builder()` instead"
    )]
    pub fn new<P>(
        base_path: P,
        rotation_policy: RotationPolicy,
        max_files: usize,
        rotate_on_open: bool,
    ) -> Result<Self>
    where
        P: Into<PathBuf>,
    {
        Self::builder()
            .base_path(base_path)
            .rotation_policy(rotation_policy)
            .max_files(max_files)
            .rotate_on_open(rotate_on_open)
            .build()
    }

    #[cfg(test)]
    #[must_use]
    fn _current_size(&self) -> u64 {
        if let RotatorKind::FileSize(rotator) = &self.rotator {
            rotator.inner.lock().current_size
        } else {
            panic!();
        }
    }
}

impl Sink for RotatingFileSink {
    fn log(&self, record: &Record) -> Result<()> {
        let mut string_buf = StringBuf::new();
        let mut ctx = FormatterContext::new();
        self.common_impl
            .formatter
            .read()
            .format(record, &mut string_buf, &mut ctx)?;

        self.rotator.log(record, &string_buf)
    }

    fn flush(&self) -> Result<()> {
        self.rotator.flush()
    }

    helper::common_impl!(@Sink: common_impl);
}

impl Drop for RotatingFileSink {
    fn drop(&mut self) {
        if let Err(err) = self.rotator.drop_flush() {
            self.common_impl
                .non_returnable_error("RotatingFileSink", err)
        }
    }
}

impl RotationPolicy {
    fn validate(&self) -> StdResult<(), String> {
        match self {
            Self::FileSize(max_size) => {
                if *max_size == 0 {
                    return Err(format!(
                        "policy 'file size' expect `max_size` to be (0, u64::MAX] but got {}",
                        *max_size
                    ));
                }
            }
            Self::Daily { hour, minute } => {
                if *hour > 23 || *minute > 59 {
                    return Err(format!(
                        "policy 'daily' expect `(hour, minute)` to be ([0, 23], [0, 59]) but got ({}, {})",
                        *hour, *minute
                    ));
                }
            }
            Self::Hourly => {}
            Self::Period(duration) => {
                if *duration < MINUTE_1 {
                    return Err(format!(
                        "policy 'period' expect duration greater then 1 minute but got {:?}",
                        *duration
                    ));
                }
            }
        }
        Ok(())
    }
}

impl Rotator for RotatorKind {
    fn log(&self, record: &Record, string_buf: &StringBuf) -> Result<()> {
        match self {
            Self::FileSize(rotator) => rotator.log(record, string_buf),
            Self::TimePoint(rotator) => rotator.log(record, string_buf),
        }
    }

    fn flush(&self) -> Result<()> {
        match self {
            Self::FileSize(rotator) => rotator.flush(),
            Self::TimePoint(rotator) => rotator.flush(),
        }
    }

    fn drop_flush(&mut self) -> Result<()> {
        match self {
            Self::FileSize(rotator) => rotator.drop_flush(),
            Self::TimePoint(rotator) => rotator.drop_flush(),
        }
    }
}

impl RotatorFileSize {
    fn new(
        base_path: PathBuf,
        max_size: u64,
        max_files: usize,
        rotate_on_open: bool,
    ) -> Result<Self> {
        let file = utils::open_file(&base_path, false)?;
        let current_size = file.metadata().map_err(Error::QueryFileMetadata)?.len();

        let res = Self {
            base_path,
            max_size,
            max_files,
            inner: SpinMutex::new(RotatorFileSizeInner::new(file, current_size)),
        };

        if rotate_on_open && current_size > 0 {
            res.rotate(&mut res.inner.lock())?;
            res.inner.lock().current_size = 0;
        }

        Ok(res)
    }

    fn reopen(&self) -> Result<File> {
        // always truncate
        utils::open_file(&self.base_path, true)
    }

    fn rotate(&self, opened_file: &mut SpinMutexGuard<RotatorFileSizeInner>) -> Result<()> {
        let inner = || {
            for i in (1..self.max_files).rev() {
                let src = Self::calc_file_path(&self.base_path, i - 1);
                if !src.exists() {
                    continue;
                }

                let dst = Self::calc_file_path(&self.base_path, i);
                if dst.exists() {
                    fs::remove_file(&dst).map_err(Error::RemoveFile)?;
                }

                fs::rename(src, dst).map_err(Error::RenameFile)?;
            }
            Ok(())
        };

        opened_file.file = None;

        let res = inner();
        if res.is_err() {
            opened_file.current_size = 0;
        }

        opened_file.file = Some(BufWriter::new(self.reopen()?));

        res
    }

    #[must_use]
    fn calc_file_path(base_path: impl AsRef<Path>, index: usize) -> PathBuf {
        let base_path = base_path.as_ref();

        if index == 0 {
            return base_path.to_owned();
        }

        let mut file_name = base_path
            .file_stem()
            .map(|s| s.to_owned())
            .unwrap_or_else(|| OsString::from(""));

        let externsion = base_path.extension();

        // append index
        file_name.push(format!("_{}", index));

        let mut path = base_path.to_owned();
        path.set_file_name(file_name);
        if let Some(externsion) = externsion {
            path.set_extension(externsion);
        }

        path
    }

    // if `self.inner.file` is `None`, try to reopen the file.
    fn lock_inner(&self) -> Result<SpinMutexGuard<RotatorFileSizeInner>> {
        let mut inner = self.inner.lock();
        if inner.file.is_none() {
            inner.file = Some(BufWriter::new(self.reopen()?));
        }
        Ok(inner)
    }
}

impl Rotator for RotatorFileSize {
    fn log(&self, _record: &Record, string_buf: &StringBuf) -> Result<()> {
        let mut inner = self.lock_inner()?;

        inner.current_size += string_buf.len() as u64;
        if inner.current_size > self.max_size {
            self.rotate(&mut inner)?;
            inner.current_size = string_buf.len() as u64;
        }

        inner
            .file
            .as_mut()
            .unwrap()
            .write_all(string_buf.as_bytes())
            .map_err(Error::WriteRecord)
    }

    fn flush(&self) -> Result<()> {
        self.lock_inner()?
            .file
            .as_mut()
            .unwrap()
            .flush()
            .map_err(Error::FlushBuffer)
    }

    fn drop_flush(&mut self) -> Result<()> {
        let mut inner = self.inner.lock();
        if let Some(file) = inner.file.as_mut() {
            file.flush().map_err(Error::FlushBuffer)
        } else {
            Ok(())
        }
    }
}

impl RotatorFileSizeInner {
    #[must_use]
    fn new(file: File, current_size: u64) -> Self {
        Self {
            file: Some(BufWriter::new(file)),
            current_size,
        }
    }
}

impl RotatorTimePoint {
    fn new(
        override_now: Option<SystemTime>,
        base_path: PathBuf,
        time_point: TimePoint,
        max_files: usize,
        truncate: bool,
    ) -> Result<Self> {
        let now = override_now.unwrap_or_else(SystemTime::now);
        let file_path = Self::calc_file_path(base_path.as_path(), time_point, now);
        let file = utils::open_file(file_path, truncate)?;

        let inner = RotatorTimePointInner {
            file: BufWriter::new(file),
            rotation_time_point: Self::next_rotation_time_point(time_point, now),
            file_paths: None,
        };

        let mut res = Self {
            base_path,
            time_point,
            max_files,
            inner: SpinMutex::new(inner),
        };

        res.init_previous_file_paths(max_files, now);

        Ok(res)
    }

    fn init_previous_file_paths(&mut self, max_files: usize, mut now: SystemTime) {
        if max_files > 0 {
            let mut file_paths = LinkedList::new();

            for _ in 0..max_files {
                let file_path = Self::calc_file_path(&self.base_path, self.time_point, now);

                if !file_path.exists() {
                    break;
                }

                file_paths.push_front(file_path);
                now = now.checked_sub(self.time_point.delta_std()).unwrap()
            }

            self.inner.get_mut().file_paths = Some(file_paths);
        }
    }

    // a little expensive, should only be called when rotation is needed or in
    // constructor.
    #[must_use]
    fn next_rotation_time_point(time_point: TimePoint, now: SystemTime) -> SystemTime {
        let now: DateTime<Local> = now.into();
        let mut rotation_time = now;

        match time_point {
            TimePoint::Daily { hour, minute } => {
                rotation_time = rotation_time
                    .with_hour(hour)
                    .unwrap()
                    .with_minute(minute)
                    .unwrap()
                    .with_second(0)
                    .unwrap()
                    .with_nanosecond(0)
                    .unwrap()
            }
            TimePoint::Hourly => {
                rotation_time = rotation_time
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap()
                    .with_nanosecond(0)
                    .unwrap()
            }
            TimePoint::Period { .. } => {}
        };

        if rotation_time <= now {
            rotation_time = rotation_time
                .checked_add_signed(time_point.delta_chrono())
                .unwrap();
        }
        rotation_time.into()
    }

    fn push_new_remove_old(
        &self,
        new: PathBuf,
        inner: &mut SpinMutexGuard<RotatorTimePointInner>,
    ) -> Result<()> {
        let file_paths = inner.file_paths.as_mut().unwrap();

        while file_paths.len() >= self.max_files {
            let old = file_paths.pop_front().unwrap();
            if old.exists() {
                fs::remove_file(old).map_err(Error::RemoveFile)?;
            }
        }
        file_paths.push_back(new);

        Ok(())
    }

    #[must_use]
    fn calc_file_path(
        base_path: impl AsRef<Path>,
        time_point: TimePoint,
        system_time: SystemTime,
    ) -> PathBuf {
        let base_path = base_path.as_ref();
        let local_time: DateTime<Local> = system_time.into();

        let mut file_name = base_path
            .file_stem()
            .map(|s| s.to_owned())
            .unwrap_or_else(|| OsString::from(""));

        let externsion = base_path.extension();

        match time_point {
            TimePoint::Daily { .. } => {
                // append y-m-d
                file_name.push(format!(
                    "_{}-{:02}-{:02}",
                    local_time.year(),
                    local_time.month(),
                    local_time.day()
                ));
            }
            TimePoint::Hourly => {
                // append y-m-d_h
                file_name.push(format!(
                    "_{}-{:02}-{:02}_{:02}",
                    local_time.year(),
                    local_time.month(),
                    local_time.day(),
                    local_time.hour()
                ));
            }
            TimePoint::Period { .. } => {
                // append y-m-d_h-m
                file_name.push(format!(
                    "_{}-{:02}-{:02}_{:02}-{:02}",
                    local_time.year(),
                    local_time.month(),
                    local_time.day(),
                    local_time.hour(),
                    local_time.minute()
                ));
            }
        }

        let mut path = base_path.to_owned();
        path.set_file_name(file_name);
        if let Some(externsion) = externsion {
            path.set_extension(externsion);
        }

        path
    }
}

impl Rotator for RotatorTimePoint {
    fn log(&self, record: &Record, string_buf: &StringBuf) -> Result<()> {
        let mut inner = self.inner.lock();

        let mut file_path = None;
        let record_time = record.time();
        let should_rotate = record_time >= inner.rotation_time_point;

        if should_rotate {
            file_path = Some(Self::calc_file_path(
                &self.base_path,
                self.time_point,
                record_time,
            ));
            inner.file = BufWriter::new(utils::open_file(file_path.as_ref().unwrap(), true)?);
            inner.rotation_time_point =
                Self::next_rotation_time_point(self.time_point, record_time);
        }

        inner
            .file
            .write_all(string_buf.as_bytes())
            .map_err(Error::WriteRecord)?;

        if should_rotate && inner.file_paths.is_some() {
            self.push_new_remove_old(file_path.unwrap(), &mut inner)?;
        }

        Ok(())
    }

    fn flush(&self) -> Result<()> {
        self.inner.lock().file.flush().map_err(Error::FlushBuffer)
    }
}

impl TimePoint {
    #[must_use]
    fn delta_std(&self) -> Duration {
        match self {
            Self::Daily { .. } => DAY_1,
            Self::Hourly { .. } => HOUR_1,
            Self::Period(duration) => *duration,
        }
    }

    #[must_use]
    fn delta_chrono(&self) -> chrono::Duration {
        match self {
            Self::Daily { .. } => chrono::Duration::days(1),
            Self::Hourly { .. } => chrono::Duration::hours(1),
            Self::Period(duration) => chrono::Duration::from_std(*duration).unwrap(),
        }
    }
}

impl<ArgBP, ArgRP> RotatingFileSinkBuilder<ArgBP, ArgRP> {
    /// Specifies the base path of the log file.
    ///
    /// The path needs to be suffixed with an extension, if you expect the
    /// rotated eventual file names to contain the extension.
    ///
    /// If there is an extension, the different rotation policies will insert
    /// relevant information in the front of the extension. If there is not
    /// an extension, it will be appended to the end.
    ///
    /// Supposes the given base path is `/path/to/base_file.log`, the eventual
    /// file names may look like the following:
    ///
    /// - `/path/to/base_file_1.log`
    /// - `/path/to/base_file_2.log`
    /// - `/path/to/base_file_2022-03-23.log`
    /// - `/path/to/base_file_2022-03-24.log`
    /// - `/path/to/base_file_2022-03-23_03.log`
    /// - `/path/to/base_file_2022-03-23_04.log`
    ///
    /// This parameter is **required**.
    #[must_use]
    pub fn base_path<P>(self, base_path: P) -> RotatingFileSinkBuilder<PathBuf, ArgRP>
    where
        P: Into<PathBuf>,
    {
        RotatingFileSinkBuilder {
            common_builder_impl: self.common_builder_impl,
            base_path: base_path.into(),
            rotation_policy: self.rotation_policy,
            max_files: self.max_files,
            rotate_on_open: self.rotate_on_open,
        }
    }

    /// Specifies the rotation policy.
    ///
    /// This parameter is **required**.
    #[must_use]
    pub fn rotation_policy(
        self,
        rotation_policy: RotationPolicy,
    ) -> RotatingFileSinkBuilder<ArgBP, RotationPolicy> {
        RotatingFileSinkBuilder {
            common_builder_impl: self.common_builder_impl,
            base_path: self.base_path,
            rotation_policy,
            max_files: self.max_files,
            rotate_on_open: self.rotate_on_open,
        }
    }

    /// Specifies the maximum number of files.
    ///
    /// If the number of existing files reaches this parameter, the oldest file
    /// will be deleted on the next rotation.
    ///
    /// Specify `0` for no limit.
    ///
    /// This parameter is **optional**.
    #[must_use]
    pub fn max_files(mut self, max_files: usize) -> Self {
        self.max_files = max_files;
        self
    }

    /// Specifies whether to rotate files once when constructing
    /// `RotatingFileSink`.
    ///
    /// For the [`RotationPolicy::Daily`], [`RotationPolicy::Hourly`], and
    /// [`RotationPolicy::Period`] rotation policies, it may truncate the
    /// contents of the existing file if the parameter is `true`, since the
    /// file name is a time point and not an index.
    ///
    /// This parameter is **optional**.
    #[must_use]
    pub fn rotate_on_open(mut self, rotate_on_open: bool) -> Self {
        self.rotate_on_open = rotate_on_open;
        self
    }

    helper::common_impl!(@SinkBuilder: common_builder_impl);
}

impl<ArgRP> RotatingFileSinkBuilder<(), ArgRP> {
    #[doc(hidden)]
    #[deprecated(note = "\n\n\
        builder compile-time error:\n\
        - missing required parameter `base_path`\n\n\
    ")]
    pub fn build(self, _: Infallible) {}
}

impl RotatingFileSinkBuilder<PathBuf, ()> {
    #[doc(hidden)]
    #[deprecated(note = "\n\n\
        builder compile-time error:\n\
        - missing required parameter `rotation_policy`\n\n\
    ")]
    pub fn build(self, _: Infallible) {}
}

impl RotatingFileSinkBuilder<PathBuf, RotationPolicy> {
    /// Builds a [`RotatingFileSink`].
    ///
    /// # Error
    ///
    /// If the argument `rotation_policy` is invalid, or an error occurs opening
    /// the file, [`Error::CreateDirectory`] or [`Error::OpenFile`] will be
    /// returned.
    pub fn build(self) -> Result<RotatingFileSink> {
        self.build_with_initial_time(None)
    }

    fn build_with_initial_time(self, override_now: Option<SystemTime>) -> Result<RotatingFileSink> {
        self.rotation_policy
            .validate()
            .map_err(|err| Error::InvalidArgument(InvalidArgumentError::RotationPolicy(err)))?;

        let rotator = match self.rotation_policy {
            RotationPolicy::FileSize(max_size) => RotatorKind::FileSize(RotatorFileSize::new(
                self.base_path,
                max_size,
                self.max_files,
                self.rotate_on_open,
            )?),
            RotationPolicy::Daily { hour, minute } => {
                RotatorKind::TimePoint(RotatorTimePoint::new(
                    override_now,
                    self.base_path,
                    TimePoint::Daily { hour, minute },
                    self.max_files,
                    self.rotate_on_open,
                )?)
            }
            RotationPolicy::Hourly => RotatorKind::TimePoint(RotatorTimePoint::new(
                override_now,
                self.base_path,
                TimePoint::Hourly,
                self.max_files,
                self.rotate_on_open,
            )?),
            RotationPolicy::Period(duration) => RotatorKind::TimePoint(RotatorTimePoint::new(
                override_now,
                self.base_path,
                TimePoint::Period(duration),
                self.max_files,
                self.rotate_on_open,
            )?),
        };

        let res = RotatingFileSink {
            common_impl: helper::CommonImpl::from_builder(self.common_builder_impl),
            rotator,
        };

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{prelude::*, test_utils::*, Level, Record};

    static BASE_LOGS_PATH: Lazy<PathBuf> = Lazy::new(|| {
        let path = TEST_LOGS_PATH.join("rotating_file_sink");
        if !path.exists() {
            _ = fs::create_dir(&path);
        }
        path
    });

    const SECOND_1: Duration = Duration::from_secs(1);

    mod policy_file_size {
        use super::*;

        static LOGS_PATH: Lazy<PathBuf> = Lazy::new(|| {
            let path = BASE_LOGS_PATH.join("policy_file_size");
            fs::create_dir_all(&path).unwrap();
            path
        });

        #[test]
        fn calc_file_path() {
            let calc = |base_path, index| {
                RotatorFileSize::calc_file_path(base_path, index)
                    .to_str()
                    .unwrap()
                    .to_string()
            };

            #[cfg(not(windows))]
            let run = || {
                assert_eq!(calc("/tmp/test.log", 0), "/tmp/test.log");
                assert_eq!(calc("/tmp/test", 0), "/tmp/test");

                assert_eq!(calc("/tmp/test.log", 1), "/tmp/test_1.log");
                assert_eq!(calc("/tmp/test", 1), "/tmp/test_1");

                assert_eq!(calc("/tmp/test.log", 23), "/tmp/test_23.log");
                assert_eq!(calc("/tmp/test", 23), "/tmp/test_23");
            };

            #[cfg(windows)]
            let run = || {
                assert_eq!(calc("D:\\tmp\\test.txt", 0), "D:\\tmp\\test.txt");
                assert_eq!(calc("D:\\tmp\\test", 0), "D:\\tmp\\test");

                assert_eq!(calc("D:\\tmp\\test.txt", 1), "D:\\tmp\\test_1.txt");
                assert_eq!(calc("D:\\tmp\\test", 1), "D:\\tmp\\test_1");

                assert_eq!(calc("D:\\tmp\\test.txt", 23), "D:\\tmp\\test_23.txt");
                assert_eq!(calc("D:\\tmp\\test", 23), "D:\\tmp\\test_23");
            };

            run();
        }

        #[test]
        fn rotate() {
            let base_path = LOGS_PATH.join("test.log");

            let build = |clean, rotate_on_open| {
                if clean {
                    fs::remove_dir_all(LOGS_PATH.as_path()).unwrap();
                    if !LOGS_PATH.exists() {
                        fs::create_dir(LOGS_PATH.as_path()).unwrap();
                    }
                }

                let formatter = Box::new(NoModFormatter::new());
                let sink = RotatingFileSink::builder()
                    .base_path(LOGS_PATH.join(&base_path))
                    .rotation_policy(RotationPolicy::FileSize(16))
                    .max_files(3)
                    .rotate_on_open(rotate_on_open)
                    .build()
                    .unwrap();
                sink.set_formatter(formatter);
                let sink = Arc::new(sink);
                let logger = build_test_logger(|b| b.sink(sink.clone()));
                logger.set_level_filter(LevelFilter::All);
                (sink, logger)
            };

            let index_to_path =
                |index| RotatorFileSize::calc_file_path(PathBuf::from(&base_path), index);

            let file_exists = |index| index_to_path(index).exists();
            let files_exists_4 = || {
                (
                    file_exists(0),
                    file_exists(1),
                    file_exists(2),
                    file_exists(3),
                )
            };

            let read_file = |index| fs::read_to_string(index_to_path(index)).ok();
            let read_file_4 = || (read_file(0), read_file(1), read_file(2), read_file(3));

            const STR_4: &str = "abcd";
            const STR_5: &str = "abcde";

            {
                let (sink, logger) = build(true, false);

                assert_eq!(files_exists_4(), (true, false, false, false));
                assert_eq!(sink._current_size(), 0);

                info!(logger: logger, "{}", STR_4);
                assert_eq!(files_exists_4(), (true, false, false, false));
                assert_eq!(sink._current_size(), 4);

                info!(logger: logger, "{}", STR_4);
                assert_eq!(files_exists_4(), (true, false, false, false));
                assert_eq!(sink._current_size(), 8);

                info!(logger: logger, "{}", STR_4);
                assert_eq!(files_exists_4(), (true, false, false, false));
                assert_eq!(sink._current_size(), 12);

                info!(logger: logger, "{}", STR_4);
                assert_eq!(files_exists_4(), (true, false, false, false));
                assert_eq!(sink._current_size(), 16);

                info!(logger: logger, "{}", STR_4);
                assert_eq!(files_exists_4(), (true, true, false, false));
                assert_eq!(sink._current_size(), 4);
            }
            assert_eq!(
                read_file_4(),
                (
                    Some("abcd".to_string()),
                    Some("abcdabcdabcdabcd".to_string()),
                    None,
                    None
                )
            );

            {
                let (sink, logger) = build(true, false);

                assert_eq!(files_exists_4(), (true, false, false, false));
                assert_eq!(sink._current_size(), 0);

                info!(logger: logger, "{}", STR_4);
                info!(logger: logger, "{}", STR_4);
                info!(logger: logger, "{}", STR_4);
                assert_eq!(files_exists_4(), (true, false, false, false));
                assert_eq!(sink._current_size(), 12);

                info!(logger: logger, "{}", STR_5);
                assert_eq!(files_exists_4(), (true, true, false, false));
                assert_eq!(sink._current_size(), 5);
            }
            assert_eq!(
                read_file_4(),
                (
                    Some("abcde".to_string()),
                    Some("abcdabcdabcd".to_string()),
                    None,
                    None
                )
            );

            // test `rotate_on_open` == false
            {
                let (sink, logger) = build(false, false);

                assert_eq!(files_exists_4(), (true, true, false, false));
                assert_eq!(sink._current_size(), 5);

                info!(logger: logger, "{}", STR_5);
                assert_eq!(files_exists_4(), (true, true, false, false));
                assert_eq!(sink._current_size(), 10);
            }
            assert_eq!(
                read_file_4(),
                (
                    Some("abcdeabcde".to_string()),
                    Some("abcdabcdabcd".to_string()),
                    None,
                    None
                )
            );

            // test `rotate_on_open` == true
            {
                let (sink, logger) = build(false, true);

                assert_eq!(files_exists_4(), (true, true, true, false));
                assert_eq!(sink._current_size(), 0);

                info!(logger: logger, "{}", STR_5);
                assert_eq!(files_exists_4(), (true, true, true, false));
                assert_eq!(sink._current_size(), 5);
            }
            assert_eq!(
                read_file_4(),
                (
                    Some("abcde".to_string()),
                    Some("abcdeabcde".to_string()),
                    Some("abcdabcdabcd".to_string()),
                    None
                )
            );

            // test `max_files`
            {
                let (sink, logger) = build(false, true);

                assert_eq!(files_exists_4(), (true, true, true, false));
                assert_eq!(sink._current_size(), 0);

                info!(logger: logger, "{}", STR_4);
                assert_eq!(files_exists_4(), (true, true, true, false));
                assert_eq!(sink._current_size(), 4);
            }
            assert_eq!(
                read_file_4(),
                (
                    Some("abcd".to_string()),
                    Some("abcde".to_string()),
                    Some("abcdeabcde".to_string()),
                    None
                )
            );
        }
    }

    mod policy_time_point {
        use super::*;

        static LOGS_PATH: Lazy<PathBuf> = Lazy::new(|| {
            let path = BASE_LOGS_PATH.join("policy_time_point");
            _ = fs::remove_dir_all(&path);
            fs::create_dir_all(&path).unwrap();
            path
        });

        #[track_caller]
        fn assert_files_count(file_name_prefix: &str, expected: usize) {
            let paths = fs::read_dir(LOGS_PATH.clone()).unwrap();

            let mut filenames = Vec::new();
            let actual = paths.fold(0_usize, |mut count, entry| {
                let filename = entry.unwrap().file_name();
                if filename.to_string_lossy().starts_with(file_name_prefix) {
                    count += 1;
                    filenames.push(filename);
                }
                count
            });
            println!("found files: {:?}", filenames);
            assert_eq!(actual, expected)
        }

        #[test]
        fn calc_file_path() {
            let system_time = Local.with_ymd_and_hms(2012, 3, 4, 5, 6, 7).unwrap().into();

            let calc_daily = |base_path| {
                RotatorTimePoint::calc_file_path(
                    base_path,
                    TimePoint::Daily { hour: 8, minute: 9 },
                    system_time,
                )
                .to_str()
                .unwrap()
                .to_string()
            };

            let calc_hourly = |base_path| {
                RotatorTimePoint::calc_file_path(base_path, TimePoint::Hourly, system_time)
                    .to_str()
                    .unwrap()
                    .to_string()
            };

            let calc_period = |base_path| {
                RotatorTimePoint::calc_file_path(
                    base_path,
                    TimePoint::Period(10 * MINUTE_1),
                    system_time,
                )
                .to_str()
                .unwrap()
                .to_string()
            };

            #[cfg(not(windows))]
            let run = || {
                assert_eq!(calc_daily("/tmp/test.log"), "/tmp/test_2012-03-04.log");
                assert_eq!(calc_daily("/tmp/test"), "/tmp/test_2012-03-04");

                assert_eq!(calc_hourly("/tmp/test.log"), "/tmp/test_2012-03-04_05.log");
                assert_eq!(calc_hourly("/tmp/test"), "/tmp/test_2012-03-04_05");

                assert_eq!(
                    calc_period("/tmp/test.log"),
                    "/tmp/test_2012-03-04_05-06.log"
                );
                assert_eq!(calc_period("/tmp/test"), "/tmp/test_2012-03-04_05-06");
            };

            #[cfg(windows)]
            #[rustfmt::skip]
            let run = || {
                assert_eq!(calc_daily("D:\\tmp\\test.txt"), "D:\\tmp\\test_2012-03-04.txt");
                assert_eq!(calc_daily("D:\\tmp\\test"), "D:\\tmp\\test_2012-03-04");

                assert_eq!(calc_hourly("D:\\tmp\\test.txt"), "D:\\tmp\\test_2012-03-04_05.txt");
                assert_eq!(calc_hourly("D:\\tmp\\test"), "D:\\tmp\\test_2012-03-04_05");

                assert_eq!(calc_period("D:\\tmp\\test.txt"), "D:\\tmp\\test_2012-03-04_05-06.txt");
                assert_eq!(calc_period("D:\\tmp\\test"), "D:\\tmp\\test_2012-03-04_05-06");
            };

            run();
        }

        #[test]
        fn rotate() {
            let build = |rotate_on_open| {
                let hourly_sink = RotatingFileSink::builder()
                    .base_path(LOGS_PATH.join("hourly.log"))
                    .rotation_policy(RotationPolicy::Hourly)
                    .rotate_on_open(rotate_on_open)
                    .build()
                    .unwrap();

                let period_sink = RotatingFileSink::builder()
                    .base_path(LOGS_PATH.join("period.log"))
                    .rotation_policy(RotationPolicy::Period(HOUR_1 + 2 * MINUTE_1 + 3 * SECOND_1))
                    .rotate_on_open(rotate_on_open)
                    .build()
                    .unwrap();

                let local_time_now = Local::now();
                let daily_sink = RotatingFileSink::builder()
                    .base_path(LOGS_PATH.join("daily.log"))
                    .rotation_policy(RotationPolicy::Daily {
                        hour: local_time_now.hour(),
                        minute: local_time_now.minute(),
                    })
                    .rotate_on_open(rotate_on_open)
                    .build()
                    .unwrap();

                let sinks: [Arc<dyn Sink>; 3] = [
                    Arc::new(hourly_sink),
                    Arc::new(period_sink),
                    Arc::new(daily_sink),
                ];
                let logger = build_test_logger(|b| b.sinks(sinks));
                logger.set_level_filter(LevelFilter::All);
                logger
            };

            {
                let logger = build(true);
                let mut record = Record::new(Level::Info, "test log message");
                let initial_time = record.time();

                assert_files_count("hourly", 1);
                assert_files_count("period", 1);
                assert_files_count("daily", 1);

                logger.log(&record);
                assert_files_count("hourly", 1);
                assert_files_count("period", 1);
                assert_files_count("daily", 1);

                record.set_time(record.time() + HOUR_1 + SECOND_1);
                logger.log(&record);
                assert_files_count("hourly", 2);
                assert_files_count("period", 1);
                assert_files_count("daily", 1);

                record.set_time(record.time() + HOUR_1 + SECOND_1);
                logger.log(&record);
                assert_files_count("hourly", 3);
                assert_files_count("period", 2);
                assert_files_count("daily", 1);

                record.set_time(record.time() + SECOND_1);
                logger.log(&record);
                assert_files_count("hourly", 3);
                assert_files_count("period", 2);
                assert_files_count("daily", 1);

                record.set_time(initial_time + DAY_1 + SECOND_1);
                logger.log(&record);
                assert_files_count("hourly", 4);
                assert_files_count("period", 3);
                assert_files_count("daily", 2);
            }
        }

        // This test may only detect issues if the system time zone is not UTC.
        #[test]
        fn respect_local_tz() {
            let prefix = "respect_local_tz";

            let initial_time = Local // FixedOffset::east_opt(8 * 3600).unwrap()
                .with_ymd_and_hms(2024, 8, 29, 11, 45, 14)
                .unwrap();

            let logger = {
                let daily_sink = RotatingFileSink::builder()
                    .base_path(LOGS_PATH.join(format!("{prefix}.log")))
                    .rotation_policy(RotationPolicy::Daily { hour: 0, minute: 0 })
                    .rotate_on_open(true)
                    .build_with_initial_time(Some(initial_time.to_utc().into()))
                    .unwrap();

                build_test_logger(|b| b.sink(Arc::new(daily_sink)).level_filter(LevelFilter::All))
            };

            {
                let mut record = Record::new(Level::Info, "test log message");

                assert_files_count(prefix, 1);

                record.set_time(initial_time.to_utc().into());
                logger.log(&record);
                assert_files_count(prefix, 1);

                record.set_time(record.time() + HOUR_1 + SECOND_1);
                logger.log(&record);
                assert_files_count(prefix, 1);

                record.set_time(record.time() + HOUR_1 + SECOND_1);
                logger.log(&record);
                assert_files_count(prefix, 1);

                record.set_time(
                    initial_time
                        .with_day(30)
                        .unwrap()
                        .with_hour(0)
                        .unwrap()
                        .with_minute(1)
                        .unwrap()
                        .to_utc()
                        .into(),
                );
                logger.log(&record);
                assert_files_count(prefix, 2);

                record.set_time(record.time() + HOUR_1 + SECOND_1);
                logger.log(&record);
                assert_files_count(prefix, 2);

                record.set_time(
                    initial_time
                        .with_day(30)
                        .unwrap()
                        .with_hour(8)
                        .unwrap()
                        .with_minute(2)
                        .unwrap()
                        .to_utc()
                        .into(),
                );
                logger.log(&record);
                assert_files_count(prefix, 2);

                record.set_time(record.time() + HOUR_1 + SECOND_1);
                logger.log(&record);
                assert_files_count(prefix, 2);

                record.set_time(
                    initial_time
                        .with_day(31)
                        .unwrap()
                        .with_hour(0)
                        .unwrap()
                        .to_utc()
                        .into(),
                );
                logger.log(&record);
                assert_files_count(prefix, 3);

                record.set_time(record.time() + HOUR_1 + SECOND_1);
                logger.log(&record);
                assert_files_count(prefix, 3);
            }
        }
    }

    #[test]
    fn test_builder_optional_params() {
        // workaround for the missing `no_run` attribute
        let _ = || {
            let _: Result<RotatingFileSink> = RotatingFileSink::builder()
                .base_path("/path/to/base_log_file")
                .rotation_policy(RotationPolicy::Hourly)
                // .max_files(100)
                // .rotate_on_open(true)
                .build();

            let _: Result<RotatingFileSink> = RotatingFileSink::builder()
                .base_path("/path/to/base_log_file")
                .rotation_policy(RotationPolicy::Hourly)
                .max_files(100)
                // .rotate_on_open(true)
                .build();

            let _: Result<RotatingFileSink> = RotatingFileSink::builder()
                .base_path("/path/to/base_log_file")
                .rotation_policy(RotationPolicy::Hourly)
                // .max_files(100)
                .rotate_on_open(true)
                .build();

            let _: Result<RotatingFileSink> = RotatingFileSink::builder()
                .base_path("/path/to/base_log_file")
                .rotation_policy(RotationPolicy::Hourly)
                .max_files(100)
                .rotate_on_open(true)
                .build();
        };
    }

    #[test]
    fn test_invalid_rotation_policy() {
        use RotationPolicy::*;

        fn daily(hour: u32, minute: u32) -> RotationPolicy {
            Daily { hour, minute }
        }
        fn period(duration: Duration) -> RotationPolicy {
            Period(duration)
        }

        assert!(FileSize(1).validate().is_ok());
        assert!(FileSize(1024).validate().is_ok());
        assert!(FileSize(u64::MAX).validate().is_ok());
        assert!(FileSize(0).validate().is_err());

        assert!(daily(0, 0).validate().is_ok());
        assert!(daily(15, 30).validate().is_ok());
        assert!(daily(23, 59).validate().is_ok());
        assert!(daily(24, 59).validate().is_err());
        assert!(daily(23, 60).validate().is_err());
        assert!(daily(24, 60).validate().is_err());

        assert!(period(Duration::from_secs(0)).validate().is_err());
        assert!(period(SECOND_1).validate().is_err());
        assert!(period(59 * SECOND_1).validate().is_err());
        assert!(period(MINUTE_1).validate().is_ok());
        assert!(period(HOUR_1).validate().is_ok());
        assert!(period(HOUR_1 + MINUTE_1 + SECOND_1).validate().is_ok());
        assert!(period(60 * HOUR_1 + MINUTE_1 + SECOND_1).validate().is_ok());
        assert!(period(2 * DAY_1 + 60 * HOUR_1 + MINUTE_1 + SECOND_1)
            .validate()
            .is_ok());
    }
}
