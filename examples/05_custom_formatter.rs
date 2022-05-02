use std::{fmt::Write, sync::Arc};

use spdlog::{
    formatter::{FmtExtraInfo, Formatter},
    prelude::*,
    sink::Sink,
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

        write!(dest, " {}\n", record.payload()).map_err(spdlog::Error::FormatRecord)?;

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

    let default_logger: Arc<Logger> = spdlog::default_logger();

    // Building a custom formatter.
    let new_formatter: Box<CustomFormatter> = Box::new(CustomFormatter::new());

    // Setting new formatter for each sink of the default logger and saving old
    // formatters.
    let old_formatters: Vec<Box<dyn Formatter>> = default_logger
        .sinks()
        .iter()
        .map(|sink: &Arc<dyn Sink>| sink.swap_formatter(new_formatter.clone()))
        .collect::<Vec<Box<dyn Formatter>>>();

    info!("hello, world");

    // Setting back old formatters.
    default_logger
        .sinks()
        .iter()
        .zip(old_formatters.into_iter())
        .for_each(|(sink, formatter): (&Arc<dyn Sink>, Box<dyn Formatter>)| {
            sink.set_formatter(formatter)
        });

    info!("hello, world");
}
