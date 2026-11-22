use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::grid("grid-order")
        .theme(theme.clone())
        .columns(4)
        .gap(16.)
        .class("site-grid-order")
        .items([2, 3, 1].map(|i| {
            sapid::grid_item(i.to_string()).fill(controls::grid_demo_cell(i.to_string(), &theme))
        }))
}
