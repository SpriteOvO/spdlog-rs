//! Provides a stderr style text sink.

use std::io::{self, Stderr};

use super::std_out_stream_style_sink::{macros::forward_style_sink_methods, StdOutStreamStyleSink};

/// A style sink with `stderr` as the target.
pub struct StderrStyleSink {
    inner: StdOutStreamStyleSink<Stderr>,
}

forward_style_sink_methods!(StderrStyleSink, inner);

impl StderrStyleSink {
    /// Constructs a [`StderrStyleSink`].
    pub fn new() -> StderrStyleSink {
        StderrStyleSink {
            inner: StdOutStreamStyleSink::new(io::stderr(), atty::Stream::Stderr),
        }
    }
}

impl Default for StderrStyleSink {
    fn default() -> Self {
        StderrStyleSink::new()
    }
}
