use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::json_viewer("json-line-numbers").theme(theme).source(r#"{"name":"Sapid","version":1,"settings":{"enabled":true,"limits":{"min":2,"max":20}},"members":["Ada","Lin"]}"#).tree_navigation(true).with_line_numbers(true)
}
