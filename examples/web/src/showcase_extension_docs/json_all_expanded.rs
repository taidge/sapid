use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::json_viewer("expanded-view").theme(theme).source(r#"{"name":"Sapid","settings":{"enabled":true,"theme":{"color":"blue"}},"members":["Ada","Lin"]}"#).default_expand_depth(usize::MAX)
}
