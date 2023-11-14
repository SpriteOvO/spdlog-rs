mod registry;
mod source;

pub(crate) mod parse;

pub use registry::*;
use serde::{de::DeserializeOwned, Deserialize};
pub use source::*;

use crate::{sync::*, Result};

// TODO: Force `'static` on name?
//       Builder?
#[derive(PartialEq, Eq, Hash)]
pub struct ComponentMetadata<'a> {
    pub(crate) name: &'a str,
}

pub trait Configurable: Sized {
    type Params: DeserializeOwned + Default + Send;

    fn metadata() -> ComponentMetadata<'static>;
    fn build(params: Self::Params) -> Result<Self>;
}
