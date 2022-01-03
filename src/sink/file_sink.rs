//! Provides a file sink.
use std::{
    fs::{self, File, OpenOptions},
    io::{BufWriter, Write},
    path::Path,
};

use crate::{
    formatter::{BasicFormatter, Formatter},
    sink::Sink,
    Error, LevelFilter, Record, Result, StringBuf,
};

/// A sink with a file as the target.
pub struct FileSink {
    level_filter: LevelFilter,
    formatter: Box<dyn Formatter>,
    file: spin::Mutex<BufWriter<File>>,
}

impl FileSink {
    /// Constructs a [`FileSink`].
    pub fn new<P>(path: P, truncate: bool) -> Result<FileSink>
    where
        P: AsRef<Path>,
    {
        if let Some(parent) = path.as_ref().parent() {
            fs::create_dir_all(parent).map_err(Error::CreateDirectory)?;
        }

        let mut open_options = OpenOptions::new();
        if truncate {
            open_options.write(true).truncate(true);
        } else {
            open_options.append(true);
        }
        let file = open_options
            .create(true)
            .open(path)
            .map_err(Error::OpenFile)?;

        let sink = FileSink {
            level_filter: LevelFilter::All,
            formatter: Box::new(BasicFormatter::new()),
            file: spin::Mutex::new(BufWriter::new(file)),
        };

        Ok(sink)
    }
}

impl Sink for FileSink {
    fn log(&self, record: &Record) -> Result<()> {
        let mut string_buf = StringBuf::new();
        self.formatter.format(record, &mut string_buf)?;

        let mut file = self.file.lock();

        file.write_all(string_buf.as_bytes())
            .map_err(Error::WriteRecord)?;
        file.write_all(b"\n").map_err(Error::WriteRecord)?;

        Ok(())
    }

    fn flush(&self) -> Result<()> {
        self.file.lock().flush().map_err(Error::FlushBuffer)
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

impl Drop for FileSink {
    fn drop(&mut self) {
        if let Err(err) = self.file.lock().flush() {
            // Sinks do not have an error handler, because it would increase complexity and
            // the error is not common. So currently users cannot handle this error by
            // themselves.
            crate::default_error_handler("FileSink", Error::FlushBuffer(err));
        }
    }
}
