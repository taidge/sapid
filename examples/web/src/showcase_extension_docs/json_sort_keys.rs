use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::json_viewer("json-sort-keys").theme(theme).source(r#"{"zebra":"last","apple":"first","mango":"middle","banana":"second","nested":{"zoo":3,"ark":1,"bee":2}}"#).sort_keys(true)
}
