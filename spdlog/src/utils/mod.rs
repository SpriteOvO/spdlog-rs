mod ref_str;

use std::{
    fs::{self, File, OpenOptions},
    io::BufWriter,
    path::Path,
};

pub(crate) use ref_str::*;

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

pub fn open_file_bufw(
    path: impl AsRef<Path>,
    truncate: bool,
    capacity: Option<usize>,
) -> Result<BufWriter<File>> {
    let file = open_file(path, truncate)?;
    Ok(match capacity {
        Some(capacity) => BufWriter::with_capacity(capacity, file),
        None => BufWriter::new(file), // Use std internal default capacity
    })
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

// TODO: Remove this when MSRV reaches 1.82.
pub(crate) fn is_none_or<T>(opt: Option<T>, f: impl FnOnce(T) -> bool) -> bool {
    match opt {
        None => true,
        Some(x) => f(x),
    }
}
