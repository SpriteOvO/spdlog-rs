//! Provides a stderr plain text sink.

use std::io::{self, Stderr};

use super::{macros::forward_sink_methods, std_out_stream_sink::StdOutStreamSink};

/// A sink with `stderr` as the target.
pub struct StderrSink {
    inner: StdOutStreamSink<Stderr>,
}

forward_sink_methods!(StderrSink, inner);

impl StderrSink {
    /// Constructs a [`StderrSink`].
    pub fn new() -> StderrSink {
        StderrSink {
            inner: StdOutStreamSink::new(io::stderr()),
        }
    }
}

impl Default for StderrSink {
    fn default() -> Self {
        StderrSink::new()
    }
}
