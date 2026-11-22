use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let body = theme.clone();
    view().class("site-reference-dialog-buttons").fill(
        sapid::popover("reference-profile-overlay")
            .theme(theme)
            .label("Open project profile")
            .trigger_content(|| text().content("S"))
            .class("site-ref-profile-popover")
            .with_arrow(true)
            .modal(true)
            .title("Sapid")
            .content(move || {
                sapid::stack().theme(body.clone()).gap(12.).fill((
                    text().content("@sapid"),
                    text().content("Rust components for accessible interfaces."),
                    text().content("0 Following     1,174 Followers"),
                ))
            }),
    )
}
