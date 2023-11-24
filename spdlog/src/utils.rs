use std::{
    fs::{self, File, OpenOptions},
    path::Path,
};

use crate::{Error, Result};

pub fn open_file(path: impl AsRef<Path>, truncate: bool) -> Result<File> {
    if let Some(parent) = path.as_ref().parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(Error::CreateDirectory)?;
        }
    }

    let mut open_options = OpenOptions::new();

    if truncate {
        open_options.write(true).truncate(true);
    } else {
        open_options.append(true);
    }

    open_options
        .create(true)
        .open(path)
        .map_err(Error::OpenFile)
}

// Credits `static_assertions` crate
macro_rules! const_assert {
    ( $cond:expr $(,)? ) => {
        const _: [(); 0 - !{
            const EVALUATED: bool = $cond;
            EVALUATED
        } as usize] = [];
    };
}
pub(crate) use const_assert;
