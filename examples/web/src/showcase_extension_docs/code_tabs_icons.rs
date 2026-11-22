use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
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
    let mut tabs = sapid::tabs("file-icons")
        .theme(theme.clone())
        .name("Source files")
        .default_value("main");
    for (name, badge, source, language) in files {
        let palette = theme.clone();
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
