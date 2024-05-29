use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

use once_cell::sync::Lazy;
use spdlog::{
    config::{self, Config},
    formatter::{pattern, PatternFormatter},
};

include!(concat!(
    env!("OUT_DIR"),
    "/test_utils/common_for_integration_test.rs"
));
use test_utils::*;

static TEMP_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let temp_dir = PathBuf::from(env!("OUT_DIR"))
        .join("dev")
        .join("integration-test-config");
    fs::create_dir_all(&temp_dir).unwrap();
    temp_dir
});

#[test]
fn test_config_full() {
    let path = TEMP_DIR.join("file-sink.log");
    let inputs = format!(
        r#"
[loggers.default]
sinks = [
    {{ name = "$ConfigMockSink1", arg = 114 }},
    {{ name = "$ConfigMockSink2", arg = 514 }},
    {{ name = "$ConfigMockSink3", arg = 1919 }},
    {{ name = "FileSink", path = "{}", formatter = {{ name = "PatternFormatter", template = "Meow! {{payload}}{{eol}}" }} }}
]
flush_level_filter = "Equal(Info)" # TODO: reconsider the syntax

[loggers.network]
sinks = [ {{ name = "$ConfigMockSink2", arg = 810 }} ]
# TODO: flush_period = "10s"
            "#,
        path.display()
    );
    let config = Config::new_for_test(&inputs).unwrap();
}
