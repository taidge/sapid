use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::json_viewer("types")
        .theme(theme)
        .source(
            r#"{"name":"Sapid","version":9,"stable":true,"deprecated":null,"tags":["rust","ui"]}"#,
        )
        .with_types(true)
}
