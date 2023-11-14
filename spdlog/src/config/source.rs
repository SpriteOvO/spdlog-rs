use std::{marker::PhantomData, path::Path};

pub trait Source {
    fn file<P>(path: P)
    where
        P: AsRef<Path>,
    {
    }
}

// TODO: place it into a format mod?
pub struct Toml {
    phantom: PhantomData<()>,
}

impl Source for Toml {
    //
}
