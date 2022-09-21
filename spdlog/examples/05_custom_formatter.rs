use std::fmt::Write;

use spdlog::{
    formatter::{FmtExtraInfo, Formatter},
    prelude::*,
    Record, StringBuf,
};

#[derive(Clone, Default)]
struct CustomFormatter;

impl CustomFormatter {
    fn new() -> Self {
        Self
    }
}

impl Formatter for CustomFormatter {
    fn format(&self, record: &Record, dest: &mut StringBuf) -> spdlog::Result<FmtExtraInfo> {
        let style_range_begin: usize = dest.len();

        dest.write_str(&record.level().as_str().to_ascii_uppercase())
            .map_err(spdlog::Error::FormatRecord)?;

        let style_range_end: usize = dest.len();

        writeln!(dest, " {}", record.payload()).map_err(spdlog::Error::FormatRecord)?;

        Ok(FmtExtraInfo::builder()
            .style_range(style_range_begin..style_range_end)
            .build())
    }

    fn clone_box(&self) -> Box<dyn Formatter> {
        Box::new(self.clone())
    }
}

fn main() {
    info!("hello, world");

    // Building a custom formatter.
    let new_formatter: Box<CustomFormatter> = Box::new(CustomFormatter::new());

    // Setting the new formatter for each sink of the default logger.
    for sink in spdlog::default_logger().sinks() {
        sink.set_formatter(new_formatter.clone())
    }

    info!("hello, world");
}
