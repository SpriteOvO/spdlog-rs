//! Provides a file sink.
use std::{
    fs::{self, File, OpenOptions},
    io::{BufWriter, Write},
    path::Path,
    sync::Mutex,
};

use crate::{
    formatter::{BasicFormatter, Formatter},
    sink::Sink,
    LevelFilter, LogMsg, Result, StrBuf,
};

/// A sink with a file as the target.
pub struct FileSink {
    level: LevelFilter,
    formatter: Box<dyn Formatter>,
    file: Mutex<BufWriter<File>>,
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
            file: Mutex::new(BufWriter::new(file)),
        };

        Ok(sink)
    }
}

impl Sink for FileSink {
    fn log(&self, msg: &LogMsg) -> Result<()> {
        let mut str_buf = StrBuf::new();
        self.formatter.format(msg, &mut str_buf)?;

        let mut file = self.file.lock().unwrap();
        writeln!(file, "{}", str_buf)?;

        Ok(())
    }

    fn flush(&self) -> Result<()> {
        self.file.lock().unwrap().flush()?;
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
