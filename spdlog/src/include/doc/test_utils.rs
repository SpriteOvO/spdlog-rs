# // Workaround for the Rustdoc bug https://github.com/rust-lang/rust/issues/67295
# mod doc_test_utils {
#     use std::sync::Arc;
# 
#     use spdlog::{
#         formatter::{Formatter, Pattern, PatternFormatter},
#         prelude::*,
#         sink::WriteSink,
#     };
# 
#     pub fn echo_logger_from_pattern(
#         pattern: impl Pattern + Clone + 'static,
#         name: Option<&'static str>,
#     ) -> (Logger, Arc<WriteSink<Vec<u8>>>) {
#         echo_logger_from_formatter(Box::new(PatternFormatter::new(pattern)), name)
#     }
# 
#     pub fn echo_logger_from_formatter(
#         formatter: Box<dyn Formatter>,
#         name: Option<&'static str>,
#     ) -> (Logger, Arc<WriteSink<Vec<u8>>>) {
#         let sink = Arc::new(
#             WriteSink::builder()
#                 .formatter(formatter)
#                 .target(Vec::new())
#                 .build()
#                 .unwrap(),
#         );
# 
#         let mut builder = Logger::builder();
# 
#         builder.sink(sink.clone());
#         if let Some(name) = name {
#             builder.name(name);
#         }
# 
#         (builder.build().unwrap(), sink)
#     }
# }
