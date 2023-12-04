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

// #[derive(Deserialize)]
// #[serde(deny_unknown_fields)]
// pub(super) struct Logger(#[serde(deserialize_with =
// "crate::config::deser::logger")] crate::Logger);

#[derive(PartialEq, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ConfigView {
    loggers: HashMap<String, toml::Value>,
}

#[derive(PartialEq, Debug)]
pub struct Config {
    view: ConfigView, // Stores the config values only, build lazily
}

impl Config {
    // TODO: Remember to remove me
    pub fn new_for_test(inputs: &str) -> Result<Self> {
        let view = toml::from_str(inputs).unwrap();
        Ok(Self { view })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{config::*, TEST_LOGS_PATH};

    #[test]
    fn full() {
        let path = TEST_LOGS_PATH.join("unit_test_config_full.log");
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

        register_global();

        let config = Config::new_for_test(&inputs).unwrap();

        assert_eq!(
            config.view,
            ConfigView {
                loggers: HashMap::from([
                    (
                        "default".to_string(),
                        toml::Value::Table(toml::Table::from_iter([
                            (
                                "sinks".to_string(),
                                toml::Value::Array(vec![
                                    toml::Value::Table(toml::Table::from_iter([
                                        (
                                            "name".to_string(),
                                            toml::Value::String("$ConfigMockSink1".to_string())
                                        ),
                                        ("arg".to_string(), toml::Value::Integer(114))
                                    ])),
                                    toml::Value::Table(toml::Table::from_iter([
                                        (
                                            "name".to_string(),
                                            toml::Value::String("$ConfigMockSink2".to_string())
                                        ),
                                        ("arg".to_string(), toml::Value::Integer(514))
                                    ])),
                                    toml::Value::Table(toml::Table::from_iter([
                                        (
                                            "name".to_string(),
                                            toml::Value::String("$ConfigMockSink3".to_string())
                                        ),
                                        ("arg".to_string(), toml::Value::Integer(1919))
                                    ])),
                                    toml::Value::Table(toml::Table::from_iter([
                                        (
                                            "name".to_string(),
                                            toml::Value::String("FileSink".to_string())
                                        ),
                                        (
                                            "path".to_string(),
                                            toml::Value::String(path.display().to_string())
                                        ),
                                        (
                                            "formatter".to_string(),
                                            toml::Value::Table(toml::Table::from_iter([
                                                (
                                                    "name".to_string(),
                                                    toml::Value::String(
                                                        "PatternFormatter".to_string()
                                                    ),
                                                ),
                                                (
                                                    "template".to_string(),
                                                    toml::Value::String(
                                                        "Meow! {payload}{eol}".to_string()
                                                    ),
                                                )
                                            ]))
                                        )
                                    ]))
                                ])
                            ),
                            (
                                "flush_level_filter".to_string(),
                                toml::Value::String("Equal(Info)".to_string())
                            )
                        ]))
                    ),
                    (
                        "network".to_string(),
                        toml::Value::Table(toml::Table::from_iter([(
                            "sinks".to_string(),
                            toml::Value::Array(vec![toml::Value::Table(toml::Table::from_iter([
                                (
                                    "name".to_string(),
                                    toml::Value::String("$ConfigMockSink2".to_string())
                                ),
                                ("arg".to_string(), toml::Value::Integer(810))
                            ]))])
                        )]))
                    )
                ])
            }
        );

        // TODO
    }
}
