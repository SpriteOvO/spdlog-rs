use std::collections::HashMap;

use erased_serde::Deserializer as ErasedDeserializer;

use super::ComponentMetadata;
use crate::{
    config::Configurable,
    error::ConfigError,
    formatter::{Formatter, FullFormatter, PatternFormatter, RuntimePattern},
    sink::*,
    sync::*,
    Error, Logger, Result, Sink,
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

type ComponentDeser<C> = fn(de: &mut dyn ErasedDeserializer) -> Result<C>;
type RegisteredComponents<C> = HashMap<&'static str, ComponentDeser<C>>;

pub struct Registry {
    // TODO: Consider make them compile-time
    builtin_sink: Mutex<RegisteredComponents<Arc<dyn Sink>>>,
    builtin_formatter: Mutex<RegisteredComponents<Box<dyn Formatter>>>,

    custom_sink: Mutex<RegisteredComponents<Arc<dyn Sink>>>,
    custom_formatter: Mutex<RegisteredComponents<Box<dyn Formatter>>>,
}

impl Registry {
    pub fn register_sink<S>(&self) -> Result<()>
    where
        S: Sink + Configurable + 'static,
    {
        self.register_sink_inner::<S>()
    }

    pub fn register_formatter<F>(&self) -> Result<()>
    where
        F: Formatter + Configurable + 'static,
    {
        self.register_formatter_inner::<F>()
    }
}

macro_rules! deser_closure {
    ( $wrap:ident<dyn $trait:ident> ) => {
        deser_closure!(@INNER, $wrap<dyn $trait>, $wrap::new)
    };
    ( @INNER, $ret_ty:ty, $ret_expr:expr ) => {
        |de: &mut dyn ErasedDeserializer| -> Result<$ret_ty> {
            let mut params = C::Params::default();
            params
                .erased_deserialize_in_place(de)
                .map_err(|err| Error::Config(ConfigError::BuildComponent(err.to_string())))?;
            Ok($ret_expr(C::build(params)?))
        }
    };
}

impl Registry {
    pub(crate) fn with_builtin() -> Self {
        let mut registry = Self {
            builtin_sink: Mutex::new(HashMap::new()),
            builtin_formatter: Mutex::new(HashMap::new()),
            custom_sink: Mutex::new(HashMap::new()),
            custom_formatter: Mutex::new(HashMap::new()),
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
    ) -> Result<Arc<dyn Sink>> {
        let (registered, name) = if !name.starts_with('$') {
            (&self.builtin_sink, name)
        } else {
            (&self.custom_sink, name.strip_prefix('$').unwrap())
        };
        registered
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
        let (registered, name) = if !name.starts_with('$') {
            (&self.builtin_formatter, name)
        } else {
            (&self.custom_formatter, name.strip_prefix('$').unwrap())
        };
        registered
            .lock_expect()
            .get(name)
            .ok_or_else(|| Error::Config(ConfigError::UnknownComponent(name.to_string())))
            .and_then(|f| f(de))
    }

    fn register_sink_inner<C>(&self) -> Result<()>
    where
        C: Sink + Configurable + 'static,
    {
        self.custom_sink
            .lock_expect()
            .insert(C::metadata().name, deser_closure!(Arc<dyn Sink>))
            .map_or(Ok(()), |_| {
                Err(Error::Config(ConfigError::MultipleRegistration))
            })
    }

    fn register_formatter_inner<C>(&self) -> Result<()>
    where
        C: Formatter + Configurable + 'static,
    {
        self.custom_formatter
            .lock_expect()
            .insert(C::metadata().name, deser_closure!(Box<dyn Formatter>))
            .map_or(Ok(()), |_| {
                Err(Error::Config(ConfigError::MultipleRegistration))
            })
    }

    fn register_builtin_sink<C>(&self) -> Result<()>
    where
        C: Sink + Configurable + 'static,
    {
        self.builtin_sink
            .lock_expect()
            .insert(C::metadata().name, deser_closure!(Arc<dyn Sink>))
            .map_or(Ok(()), |_| {
                Err(Error::Config(ConfigError::MultipleRegistration))
            })
    }

    fn register_builtin_formatter<C>(&self) -> Result<()>
    where
        C: Formatter + Configurable + 'static,
    {
        self.builtin_formatter
            .lock_expect()
            .insert(C::metadata().name, deser_closure!(Box<dyn Formatter>))
            .map_or(Ok(()), |_| {
                Err(Error::Config(ConfigError::MultipleRegistration))
            })
    }
}

// TODO: Consider removing the `'static` lifetime. Maybe using `Arc<>`?
pub(crate) fn registry() -> &'static Registry {
    static REGISTRY: Lazy<Registry> = Lazy::new(Registry::with_builtin);
    &REGISTRY
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{prelude::*, test_utils::config::*, Record, StringBuf};

    #[test]
    fn build_sink_from_params() {
        let registry = registry_for_test();

        let mut erased_de =
            <dyn ErasedDeserializer>::erase(toml::Deserializer::new("arg = 114514"));
        let sink = registry
            .build_sink("$ConfigMockSink2", &mut erased_de)
            .unwrap();
        assert!(matches!(
            sink.flush(),
            Err(Error::__ForInternalTestsUseOnly(2, 114514))
        ));

        let mut erased_de =
            <dyn ErasedDeserializer>::erase(toml::Deserializer::new("unmatched_arg = 114514"));
        assert!(matches!(
            registry.build_sink("$ConfigMockSink2", &mut erased_de),
            Err(Error::Config(ConfigError::BuildComponent(_)))
        ));

        let mut erased_de =
            <dyn ErasedDeserializer>::erase(toml::Deserializer::new("arg = 114514"));
        assert!(matches!(
            registry.build_sink("$ConfigMockSinkUnregistered", &mut erased_de),
            Err(Error::Config(ConfigError::UnknownComponent(_)))
        ));
    }

    #[test]
    fn build_formatter_from_params() {
        let registry = registry_for_test();

        let mut erased_de =
            <dyn ErasedDeserializer>::erase(toml::Deserializer::new("arg = 1919810"));
        let formatter = registry
            .build_formatter("$ConfigMockFormatter", &mut erased_de)
            .unwrap();
        let mut dest = StringBuf::new();
        formatter
            .format(&Record::new(Level::Info, ""), &mut dest)
            .unwrap();
        assert_eq!(dest, "1919810")
    }

    // TODO: Test custom components
}
