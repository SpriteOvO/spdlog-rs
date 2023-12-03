use std::{marker::PhantomData, result::Result as StdResult};

use erased_serde::Deserializer as ErasedDeserializer;
use serde::{
    de::{
        value::{MapAccessDeserializer, UnitDeserializer},
        Error as SerdeDeError, MapAccess, Visitor,
    },
    Deserialize, Deserializer,
};

use crate::{config, formatter::*, sync::*, Logger, LoggerBuilder, LoggerParams, Result, Sink};

trait Component {
    type Value;

    fn expecting(formatter: &mut std::fmt::Formatter) -> std::fmt::Result;
    fn build(name: &str, de: &mut dyn ErasedDeserializer) -> Result<Self::Value>;
}

struct ComponentFormatter;

impl Component for ComponentFormatter {
    type Value = Box<dyn Formatter>;

    fn expecting(formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a spdlog-rs formatter")
    }

    fn build(name: &str, de: &mut dyn ErasedDeserializer) -> Result<Self::Value> {
        config::registry().build_formatter(&name, de)
    }
}

struct ComponentSink;

impl Component for ComponentSink {
    type Value = Arc<dyn Sink>;

    fn expecting(formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a spdlog-rs sink")
    }

    fn build(name: &str, de: &mut dyn ErasedDeserializer) -> Result<Self::Value> {
        config::registry().build_sink(&name, de)
    }
}

// Unit for 0 parameter components, map for components with parameters
struct UnitOrMapDeserializer<A> {
    map: A,
}

impl<'de, A> Deserializer<'de> for UnitOrMapDeserializer<A>
where
    A: MapAccess<'de>,
{
    type Error = A::Error;

    fn deserialize_any<V>(self, visitor: V) -> StdResult<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(self.map)
    }

    fn deserialize_unit<V>(self, visitor: V) -> StdResult<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> StdResult<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option enum unit_struct seq tuple
        tuple_struct map struct identifier ignored_any
    }
}

struct ComponentVisitor<C>(PhantomData<C>);

impl<'de, C> Visitor<'de> for ComponentVisitor<C>
where
    C: Component,
{
    type Value = C::Value;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        C::expecting(formatter)
    }

    fn visit_map<A>(self, mut map: A) -> StdResult<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let name = map
            .next_entry::<String, String>()?
            .filter(|(key, _)| key == "name")
            .map(|(_, value)| value)
            .ok_or_else(|| SerdeDeError::missing_field("name"))?;

        let mut erased_de = <dyn ErasedDeserializer>::erase(UnitOrMapDeserializer { map });
        let component = C::build(&name, &mut erased_de).map_err(SerdeDeError::custom)?;

        Ok(component)
    }
}

pub fn formatter<'de, D>(de: D) -> StdResult<Option<Box<dyn Formatter>>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Some(de.deserialize_map(ComponentVisitor::<
        ComponentFormatter,
    >(PhantomData))?))
}

pub fn sink<'de, D>(de: D) -> StdResult<Option<Arc<dyn Sink>>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Some(de.deserialize_map(
        ComponentVisitor::<ComponentSink>(PhantomData),
    )?))
}

pub fn logger<'de, D>(de: D) -> StdResult<Logger, D::Error>
where
    D: Deserializer<'de>,
{
    let params = LoggerParams::deserialize(de)?;
    LoggerBuilder::build_config(params).map_err(SerdeDeError::custom)
}
