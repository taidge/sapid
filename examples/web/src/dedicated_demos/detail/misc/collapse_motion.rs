use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let content = theme.clone();
    sapid::collapsible("collapse-motion-example")
        .theme(theme)
        .class("site-reference-collapse site-collapse-center site-collapse-linear")
        .label("Toggle with linear transition")
        .height_transition(500)
        .content(move || display_sample::collapse_paragraph(content.clone()))
}
