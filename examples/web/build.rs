fn main() {
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
