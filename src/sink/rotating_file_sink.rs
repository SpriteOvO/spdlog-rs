//! Provides a rotating file sink.

use std::{
    collections::LinkedList,
    convert::Infallible,
    ffi::OsString,
    fs::{self, File},
    hash::Hash,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    time::{Duration, SystemTime},
};

use chrono::prelude::*;

use crate::{
    sink::{helper, Sink},
    sync::*,
    utils, Error, Record, Result, StringBuf,
};

/// Rotation policies for [`RotatingFileSink`].
///
/// # Panics
///
/// Note that some parameters have range requirements, functions that receive it
/// will panic if the requirements are not met.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum RotationPolicy {
    /// Rotates when the log file reaches the given max file size.
    FileSize(
        /// Max file size (in bytes). Range: (0, u64::MAX].
        u64,
    ),
    /// Rotates daily at the given time point.
    Daily {
        /// Hour of the time point. Range: [0, 23].
        hour: u32,
        /// Minute of the time point. Range: [0, 59].
        minute: u32,
    },
    /// Rotates hourly.
    Hourly,
}

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
}

struct RotatorTimePointInner {
    file: BufWriter<File>,
    rotation_time_point: SystemTime,
    file_paths: Option<LinkedList<PathBuf>>,
}

/// A sink with a file as the target, rotating according to the rotation policy.
///
/// # Examples
///
/// See [./examples] directory.
///
/// [./examples]: https://github.com/SpriteOvO/spdlog-rs/tree/main/examples
pub struct RotatingFileSink {
    common_impl: helper::CommonImpl,
    rotator: RotatorKind,
}

/// The builder of [`RotatingFileSink`].
#[doc = include_str!("../include/doc/generic-builder-note.md")]
///
/// # Examples
///
/// - Building a [`RotatingFileSink`].
///
///   ```no_run
///   use spdlog::sink::{RotatingFileSink, RotationPolicy};
///  
///   let sink: spdlog::Result<RotatingFileSink> = RotatingFileSink::builder()
///       .base_path("/path/to/base_log_file") // required
///       .rotation_policy(RotationPolicy::Hourly) // required
///       // .max_files(100) // optional, defaults to `0` for no limit
///       // .rotate_on_open(true) // optional, defaults to `false`
///       .build();
///   ```
///
/// - If any required parameters are missing, a compile-time error will be
///   raised.
///
///   ```compile_fail
///   use spdlog::sink::{RotatingFileSink, RotationPolicy};
///  
///   let sink: spdlog::Result<RotatingFileSink> = RotatingFileSink::builder()
///       // .base_path("/path/to/base_log_file") // required
///       .rotation_policy(RotationPolicy::Hourly) // required
///       .max_files(100) // optional, defaults to `0` for no limit
///       .rotate_on_open(true) // optional, defaults to `false`
///       .build();
///   ```
///
///   ```compile_fail
///   use spdlog::sink::{RotatingFileSink, RotationPolicy};
///  
///   let sink: spdlog::Result<RotatingFileSink> = RotatingFileSink::builder()
///       .base_path("/path/to/base_log_file") // required
///       // .rotation_policy(RotationPolicy::Hourly) // required
///       .max_files(100) // optional, defaults to `0` for no limit
///       .rotate_on_open(true) // optional, defaults to `false`
///       .build();
///   ```
pub struct RotatingFileSinkBuilder<ArgBP, ArgRP> {
    common_builder_impl: helper::CommonBuilderImpl,
    base_path: ArgBP,
    rotation_policy: ArgRP,
    max_files: usize,
    rotate_on_open: bool,
}

impl RotatingFileSink {
    /// Constructs a builder of `RotatingFileSink`.
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
    /// when constructing `RotatingFileSink`. For the [`RotationPolicy::Daily`]
    /// and [`RotationPolicy::Hourly`] rotation policies, it may truncate the
    /// contents of the existing file if the parameter is `true`, since the file
    /// name is a time point and not an index.
    ///
    /// # Errors
    ///
    /// If an error occurs opening the file, [`Error::CreateDirectory`] or
    /// [`Error::OpenFile`] will be returned.
    ///
    /// # Panics
    ///
    /// Panics if the parameter `rotation_policy` is invalid. See the
    /// documentation of [`RotationPolicy`] for requirements.
    #[deprecated(
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
        if !self.should_log(record.level()) {
            return Ok(());
        }

        let mut string_buf = StringBuf::new();
        self.common_impl
            .formatter
            .read()
            .format(record, &mut string_buf)?;

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
                .non_throwable_error("RotatingFileSink", err)
        }
    }
}

impl RotationPolicy {
    fn validate(&self) {
        match self {
            Self::FileSize(max_size) => {
                if *max_size == 0 {
                    panic!(
                        "invalid rotation policy. (FileSize) \
                         expect `max_size` to be (0, u64::MAX] but {}",
                        *max_size
                    );
                }
            }
            Self::Daily { hour, minute } => {
                if *hour > 23 || *minute > 59 {
                    panic!(
                        "invalid rotation policy. (Daily) \
                         expect (`hour`, `minute`) to be ([0, 23], [0, 59]) but ({}, {})",
                        *hour, *minute
                    );
                }
            }
            Self::Hourly => {}
        }
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
    fn new(file: File, current_size: u64) -> Self {
        Self {
            file: Some(BufWriter::new(file)),
            current_size,
        }
    }
}

impl RotatorTimePoint {
    fn new(
        base_path: PathBuf,
        time_point: TimePoint,
        max_files: usize,
        truncate: bool,
    ) -> Result<Self> {
        let now = SystemTime::now();
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
    fn next_rotation_time_point(time_point: TimePoint, now: SystemTime) -> SystemTime {
        let now: DateTime<Utc> = now.into();
        let mut rotation_time: DateTime<Utc> = now;

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
        };

        if rotation_time < now {
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
    fn delta_std(&self) -> Duration {
        const HOUR_1: Duration = Duration::from_secs(60 * 60);
        const DAY_1: Duration = Duration::from_secs(60 * 60 * 24);

        match self {
            Self::Daily { .. } => DAY_1,
            Self::Hourly { .. } => HOUR_1,
        }
    }

    fn delta_chrono(&self) -> chrono::Duration {
        match self {
            Self::Daily { .. } => chrono::Duration::days(1),
            Self::Hourly { .. } => chrono::Duration::hours(1),
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
    /// This parameter is required.
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
    /// This parameter is required.
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
    /// Pass `0` for no limit.
    ///
    /// This parameter is optional, and defaults to `0`.
    pub fn max_files(mut self, max_files: usize) -> Self {
        self.max_files = max_files;
        self
    }

    /// Specifies whether to rotate files once when constructing
    /// `RotatingFileSink`.
    ///
    /// For the [`RotationPolicy::Daily`] and [`RotationPolicy::Hourly`]
    /// rotation policies, it may truncate the contents of the existing file if
    /// the parameter is `true`, since the file name is a time point and not an
    /// index.
    ///
    /// This parameter is optional, and defaults to `false`.
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
        - missing required field `base_path`\n\n\
    ")]
    pub fn build(self, _: Infallible) {}
}

impl RotatingFileSinkBuilder<PathBuf, ()> {
    #[doc(hidden)]
    #[deprecated(note = "\n\n\
        builder compile-time error:\n\
        - missing required field `rotation_policy`\n\n\
    ")]
    pub fn build(self, _: Infallible) {}
}

impl RotatingFileSinkBuilder<PathBuf, RotationPolicy> {
    /// Builds a [`RotatingFileSink`].
    ///
    /// # Errors
    ///
    /// If an error occurs opening the file, [`Error::CreateDirectory`] or
    /// [`Error::OpenFile`] will be returned.
    ///
    /// # Panics
    ///
    /// Panics if the parameter `rotation_policy` is invalid. See the
    /// documentation of [`RotationPolicy`] for requirements.
    pub fn build(self) -> Result<RotatingFileSink> {
        self.rotation_policy.validate();

        let rotator = match self.rotation_policy {
            RotationPolicy::FileSize(max_size) => RotatorKind::FileSize(RotatorFileSize::new(
                self.base_path,
                max_size,
                self.max_files,
                self.rotate_on_open,
            )?),
            RotationPolicy::Daily { hour, minute } => {
                RotatorKind::TimePoint(RotatorTimePoint::new(
                    self.base_path,
                    TimePoint::Daily { hour, minute },
                    self.max_files,
                    self.rotate_on_open,
                )?)
            }
            RotationPolicy::Hourly => RotatorKind::TimePoint(RotatorTimePoint::new(
                self.base_path,
                TimePoint::Hourly,
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
        fs::create_dir_all(&path).unwrap();
        path
    });

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
                    fs::create_dir(LOGS_PATH.as_path()).unwrap();
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
                let logger = test_logger_builder().sink(sink.clone()).build();
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
            fs::create_dir_all(&path).unwrap();
            path
        });

        #[test]
        fn calc_file_path() {
            let system_time = Local.ymd(2012, 3, 4).and_hms(5, 6, 7).into();

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

            #[cfg(not(windows))]
            let run = || {
                assert_eq!(calc_daily("/tmp/test.log"), "/tmp/test_2012-03-04.log");
                assert_eq!(calc_daily("/tmp/test"), "/tmp/test_2012-03-04");

                assert_eq!(calc_hourly("/tmp/test.log"), "/tmp/test_2012-03-04_05.log");
                assert_eq!(calc_hourly("/tmp/test"), "/tmp/test_2012-03-04_05");
            };

            #[cfg(windows)]
            #[rustfmt::skip]
            let run = || {
                assert_eq!(calc_daily("D:\\tmp\\test.txt"), "D:\\tmp\\test_2012-03-04.txt");
                assert_eq!(calc_daily("D:\\tmp\\test"), "D:\\tmp\\test_2012-03-04");

                assert_eq!(calc_hourly("D:\\tmp\\test.txt"), "D:\\tmp\\test_2012-03-04_05.txt");
                assert_eq!(calc_hourly("D:\\tmp\\test"), "D:\\tmp\\test_2012-03-04_05");
            };

            run();
        }

        #[test]
        fn rotate() {
            let build = |rotate_on_open| {
                fs::remove_dir_all(LOGS_PATH.as_path()).unwrap();
                fs::create_dir(LOGS_PATH.as_path()).unwrap();

                let hourly_sink = RotatingFileSink::builder()
                    .base_path(LOGS_PATH.join("hourly.log"))
                    .rotation_policy(RotationPolicy::Hourly)
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

                let sinks: [Arc<dyn Sink>; 2] = [Arc::new(hourly_sink), Arc::new(daily_sink)];
                let logger = test_logger_builder().sinks(sinks).build();
                logger.set_level_filter(LevelFilter::All);
                logger
            };

            let exist_files = |file_name_prefix| {
                let paths = fs::read_dir(LOGS_PATH.clone()).unwrap();

                paths.fold(0_usize, |count, entry| {
                    if entry
                        .unwrap()
                        .file_name()
                        .to_string_lossy()
                        .starts_with(file_name_prefix)
                    {
                        count + 1
                    } else {
                        count
                    }
                })
            };

            let exist_hourly_files = || exist_files("hourly");
            let exist_daily_files = || exist_files("daily");

            const SECOND_1: Duration = Duration::from_secs(1);
            const HOUR_1: Duration = Duration::from_secs(60 * 60);
            const DAY_1: Duration = Duration::from_secs(60 * 60 * 24);

            {
                let logger = build(true);
                let mut record = Record::new(Level::Info, "test log message");
                let initial_time = record.time();

                assert_eq!(exist_hourly_files(), 1);
                assert_eq!(exist_daily_files(), 1);

                logger.log(&record);
                assert_eq!(exist_hourly_files(), 1);
                assert_eq!(exist_daily_files(), 1);

                record.set_time(record.time() + HOUR_1 + SECOND_1);
                logger.log(&record);
                assert_eq!(exist_hourly_files(), 2);
                assert_eq!(exist_daily_files(), 1);

                record.set_time(record.time() + HOUR_1 + SECOND_1);
                logger.log(&record);
                assert_eq!(exist_hourly_files(), 3);
                assert_eq!(exist_daily_files(), 1);

                record.set_time(record.time() + SECOND_1);
                logger.log(&record);
                assert_eq!(exist_hourly_files(), 3);
                assert_eq!(exist_daily_files(), 1);

                record.set_time(initial_time + DAY_1 + SECOND_1);
                logger.log(&record);
                assert_eq!(exist_hourly_files(), 4);
                assert_eq!(exist_daily_files(), 2);
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
}
