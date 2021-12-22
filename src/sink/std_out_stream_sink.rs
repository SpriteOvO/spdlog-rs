use std::{io::Write, sync::Mutex};

use crate::{
    formatter::{BasicFormatter, Formatter},
    sink::Sink,
    LevelFilter, LogMsg, Result, StrBuf,
};

/// A standard output stream sink.
///
/// For internal use, users should not use it directly.
pub struct StdOutStreamSink<S>
where
    S: Write + Send + Sync,
{
    level: LevelFilter,
    formatter: Box<dyn Formatter>,
    out_stream: Mutex<S>,
}

impl<S> StdOutStreamSink<S>
where
    S: Write + Send + Sync,
{
    /// Constructs a [`StdOutStreamSink`].
    ///
    /// Level default maximum (no discard)
    pub fn new(out_stream: S) -> StdOutStreamSink<S> {
        StdOutStreamSink {
            level: LevelFilter::max(),
            formatter: Box::new(BasicFormatter::new()),
            out_stream: Mutex::new(out_stream),
        }
    }
}

impl<S> Sink for StdOutStreamSink<S>
where
    S: Write + Send + Sync,
{
    fn log(&self, msg: &LogMsg) -> Result<()> {
        let mut str_buf = StrBuf::new();
        self.formatter.format(msg, &mut str_buf)?;

        let mut out_stream = self.out_stream.lock().unwrap();
        writeln!(out_stream, "{}", str_buf)?;

        out_stream.flush()?;
        Ok(())
    }

    fn flush(&self) -> Result<()> {
        self.out_stream.lock().unwrap().flush()?;
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

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use log::RecordBuilder;

    use super::*;
    use crate::Level;

    #[test]
    fn log() {
        let mut out_stream = Vec::<u8>::new();

        let sink = StdOutStreamSink::new(&mut out_stream);

        let record = (
            RecordBuilder::new()
                .level(Level::Warn)
                .target("target")
                .args(format_args!("test log content 0"))
                .build(),
            RecordBuilder::new()
                .level(Level::Info)
                .target("target")
                .args(format_args!("test log content 1"))
                .build(),
        );

        let msg = (LogMsg::new(&record.0), LogMsg::new(&record.1));

        sink.log(&msg.0).unwrap();
        sink.log(&msg.1).unwrap();

        assert_eq!(
            format!(
                "[{}] [target] [WARN] test log content 0\n\
                 [{}] [target] [INFO] test log content 1\n",
                Into::<DateTime::<Local>>::into(msg.0.time().clone())
                    .format("%Y-%m-%d %H:%M:%S.%3f"),
                Into::<DateTime::<Local>>::into(msg.1.time().clone())
                    .format("%Y-%m-%d %H:%M:%S.%3f")
            )
            .as_bytes(),
            out_stream
        );
    }
}
