//! Provides a file sink.
use std::{
    fs::{self, File, OpenOptions},
    io::{BufWriter, Write},
    path::Path,
};

use crate::{
    formatter::{BasicFormatter, Formatter},
    sink::Sink,
    LevelFilter, Record, Result, StringBuf,
};

/// A sink with a file as the target.
pub struct FileSink {
    level: LevelFilter,
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
            fs::create_dir_all(parent)?;
        }

        let mut open_options = OpenOptions::new();
        if truncate {
            open_options.write(true).truncate(true);
        } else {
            open_options.append(true);
        }
        let file = open_options.create(true).open(path)?;

        let sink = FileSink {
            level: LevelFilter::max(),
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
        writeln!(file, "{}", string_buf)?;

        Ok(())
    }

    fn flush(&self) -> Result<()> {
        self.file.lock().flush()?;
        Ok(())
    }

    fn level(&self) -> LevelFilter {
        self.level
    }

    fn set_level(&mut self, level: LevelFilter) {
        self.level = level;
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
            crate::default_error_handler("FileSink", err.into());
        }
    }
}
