use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::json_viewer("highlight-items").source(r#"{"name":"Sapid","version":"9.7","deprecated":"Use the current API","author":{"name":"Ari","url":"https://example.org"}}"#).default_expand_depth(2).highlight_path("/version",theme.success_soft).highlight_path("/deprecated",theme.danger_soft).highlight_path("/author/url",theme.success_soft).theme(theme)
}
