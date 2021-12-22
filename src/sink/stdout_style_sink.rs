//! Provides a stdout style text sink.

use std::io::{self, Stdout};

use super::std_out_stream_style_sink::{macros::forward_style_sink_methods, StdOutStreamStyleSink};

/// A style sink with `stdout` as the target.
pub struct StdoutStyleSink {
    inner: StdOutStreamStyleSink<Stdout>,
}

forward_style_sink_methods!(StdoutStyleSink, inner);

impl StdoutStyleSink {
    /// Constructs a [`StdoutStyleSink`].
    pub fn new() -> StdoutStyleSink {
        StdoutStyleSink {
            inner: StdOutStreamStyleSink::new(io::stdout(), atty::Stream::Stdout),
        }
    }
}

impl Default for StdoutStyleSink {
    fn default() -> Self {
        StdoutStyleSink::new()
    }
}
