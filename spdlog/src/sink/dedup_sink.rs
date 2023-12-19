use std::{cmp::Ordering, convert::Infallible, sync::Arc, time::Duration};

use crate::{
    sink::{helper, Sink, Sinks},
    sync::*,
    Error, Record, RecordOwned, Result,
};

struct DedupSinkState {
    last_record: Option<RecordOwned>,
    skipped_count: usize,
}

/// A [combined sink], skip consecutive repeated records.
///
/// More than 2 consecutive repeated records, the records after the first one
/// will be replaced with a single record `"(skipped {count} duplicates)"`.
///
/// The skip will stop if the incoming record compares to the last skipped
/// records:
/// - content changed, or
/// - logging level changed, or
/// - interval exceeded the skip duration
///
/// # Example
///
/// ```
/// use std::time::Duration;
///
/// use spdlog::{prelude::*, sink::DedupSink};
/// # use std::sync::Arc;
/// # use spdlog::{
/// #     formatter::{pattern, PatternFormatter},
/// #     sink::WriteSink,
/// # };
/// #
/// # fn main() -> Result<(), spdlog::Error> {
/// # let underlying_sink = Arc::new(
/// #     WriteSink::builder()
/// #         .formatter(Box::new(PatternFormatter::new(pattern!("{payload}\n"))))
/// #         .target(Vec::new())
/// #         .build()?
/// # );
///
/// # let sink = {
/// #     let underlying_sink = underlying_sink.clone();
/// let sink = Arc::new(
///     DedupSink::builder()
///         .sink(underlying_sink)
///         .skip_duration(Duration::from_secs(1))
///         .build()?
/// );
/// #     sink
/// # };
/// # let doctest = Logger::builder().sink(sink).build()?;
///
/// // ... Add the `sink` to a logger
///
/// info!(logger: doctest, "I wish I was a cat");
/// info!(logger: doctest, "I wish I was a cat");
/// info!(logger: doctest, "I wish I was a cat");
/// // The skip will stop since the content changed.
/// info!(logger: doctest, "No school");
/// info!(logger: doctest, "No works");
/// info!(logger: doctest, "Just meow meow");
///
/// # assert_eq!(
/// #     String::from_utf8(underlying_sink.clone_target()).unwrap(),
/// /* Output of `underlying_sink` */
/// r#"I wish I was a cat
/// (skipped 2 duplicates)
/// No school
/// No works
/// Just meow meow
/// "#
/// # );
/// # Ok(()) }
/// ```
///
/// [combined sink]: index.html#combined-sink
pub struct DedupSink {
    common_impl: helper::CommonImpl,
    sinks: Sinks,
    skip_duration: Duration,
    state: Mutex<DedupSinkState>,
}

impl DedupSink {
    /// Constructs a builder of `DedupSink`.
    #[must_use]
    pub fn builder() -> DedupSinkBuilder<()> {
        DedupSinkBuilder {
            common_builder_impl: helper::CommonBuilderImpl::new(),
            sinks: vec![],
            skip_duration: (),
        }
    }

    /// Gets a reference to internal sinks in the combined sink.
    #[must_use]
    pub fn sinks(&self) -> &[Arc<dyn Sink>] {
        &self.sinks
    }

    #[must_use]
    fn is_dup_record(&self, last_record: Option<Record>, other: &Record) -> bool {
        if let Some(last_record) = last_record {
            last_record.payload() == other.payload()
                && last_record.level() == other.level()
                && other.time().duration_since(last_record.time()).unwrap() < self.skip_duration
        } else {
            false
        }
    }

    fn log_record(&self, record: &Record) -> Result<()> {
        #[allow(clippy::manual_try_fold)] // https://github.com/rust-lang/rust-clippy/issues/11554
        self.sinks.iter().fold(Ok(()), |result, sink| {
            Error::push_result(result, sink.log(record))
        })
    }

    fn flush_sinks(&self) -> Result<()> {
        #[allow(clippy::manual_try_fold)] // https://github.com/rust-lang/rust-clippy/issues/11554
        self.sinks.iter().fold(Ok(()), |result, sink| {
            Error::push_result(result, sink.flush())
        })
    }
}

impl Sink for DedupSink {
    fn log(&self, record: &Record) -> Result<()> {
        let mut state = self.state.lock_expect();

        if self.is_dup_record(state.last_record.as_ref().map(|r| r.as_ref()), record) {
            state.skipped_count += 1;
            return Ok(());
        }

        if state.skipped_count != 0 {
            let last_record = state.last_record.as_ref().unwrap().as_ref();
            match state.skipped_count.cmp(&1) {
                Ordering::Equal => self.log_record(&last_record)?,
                Ordering::Greater => self.log_record(
                    &last_record
                        .replace_payload(format!("(skipped {} duplicates)", state.skipped_count)),
                )?,
                Ordering::Less => unreachable!(), // We have checked if `state.skipped_count != 0`
            }
        }

        self.log_record(record)?;
        state.skipped_count = 0;
        state.last_record = Some(record.to_owned());

        Ok(())
    }

    fn flush(&self) -> Result<()> {
        self.flush_sinks()
    }

    helper::common_impl!(@Sink: common_impl);
}

/// The builder of [`DedupSink`].
pub struct DedupSinkBuilder<ArgS> {
    common_builder_impl: helper::CommonBuilderImpl,
    sinks: Sinks,
    skip_duration: ArgS,
}

impl<ArgS> DedupSinkBuilder<ArgS> {
    /// Add a [`Sink`].
    #[must_use]
    pub fn sink(mut self, sink: Arc<dyn Sink>) -> Self {
        self.sinks.push(sink);
        self
    }

    /// Add multiple [`Sink`]s.
    #[must_use]
    pub fn sinks<I>(mut self, sinks: I) -> Self
    where
        I: IntoIterator<Item = Arc<dyn Sink>>,
    {
        self.sinks.append(&mut sinks.into_iter().collect());
        self
    }

    /// Only consecutive repeated records within the given duration will be
    /// skipped.
    ///
    /// This parameter is **required**.
    #[must_use]
    pub fn skip_duration(self, duration: Duration) -> DedupSinkBuilder<Duration> {
        DedupSinkBuilder {
            common_builder_impl: self.common_builder_impl,
            sinks: self.sinks,
            skip_duration: duration,
        }
    }

    helper::common_impl!(@SinkBuilder: common_builder_impl);
}

impl DedupSinkBuilder<()> {
    #[doc(hidden)]
    #[deprecated(note = "\n\n\
        builder compile-time error:\n\
        - missing required field `skip_duration`\n\n\
    ")]
    pub fn build(self, _: Infallible) {}
}

impl DedupSinkBuilder<Duration> {
    /// Builds a [`DedupSink`].
    pub fn build(self) -> Result<DedupSink> {
        Ok(DedupSink {
            common_impl: helper::CommonImpl::from_builder(self.common_builder_impl),
            sinks: self.sinks,
            skip_duration: self.skip_duration,
            state: Mutex::new(DedupSinkState {
                last_record: None,
                skipped_count: 0,
            }),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::thread::sleep;

    use super::*;
    use crate::{prelude::*, test_utils::*};

    #[test]
    fn dedup() {
        let test_sink = Arc::new(CounterSink::new());
        let dedup_sink = Arc::new(
            DedupSink::builder()
                .skip_duration(Duration::from_secs(1))
                .sink(test_sink.clone())
                .build()
                .unwrap(),
        );
        let test = build_test_logger(|b| b.sink(dedup_sink));

        info!(logger: test, "I wish I was a cat");
        info!(logger: test, "I wish I was a cat");
        info!(logger: test, "I wish I was a cat");

        warn!(logger: test, "I wish I was a cat");
        warn!(logger: test, "I wish I was a cat");
        sleep(Duration::from_millis(1250));
        warn!(logger: test, "I wish I was a cat");

        warn!(logger: test, "No school");
        warn!(logger: test, "No works");
        info!(logger: test, "Just meow meow");

        info!(logger: test, "Meow~ Meow~");
        info!(logger: test, "Meow~ Meow~");
        info!(logger: test, "Meow~ Meow~");
        info!(logger: test, "Meow~ Meow~");
        sleep(Duration::from_millis(1250));
        info!(logger: test, "Meow~ Meow~");
        info!(logger: test, "Meow~ Meow~");
        info!(logger: test, "Meow~ Meow~");
        info!(logger: test, "Meow~ Meow...");

        let records = test_sink.records();

        assert_eq!(records.len(), 13);

        assert_eq!(records[0].payload(), "I wish I was a cat");
        assert_eq!(records[0].level(), Level::Info);

        assert_eq!(records[1].payload(), "(skipped 2 duplicates)");
        assert_eq!(records[1].level(), Level::Info);

        assert_eq!(records[2].payload(), "I wish I was a cat");
        assert_eq!(records[2].level(), Level::Warn);

        assert_eq!(records[3].payload(), "I wish I was a cat");
        assert_eq!(records[3].level(), Level::Warn);

        assert_eq!(records[4].payload(), "I wish I was a cat");
        assert_eq!(records[4].level(), Level::Warn);

        assert_eq!(records[5].payload(), "No school");
        assert_eq!(records[5].level(), Level::Warn);

        assert_eq!(records[6].payload(), "No works");
        assert_eq!(records[6].level(), Level::Warn);

        assert_eq!(records[7].payload(), "Just meow meow");
        assert_eq!(records[7].level(), Level::Info);

        assert_eq!(records[8].payload(), "Meow~ Meow~");
        assert_eq!(records[8].level(), Level::Info);

        assert_eq!(records[9].payload(), "(skipped 3 duplicates)");
        assert_eq!(records[9].level(), Level::Info);

        assert_eq!(records[10].payload(), "Meow~ Meow~");
        assert_eq!(records[10].level(), Level::Info);

        assert_eq!(records[11].payload(), "(skipped 2 duplicates)");
        assert_eq!(records[11].level(), Level::Info);

        assert_eq!(records[12].payload(), "Meow~ Meow...");
        assert_eq!(records[12].level(), Level::Info);
    }
}
