use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let content = theme.clone();
    sapid::collapsible("collapse-usage")
        .theme(theme)
        .class("site-reference-collapse site-collapse-center")
        .label("Toggle content")
        .height_transition(200)
        .content(move || display_sample::collapse_paragraph(content.clone()))
}
