//! Provides a stdout plain text sink.

use std::io::{self, Stdout};

use super::{macros::forward_sink_methods, std_out_stream_sink::StdOutStreamSink};

/// A sink with `stdout` as the target.
pub struct StdoutSink {
    inner: StdOutStreamSink<Stdout>,
}

forward_sink_methods!(StdoutSink, inner);

impl StdoutSink {
    /// Constructs a [`StdoutSink`].
    pub fn new() -> StdoutSink {
        StdoutSink {
            inner: StdOutStreamSink::new(io::stdout()),
        }
    }
}

impl Default for StdoutSink {
    fn default() -> Self {
        StdoutSink::new()
    }
}
