use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    view().class("site-reference-dialog-buttons").fill(
        sapid::pagination("compound-pages")
            .theme(theme)
            .total(Some(10))
            .with_edges(true)
            .with_controls(true),
    )
}
