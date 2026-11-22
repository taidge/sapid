use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::json_viewer("named-view").theme(theme).source(r#"{"name":"Sapid","settings":{"enabled":true,"theme":{"color":"blue"}},"members":["Ada","Lin"]}"#).root_name("response")
}
