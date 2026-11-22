use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let body = theme.clone();
    view().class("site-reference-dialog-buttons").fill(
        sapid::drawer("reference-moving-drawer")
            .theme(theme)
            .label("Open Drawer")
            .title("Authentication")
            .class("site-reference-drawer-rotation")
            .content(move || controls::authentication(body.clone(), true)),
    )
}
