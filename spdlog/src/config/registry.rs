use std::collections::HashMap;

use erased_serde::Deserializer as ErasedDeserializer;

use super::ComponentMetadata;
use crate::{
    config::Configurable,
    error::ConfigError,
    formatter::{Formatter, FullFormatter, PatternFormatter, RuntimePattern},
    sink::*,
    sync::*,
    Error, Result, Sink,
};

type StdResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// https://github.com/dtolnay/erased-serde/issues/97
mod erased_serde_ext {
    use erased_serde::Result;
    use serde::de::Deserialize;

    use super::*;

    pub trait ErasedDeserialize<'a> {
        fn erased_deserialize_in_place(
            &mut self,
            de: &mut dyn ErasedDeserializer<'a>,
        ) -> Result<()>;
    }

    pub trait ErasedDeserializeOwned: for<'a> ErasedDeserialize<'a> {}

    impl<T: for<'a> ErasedDeserialize<'a>> ErasedDeserializeOwned for T {}

    impl<'a, T: Deserialize<'a>> ErasedDeserialize<'a> for T {
        fn erased_deserialize_in_place(
            &mut self,
            de: &mut dyn ErasedDeserializer<'a>,
        ) -> Result<()> {
            Deserialize::deserialize_in_place(de, self)
        }
    }
}
use erased_serde_ext::*;

type ComponentDeser<C: Configurable> = fn(de: &mut dyn ErasedDeserializer) -> Result<C>;

type RegisteredComponents<C: Configurable> = HashMap<&'static str, ComponentDeser<C>>;

pub struct Registry {
    sink: Mutex<RegisteredComponents<Box<dyn Sink>>>,
    formatter: Mutex<RegisteredComponents<Box<dyn Formatter>>>,

    // TODO: Consider make them compile-time
    builtin_sink: Mutex<RegisteredComponents<Box<dyn Sink>>>,
    builtin_formatter: Mutex<RegisteredComponents<Box<dyn Formatter>>>,
}

impl Registry {
    pub fn register_sink<S>(&mut self) -> Result<()>
    where
        S: Sink + Configurable + 'static,
    {
        self.register_sink_inner::<S>()
    }

    pub fn register_formatter<F>(&mut self) -> Result<()>
    where
        F: Formatter + Configurable + 'static,
    {
        self.register_formatter_inner::<F>()
    }
}

impl Registry {
    pub(crate) fn with_builtin() -> Self {
        let mut registry = Self {
            sink: Mutex::new(HashMap::new()),
            formatter: Mutex::new(HashMap::new()),
            builtin_sink: Mutex::new(HashMap::new()),
            builtin_formatter: Mutex::new(HashMap::new()),
        };
        registry.register_builtin().unwrap(); // Builtin components should not fail to register
        registry
    }

    fn register_builtin(&mut self) -> Result<()> {
        self.register_builtin_sink::<FileSink>()?;
        self.register_builtin_formatter::<FullFormatter>()?;
        self.register_builtin_formatter::<PatternFormatter<RuntimePattern>>()?;
        Ok(())
    }

    pub(crate) fn build_sink(
        &self,
        name: &str,
        de: &mut dyn ErasedDeserializer,
    ) -> Result<Box<dyn Sink>> {
        self.builtin_sink
            .lock_expect()
            .get(name)
            .ok_or_else(|| Error::Config(ConfigError::UnknownComponent(name.to_string())))
            .and_then(|f| f(de))
    }

    pub(crate) fn build_formatter(
        &self,
        name: &str,
        de: &mut dyn ErasedDeserializer,
    ) -> Result<Box<dyn Formatter>> {
        self.builtin_formatter
            .lock_expect()
            .get(name)
            .ok_or_else(|| Error::Config(ConfigError::UnknownComponent(name.to_string())))
            .and_then(|f| f(de))
    }

    impl_registers! {
        fn register_sink_inner => sink, Sink,
        fn register_formatter_inner => formatter, Formatter,
        pub(crate) fn register_builtin_sink => builtin_sink, Sink,
        pub(crate) fn register_builtin_formatter => builtin_formatter, Formatter,
    }
}

// TODO: Append prefix '$' for custom components
macro_rules! impl_registers {
    ( $($vis:vis fn $fn_name:ident => $var:ident, $trait:ident),+ $(,)? ) => {
        $($vis fn $fn_name<C>(&mut self) -> Result<()>
        where
            C: $trait + Configurable + 'static,
        {
            let f = |de: &mut dyn ErasedDeserializer| -> Result<Box<dyn $trait>> {
                let mut params = C::Params::default();
                params.erased_deserialize_in_place(de).unwrap(); // TODO: Wrong input will trigger a Err, handle it!
                Ok(Box::new(C::build(params)?))
            };

            self.$var
                .lock_expect()
                .insert(C::metadata().name, f)
                .map_or(Ok(()), |_| Err(Error::Config(ConfigError::MultipleRegistration)))
        })+
    };
}
use impl_registers;

// TODO: Consider removing the `'static` lifetime. Maybe using `Arc<>`?
pub(crate) fn registry() -> &'static Registry {
    static REGISTRY: Lazy<Registry> = Lazy::new(Registry::with_builtin);
    &REGISTRY
}

#[cfg(test)]
mod tests {
    use std::fmt::Write;

    use serde::Deserializer;

    use super::*;
    use crate::{formatter::FmtExtraInfo, prelude::*, ErrorHandler, Record, StringBuf};

    pub struct MockSink(i32);

    impl Sink for MockSink {
        fn log(&self, _record: &Record) -> Result<()> {
            unimplemented!()
        }

        fn flush(&self) -> Result<()> {
            unimplemented!()
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

        fn set_error_handler(&self, handler: Option<ErrorHandler>) {
            handler.unwrap()(Error::__ForInternalTestsUseOnly(self.0))
        }
    }

    #[derive(Default, serde::Deserialize)]
    pub struct MockParams {
        arg: i32,
    }

    impl Configurable for MockSink {
        type Params = MockParams;

        fn metadata() -> ComponentMetadata<'static> {
            ComponentMetadata { name: "MockSink" }
        }

        fn build(params: Self::Params) -> Result<Self> {
            Ok(Self(params.arg))
        }
    }

    #[derive(Clone)]
    pub struct MockFormatter(i32);

    impl Formatter for MockFormatter {
        fn format(&self, _record: &Record, dest: &mut StringBuf) -> Result<FmtExtraInfo> {
            write!(dest, "{}", self.0).unwrap();
            Ok(FmtExtraInfo::new())
        }

        fn clone_box(&self) -> Box<dyn Formatter> {
            Box::new(self.clone())
        }
    }

    impl Configurable for MockFormatter {
        type Params = MockParams;

        fn metadata() -> ComponentMetadata<'static> {
            ComponentMetadata {
                name: "MockFormatter",
            }
        }

        fn build(params: Self::Params) -> Result<Self> {
            Ok(Self(params.arg))
        }
    }

    fn registry_for_test() -> Registry {
        let mut registry = Registry::with_builtin();
        registry.register_sink::<MockSink>().unwrap();
        registry.register_formatter::<MockFormatter>().unwrap();
        registry
    }

    #[test]
    fn build_sink_from_params() {
        let registry = registry_for_test();

        let mut erased_de =
            <dyn ErasedDeserializer>::erase(toml::Deserializer::new("arg = 114514"));
        let sink = registry.build_sink("MockSink", &mut erased_de).unwrap();
        sink.set_error_handler(Some(|err| {
            assert!(matches!(err, Error::__ForInternalTestsUseOnly(114514)))
        }));

        // TODO: test wrong kind
    }

    #[test]
    fn build_formatter_from_params() {
        let registry = registry_for_test();

        let mut erased_de =
            <dyn ErasedDeserializer>::erase(toml::Deserializer::new("arg = 1919810"));
        let formatter = registry
            .build_formatter("MockFormatter", &mut erased_de)
            .unwrap();
        let mut dest = StringBuf::new();
        formatter
            .format(&Record::new(Level::Info, ""), &mut dest)
            .unwrap();
        assert_eq!(dest, "1919810")

        // TODO: test wrong kind
    }

    // TODO: Test custom components
}
