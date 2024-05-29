// Test utils for unit tests only
//
// In this file, you can use public or private items from spdlog-rs as you wish,
// as they will be used from unit tests only.

use std::{env, fmt::Write, fs, path::PathBuf};

use crate::{
    config::{ComponentMetadata, Configurable, Registry},
    error::{ConfigError, Error},
    formatter::{FmtExtraInfo, Formatter},
    prelude::*,
    sync::*,
    ErrorHandler, Record, Result, Sink, StringBuf,
};

pub static TEST_LOGS_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let path = env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("dev/test_logs");
    fs::create_dir_all(&path).unwrap();
    path
});

pub mod config {
    use std::sync::Once;

    use super::*;

    pub struct ConfigMockSink<const ID: i32>(i32);

    impl<const ID: i32> Sink for ConfigMockSink<ID> {
        fn log(&self, _record: &Record) -> Result<()> {
            unimplemented!()
        }

        fn flush(&self) -> Result<()> {
            Err(Error::__ForInternalTestsUseOnly(ID, self.0))
        }

        fn level_filter(&self) -> LevelFilter {
            unimplemented!()
        }

        fn set_level_filter(&self, _level_filter: LevelFilter) {
            unimplemented!()
        }

        fn set_formatter(&self, _formatter: Box<dyn Formatter>) {
            unimplemented!()
        }

        fn set_error_handler(&self, _handler: Option<ErrorHandler>) {}
    }

    #[derive(Default, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct MockParams {
        arg: i32,
    }

    macro_rules! impl_multiple_mock_sinks {
        ( $(($id:expr, $name:literal)),+ $(,)? ) => {
            $(impl Configurable for ConfigMockSink<$id> {
                type Params = MockParams;

                fn metadata() -> ComponentMetadata {
                    ComponentMetadata::builder().name($name).build()
                }

                fn build(params: Self::Params) -> Result<Self> {
                    Ok(Self(params.arg))
                }
            })+
        };
    }

    impl_multiple_mock_sinks![
        (1, "ConfigMockSink1"),
        (2, "ConfigMockSink2"),
        (3, "ConfigMockSink3")
    ];

    #[derive(Clone)]
    pub struct ConfigMockFormatter(i32);

    impl Formatter for ConfigMockFormatter {
        fn format(&self, _record: &Record, dest: &mut StringBuf) -> Result<FmtExtraInfo> {
            write!(dest, "{}", self.0).unwrap();
            Ok(FmtExtraInfo::new())
        }

        fn clone_box(&self) -> Box<dyn Formatter> {
            Box::new(self.clone())
        }
    }

    impl Configurable for ConfigMockFormatter {
        type Params = MockParams;

        fn metadata() -> ComponentMetadata {
            ComponentMetadata::builder()
                .name("ConfigMockFormatter")
                .build()
        }

        fn build(params: Self::Params) -> Result<Self> {
            Ok(Self(params.arg))
        }
    }

    fn register_mock_components(registry: &Registry) {
        registry.register_sink::<ConfigMockSink<1>>().unwrap();
        registry.register_sink::<ConfigMockSink<2>>().unwrap();
        assert!(matches!(
            registry.register_sink::<ConfigMockSink<2>>(),
            Err(Error::Config(ConfigError::MultipleRegistration))
        ));
        registry.register_sink::<ConfigMockSink<3>>().unwrap();
        registry
            .register_formatter::<ConfigMockFormatter>()
            .unwrap();
    }

    pub fn registry_for_test() -> Registry {
        let registry = Registry::with_builtin();
        register_mock_components(&registry);
        registry
    }

    pub fn register_global() {
        static INIT: Once = Once::new();

        INIT.call_once(|| register_mock_components(crate::config::registry()));
    }
}
