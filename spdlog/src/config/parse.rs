use erased_serde::Deserializer as ErasedDeserializer;
use serde::{
    de::{
        value::{MapAccessDeserializer, UnitDeserializer},
        Error as SerdeDeError, MapAccess, Visitor,
    },
    Deserializer,
};

use crate::{config, formatter::*};

pub fn formatter_deser<'de, D>(de: D) -> Result<Option<Box<dyn Formatter>>, D::Error>
where
    D: Deserializer<'de>,
{
    struct ParseVisitor;

    impl<'de> Visitor<'de> for ParseVisitor {
        type Value = Box<dyn Formatter>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a spdlog-rs component")
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let name = map
                .next_entry::<String, String>()?
                .filter(|(key, _)| key == "name")
                .map(|(_, value)| value)
                .ok_or_else(|| SerdeDeError::missing_field("name"))?;

            let remaining_args = map.size_hint().unwrap(); // I don't know what situation it will be `None``

            let formatter = if remaining_args == 0 {
                let mut erased_de =
                    <dyn ErasedDeserializer>::erase(UnitDeserializer::<A::Error>::new());
                config::registry().build_formatter(&name, &mut erased_de)
            } else {
                let mut erased_de =
                    <dyn ErasedDeserializer>::erase(MapAccessDeserializer::new(map));
                config::registry().build_formatter(&name, &mut erased_de)
            }
            .map_err(|err| SerdeDeError::custom(err))?;

            Ok(formatter)
        }
    }

    Ok(Some(de.deserialize_map(ParseVisitor)?))
}
