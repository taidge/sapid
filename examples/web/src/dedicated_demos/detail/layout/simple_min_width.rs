use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::simple_grid("minimum-columns")
        .theme(theme.clone())
        .columns(3)
        .gap(16.)
        .class("site-simple-minimum")
        .items((1..=5).map(|i| {
            sapid::grid_item(i.to_string()).fill(controls::grid_demo_cell(i.to_string(), &theme))
        }))
}
