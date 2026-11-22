use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    view().class("site-reference-dialog-buttons").fill(
        sapid::dialog("reference-modal-compound")
            .theme(theme)
            .label("Open modal")
            .title("Modal title")
            .content(|| text().content("Modal content")),
    )
}
