use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    view().class("site-reference-dialog-buttons").fill(
        sapid::drawer("reference-drawer-compound")
            .theme(theme)
            .label("Open Drawer")
            .title("Drawer title")
            .content(|| text().content("Drawer content")),
    )
}
