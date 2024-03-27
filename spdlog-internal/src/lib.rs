pub mod pattern_parser;

#[macro_export]
macro_rules! impossible {
    ( $dbg_lit:literal, $($fmt_arg:expr),* ) => {
        panic!(
            "this should not happen, please open an issue on 'spdlog-rs' Bug Tracker\n\nsource: {}\ndebug:{}",
            format!("{}:{}", file!(), line!()),
            format!($dbg_lit, $($fmt_arg),*),
        )
    };
}
