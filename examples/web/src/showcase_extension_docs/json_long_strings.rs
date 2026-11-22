use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::json_viewer("long-strings").theme(theme).source(r#"{"title":"Short title","description":"This is a very long description that will be collapsed after thirty characters.","content":"A second long value can be expanded independently of the description.","short":"ok"}"#).collapse_strings_after(30)
}
