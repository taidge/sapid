use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::json_viewer("json-chevrons").theme(theme).source(r#"{"user":{"name":"John Doe","address":{"city":"Springfield","state":"IL"}},"settings":{"theme":"dark","notifications":true}}"#).tree_navigation(true)
}
