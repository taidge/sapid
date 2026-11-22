use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(false);
        let content = theme.clone();
        sapid::collapsible("collapse-hook")
            .theme(theme.clone())
            .class("site-reference-collapse site-collapse-hook")
            .label(open.map(|v| {
                if *v {
                    String::from("Collapse")
                } else {
                    String::from("Expand")
                }
            }))
            .open(open)
            .on_open_change(move |proposed| open.revise(|current| *current = proposed))
            .height_transition(200)
            .content(move || display_sample::collapse_paragraph(content.clone()))
            .show_in(scope)
    })
}
