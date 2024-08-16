fn main() {
    spdlog::info!("the default formatter for most sinks is `FullFormatter`");

    // There are two ways to set up custom formats

    // 1. This is the easiest and most convenient way
    use_pattern_formatter();

    // 2. When you need to implement more complex formatting logic
    impl_manually();
}

fn use_pattern_formatter() {
    use spdlog::{
        formatter::{pattern, PatternFormatter},
        prelude::*,
    };

    // Building a pattern formatter with a pattern.
    // The `pattern!` macro will parse the template string at compile-time.
    let new_formatter = Box::new(PatternFormatter::new(pattern!(
        "{datetime} - {^{level}} - {payload}{eol}"
    )));

    // Setting the new formatter for each sink of the default logger.
    for sink in spdlog::default_logger().sinks() {
        sink.set_formatter(new_formatter.clone())
    }

    info!("format by `PatternFormatter`");
}

fn impl_manually() {
    use std::fmt::Write;

    use spdlog::{
        formatter::{Formatter, FormatterContext},
        prelude::*,
        Record, StringBuf,
    };

    #[derive(Clone, Default)]
    struct MyFormatter;

    impl Formatter for MyFormatter {
        fn format(
            &self,
            record: &Record,
            dest: &mut StringBuf,
            ctx: &mut FormatterContext,
        ) -> spdlog::Result<()> {
            let style_range_begin = dest.len();

            dest.write_str(&record.level().as_str().to_ascii_uppercase())
                .map_err(spdlog::Error::FormatRecord)?;

            let style_range_end = dest.len();

            writeln!(dest, " {}", record.payload()).map_err(spdlog::Error::FormatRecord)?;

            ctx.set_style_range(Some(style_range_begin..style_range_end));
            Ok(())
        }
    }

    // Building a custom formatter.
    let new_formatter = Box::new(MyFormatter);

    // Setting the new formatter for each sink of the default logger.
    for sink in spdlog::default_logger().sinks() {
        sink.set_formatter(new_formatter.clone())
    }

    info!("format by `MyFormatter` (impl manually)");
}
