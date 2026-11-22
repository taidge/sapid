use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    fn file_icon(name: &str) -> &'static str {
        match name.rsplit('.').next() {
            Some("rs") => "Rs",
            Some("json") => "{}",
            _ => "•",
        }
    }
    let files = [
        (
            "main.rs",
            "Rs",
            "fn main() {\n    println!(\"Hello\");\n}",
            sapid::CodeLanguage::Rust,
        ),
        (
            "config.json",
            "{}",
            r#"{"theme":"blue","enabled":true}"#,
            sapid::CodeLanguage::Json,
        ),
    ];
    let mut tabs = sapid::tabs("file-types")
        .theme(theme.clone())
        .name("Source files")
        .default_value("main");
    for (name, _badge, source, language) in files {
        let palette = theme.clone();
        let badge = file_icon(name);
        tabs = tabs.tab(
            sapid::tab(if name == "main.rs" { "main" } else { "config" })
                .label(name)
                .trigger_content(move || {
                    glory::ui::group().fill((
                        text().class("showcase-code-file-icon").content(badge),
                        text().content(name),
                    ))
                })
                .content(move || {
                    sapid::code_highlight()
                        .theme(palette.clone())
                        .source(source)
                        .language(language)
                }),
        );
    }
    tabs
}
