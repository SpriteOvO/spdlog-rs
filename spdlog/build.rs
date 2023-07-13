use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use rustc_version::{version_meta, Channel};

fn main() -> Result<(), Box<dyn Error>> {
    set_cfg_channel()?;
    generate_code_test_utils()?;
    Ok(())
}

// Set cfg flags depending on release channel
fn set_cfg_channel() -> Result<(), Box<dyn Error>> {
    let channel = match version_meta()?.channel {
        Channel::Stable => "CHANNEL_STABLE",
        Channel::Beta => "CHANNEL_BETA",
        Channel::Nightly => "CHANNEL_NIGHTLY",
        Channel::Dev => "CHANNEL_DEV",
    };
    println!("cargo:rustc-cfg={}", channel);
    Ok(())
}

// Generate test utils for unit tests, integration tests and doc tests
//
// Workaround for the rustdoc bug https://github.com/rust-lang/rust/issues/67295
fn generate_code_test_utils() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?).join("test_utils");
    if !out_dir.is_dir() {
        fs::create_dir(&out_dir)?;
    }

    let input = fs::read_to_string("src/test_utils/common.rs")?;

    write_generated_code(
        out_dir.join("common_for_doc_test.rs"),
        format!("mod test_utils {{\n{}\n}}", input)
            .lines()
            .map(|line| format!("# {}\n", line))
            .collect::<String>(),
    )?;
    write_generated_code(
        out_dir.join("common_for_unit_test.rs"),
        input.replace("spdlog::", "crate::"),
    )?;

    Ok(())
}

fn write_generated_code(
    path: impl AsRef<Path>,
    contents: impl AsRef<[u8]>,
) -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed={}", path.as_ref().display());
    fs::write(path, contents)?;
    Ok(())
}
