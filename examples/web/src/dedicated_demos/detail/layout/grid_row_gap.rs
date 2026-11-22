use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::grid("row-column-gap")
        .theme(theme.clone())
        .columns(4)
        .class("site-grid-distinct-gaps")
        .items((1..=8).map(|n| {
            sapid::grid_item(n.to_string()).fill(controls::grid_demo_cell(n.to_string(), &theme))
        }))
}
