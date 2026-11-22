use std::sync::OnceLock;

include!(concat!(env!("OUT_DIR"), "/example_source_index.rs"));

pub fn get(name: &str) -> &'static str {
    static SOURCES: OnceLock<String> = OnceLock::new();
    let all = SOURCES.get_or_init(|| {
        let bytes = miniz_oxide::inflate::decompress_to_vec(include_bytes!(concat!(
            env!("OUT_DIR"),
            "/example_sources.deflate"
        )))
        .expect("valid compressed Rust sources");
        String::from_utf8(bytes).expect("UTF-8 Rust sources")
    });
    let (start, end) = bounds(name);
    &all[start..end]
}
