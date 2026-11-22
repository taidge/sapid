use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let copied = scope.cage(CopyState::Idle);
        sapid::stack().theme(theme.clone()).fill((
sapid::json_viewer("copy-json").theme(theme.clone()).tree_navigation(true).source(r#"{"project":{"name":"Sapid","active":true},"members":["Ari","Lin"],"release":1}"#)
.copy_button(sapid::copy_button().theme(theme.clone()).label("Copy complete JSON").state_ref(copied)),
sapid::text().theme(theme.clone()).content(copied.map(|s|format!("Copy: {s:?}")))
)).show_in(scope)
    })
}
