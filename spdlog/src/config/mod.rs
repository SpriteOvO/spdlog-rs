mod registry;
mod source;

pub(crate) mod deser;

use std::{collections::HashMap, convert::Infallible};

pub use registry::*;
use serde::{de::DeserializeOwned, Deserialize};
pub use source::*;

use crate::{sync::*, Result};

// TODO: Builder?
#[derive(PartialEq, Eq, Hash)]
pub struct ComponentMetadata {
    name: &'static str,
}

impl ComponentMetadata {
    pub fn builder() -> ComponentMetadataBuilder<()> {
        ComponentMetadataBuilder { name: () }
    }
}

pub struct ComponentMetadataBuilder<ArgName> {
    name: ArgName,
}

impl<ArgName> ComponentMetadataBuilder<ArgName> {
    pub fn name(self, name: &'static str) -> ComponentMetadataBuilder<&'static str> {
        ComponentMetadataBuilder { name }
    }
}

impl ComponentMetadataBuilder<()> {
    #[doc(hidden)]
    #[deprecated(note = "\n\n\
        builder compile-time error:\n\
        - missing required field `name`\n\n\
    ")]
    pub fn build(self, _: Infallible) {}
}

impl ComponentMetadataBuilder<&'static str> {
    pub fn build(self) -> ComponentMetadata {
        ComponentMetadata { name: self.name }
    }
}

pub trait Configurable: Sized {
    type Params: DeserializeOwned + Default + Send;

    fn metadata() -> ComponentMetadata;
    fn build(params: Self::Params) -> Result<Self>;
}

mod storage {
    use serde::Deserialize;

    use super::*;

    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    pub(super) struct Logger(
        #[serde(deserialize_with = "crate::config::deser::logger")] crate::Logger,
    );

    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    pub(super) struct Config {
        loggers: HashMap<String, Logger>,
    }
}

pub struct Config(storage::Config);

impl Config {
    // TODO: Remember to remove me
    pub fn new_for_test(inputs: &str) -> Result<Self> {
        let config = toml::from_str(inputs).unwrap();
        Ok(Self(config))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{config::*, TEST_LOGS_PATH};

    #[test]
    fn full() {
        let inputs = format!(
            r#"
[loggers.default]
sinks = [
    {{ name = "$ConfigMockSink1", arg = 114 }},
    {{ name = "$ConfigMockSink2", arg = 514 }},
    {{ name = "$ConfigMockSink3", arg = 1919 }},
    {{ name = "FileSink", path = "{}", formatter = {{ name = "PatternFormatter", template = "114514 {{payload}}{{eol}}" }} }}
]
# flush_level_filter = "all" # TODO: design the syntax
# TODO: flush_period = "10s"
            "#,
            TEST_LOGS_PATH.join("unit_test_config_full.log").display()
        );

        register_global();

        Config::new_for_test(&inputs).unwrap();

        // TODO
    }
}
