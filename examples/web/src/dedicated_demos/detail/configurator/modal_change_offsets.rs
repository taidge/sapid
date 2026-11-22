use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let body = theme.clone();
    sapid::dialog("reference-modal-offset")
        .theme(theme)
        .title("Authentication")
        .class("site-reference-modal-offset")
        .label("Open modal")
        .content(move || controls::authentication(body.clone(), true))
}
