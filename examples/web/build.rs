use std::{env, fmt::Write, fs, io, path::PathBuf};

// Standalone documentation examples intentionally repeat imports and complete
// helpers so each listing can be copied. Compress those listings together.
fn pack_example_sources() -> io::Result<()> {
    let destination = PathBuf::from(env::var_os("OUT_DIR").expect("Cargo sets OUT_DIR"));
    let mut contents = Vec::new();
    let mut index = String::from("fn bounds(name: &str) -> (usize, usize) { match name {\n");
    for directory in ["showcase_configurator_docs", "showcase_field_states"] {
        let folder = PathBuf::from("src").join(directory);
        println!("cargo:rerun-if-changed={}", folder.display());
        let mut files = fs::read_dir(&folder)?.collect::<Result<Vec<_>, _>>()?;
        files.sort_by_key(|entry| entry.file_name());
        for file in files {
            let path = file.path();
            if path.extension().is_none_or(|extension| extension != "rs")
                || file.file_name() == "mod.rs"
            {
                continue;
            }
            let start = contents.len();
            contents.extend(fs::read(&path)?);
            let name = path
                .file_stem()
                .and_then(|n| n.to_str())
                .expect("valid Rust module name");
            writeln!(
                index,
                "\"{directory}/{name}\" => ({start}, {}),",
                contents.len()
            )
            .unwrap();
        }
    }
    index.push_str("_ => unreachable!(\"unknown documentation source\"), } }\n");
    fs::write(destination.join("example_source_index.rs"), index)?;
    fs::write(
        destination.join("example_sources.deflate"),
        miniz_oxide::deflate::compress_to_vec(&contents, 9),
    )
}

fn main() {
    pack_example_sources().expect("pack showcase source listings");
    // Unoptimized builds retain the full layout and widget-builder frames while
    // mounting the nested calendar. The default Wasm stack overflows on DateTimePicker.
    if env::var("CARGO_CFG_TARGET_ARCH").as_deref() == Ok("wasm32")
        && env::var("PROFILE").as_deref() == Ok("debug")
        && env::var_os("CARGO_FEATURE_PUBLIC_SHOWCASE").is_some()
    {
        println!("cargo:rustc-link-arg-cdylib=-zstack-size=4194304");
    }
    // Tao runs on the entry thread. The complete debug catalogue needs more
    // stack than the Windows executable's 1 MiB default when mounting a page.
    println!("cargo:rerun-if-changed=build.rs");
    if std::env::var_os("CARGO_FEATURE_WRY_SHOWCASE").is_none()
        || std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows")
    {
        return;
    }
    let flag = if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
        "/STACK:4194304"
    } else {
        "-Wl,--stack,4194304"
    };
    println!("cargo:rustc-link-arg-bin=showcase_wry={flag}");
}
