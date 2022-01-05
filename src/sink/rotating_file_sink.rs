//! Provides a rotating file sink.

use std::{
    ffi::OsString,
    fs::{self, File},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use crate::{
    formatter::{Formatter, FullFormatter},
    sink::Sink,
    utils, Error, LevelFilter, Record, Result, StringBuf,
};

/// A sink with a file as the target, rotating according to the file size.
///
/// When the max file size is reached, close the file, rename it, and create a new file.
pub struct RotatingFileSink {
    level_filter: LevelFilter,
    formatter: Box<dyn Formatter>,
    opened_file: spin::Mutex<OpenedFile>,
    base_path: PathBuf,
    max_size: u64,
    max_files: usize,
}

struct OpenedFile {
    file: Option<BufWriter<File>>,
    current_size: u64,
}

impl OpenedFile {
    fn new(file: BufWriter<File>, current_size: u64) -> Self {
        Self {
            file: Some(file),
            current_size,
        }
    }
}

impl RotatingFileSink {
    /// Constructs a `RotatingFileSink`.
    pub fn new<P>(
        base_path: P,
        max_size: u64,
        max_files: usize,
        rotate_on_open: bool,
    ) -> Result<Self>
    where
        P: Into<PathBuf>,
    {
        let base_path = base_path.into();
        let file = utils::open_file(&base_path, false)?;
        let current_size = file.metadata().map_err(Error::QueryFileMetadata)?.len();

        let res = Self {
            level_filter: LevelFilter::All,
            formatter: Box::new(FullFormatter::new()),
            opened_file: spin::Mutex::new(OpenedFile::new(BufWriter::new(file), current_size)),
            base_path,
            max_size,
            max_files,
        };

        if rotate_on_open && current_size > 0 {
            res.rotate(&mut res.opened_file.lock())?;
            res.opened_file.lock().current_size = 0;
        }

        Ok(res)
    }

    fn reopen(&self) -> Result<File> {
        // always truncate
        utils::open_file(&self.base_path, true)
    }

    fn rotate(&self, opened_file: &mut spin::MutexGuard<OpenedFile>) -> Result<()> {
        let inner = || {
            for i in (1..=self.max_files).rev() {
                let src = Self::calc_file_name(&self.base_path, i - 1);
                if !src.exists() {
                    continue;
                }

                let dst = Self::calc_file_name(&self.base_path, i);
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

    fn calc_file_name<P>(base_path: P, index: usize) -> PathBuf
    where
        P: AsRef<Path>,
    {
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

    // if `self.opened_file.file` is `None`, try to reopen the file.
    fn lock_opened_file(&self) -> Result<spin::MutexGuard<OpenedFile>> {
        let mut opened_file = self.opened_file.lock();
        if opened_file.file.is_none() {
            opened_file.file = Some(BufWriter::new(self.reopen()?));
        }
        Ok(opened_file)
    }
}

impl Sink for RotatingFileSink {
    fn log(&self, record: &Record) -> Result<()> {
        let mut string_buf = StringBuf::new();
        self.formatter.format(record, &mut string_buf)?;

        let mut opened_file = self.lock_opened_file()?;

        opened_file.current_size += string_buf.len() as u64;
        if opened_file.current_size > self.max_size {
            self.rotate(&mut opened_file)?;
            opened_file.current_size = string_buf.len() as u64;
        }

        opened_file
            .file
            .as_mut()
            .unwrap()
            .write_all(string_buf.as_bytes())
            .map_err(Error::WriteRecord)?;

        Ok(())
    }

    fn flush(&self) -> Result<()> {
        let mut opened_file = self.lock_opened_file()?;
        opened_file
            .file
            .as_mut()
            .unwrap()
            .flush()
            .map_err(Error::OpenFile)
    }

    fn level_filter(&self) -> LevelFilter {
        self.level_filter
    }

    fn set_level_filter(&mut self, level_filter: LevelFilter) {
        self.level_filter = level_filter;
    }

    fn formatter(&self) -> &dyn Formatter {
        self.formatter.as_ref()
    }

    fn set_formatter(&mut self, formatter: Box<dyn Formatter>) {
        self.formatter = formatter;
    }
}

impl Drop for RotatingFileSink {
    fn drop(&mut self) {
        let mut opened_file = self.opened_file.lock();
        if let Some(opened_file) = opened_file.file.as_mut() {
            if let Err(err) = opened_file.flush() {
                // Sinks do not have an error handler, because it would increase complexity and
                // the error is not common. So currently users cannot handle this error by
                // themselves.
                crate::default_error_handler("RotatingFileSink", Error::FlushBuffer(err));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{prelude::*, test_utils::*};

    use std::sync::Arc;

    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref LOGS_PATH: PathBuf = {
            let path = TEST_LOGS_PATH.join("rotating_file_sink");
            fs::create_dir_all(&path).unwrap();
            path
        };
    }

    #[test]
    fn calc_file_name() {
        let calc = |base_path, index| {
            RotatingFileSink::calc_file_name(base_path, index)
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
        let base_path = LOGS_PATH.join("RotatingFileSink.log");

        let build = |clean, rotate_on_open| {
            if clean {
                fs::remove_dir_all(LOGS_PATH.as_path()).unwrap();
                fs::create_dir(LOGS_PATH.as_path()).unwrap();
            }

            let formatter = Box::new(NoModFormatter::new());
            let mut sink =
                RotatingFileSink::new(LOGS_PATH.join(&base_path), 16, 3, rotate_on_open).unwrap();
            sink.set_formatter(formatter);
            let sink = Arc::new(sink);
            let mut logger = test_logger_builder().sink(sink.clone()).build();
            logger.set_level_filter(LevelFilter::All);
            (sink, logger)
        };

        let index_to_path =
            |index| RotatingFileSink::calc_file_name(PathBuf::from(&base_path), index);

        let file_exists = |index| index_to_path(index).exists();
        let files_exists_3 = || (file_exists(0), file_exists(1), file_exists(2));

        let read_file = |index| fs::read_to_string(index_to_path(index)).ok();
        let read_file_3 = || (read_file(0), read_file(1), read_file(2));

        const STR_4: &'static str = "abcd";
        const STR_5: &'static str = "abcde";

        {
            let (sink, logger) = build(true, false);

            assert_eq!(files_exists_3(), (true, false, false));
            assert_eq!(sink.opened_file.lock().current_size, 0);

            info!(logger: logger, "{}", STR_4);
            assert_eq!(files_exists_3(), (true, false, false));
            assert_eq!(sink.opened_file.lock().current_size, 4);

            info!(logger: logger, "{}", STR_4);
            assert_eq!(files_exists_3(), (true, false, false));
            assert_eq!(sink.opened_file.lock().current_size, 8);

            info!(logger: logger, "{}", STR_4);
            assert_eq!(files_exists_3(), (true, false, false));
            assert_eq!(sink.opened_file.lock().current_size, 12);

            info!(logger: logger, "{}", STR_4);
            assert_eq!(files_exists_3(), (true, false, false));
            assert_eq!(sink.opened_file.lock().current_size, 16);

            info!(logger: logger, "{}", STR_4);
            assert_eq!(files_exists_3(), (true, true, false));
            assert_eq!(sink.opened_file.lock().current_size, 4);
        }
        assert_eq!(
            read_file_3(),
            (
                Some("abcd".to_string()),
                Some("abcdabcdabcdabcd".to_string()),
                None
            )
        );

        {
            let (sink, logger) = build(true, false);

            assert_eq!(files_exists_3(), (true, false, false));
            assert_eq!(sink.opened_file.lock().current_size, 0);

            info!(logger: logger, "{}", STR_4);
            info!(logger: logger, "{}", STR_4);
            info!(logger: logger, "{}", STR_4);
            assert_eq!(files_exists_3(), (true, false, false));
            assert_eq!(sink.opened_file.lock().current_size, 12);

            info!(logger: logger, "{}", STR_5);
            assert_eq!(files_exists_3(), (true, true, false));
            assert_eq!(sink.opened_file.lock().current_size, 5);
        }
        assert_eq!(
            read_file_3(),
            (
                Some("abcde".to_string()),
                Some("abcdabcdabcd".to_string()),
                None
            )
        );

        {
            let (sink, logger) = build(false, false);

            assert_eq!(files_exists_3(), (true, true, false));
            assert_eq!(sink.opened_file.lock().current_size, 5);

            info!(logger: logger, "{}", STR_5);
            assert_eq!(files_exists_3(), (true, true, false));
            assert_eq!(sink.opened_file.lock().current_size, 10);
        }
        assert_eq!(
            read_file_3(),
            (
                Some("abcdeabcde".to_string()),
                Some("abcdabcdabcd".to_string()),
                None
            )
        );

        {
            let (sink, logger) = build(false, true);

            assert_eq!(files_exists_3(), (true, true, true));
            assert_eq!(sink.opened_file.lock().current_size, 0);

            info!(logger: logger, "{}", STR_5);
            assert_eq!(files_exists_3(), (true, true, true));
            assert_eq!(sink.opened_file.lock().current_size, 5);
        }
        assert_eq!(
            read_file_3(),
            (
                Some("abcde".to_string()),
                Some("abcdeabcde".to_string()),
                Some("abcdabcdabcd".to_string()),
            )
        );
    }
}
