use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::grid("responsive-gap")
        .theme(theme.clone())
        .columns(3)
        .class("site-grid-responsive-gap")
        .items((1..=3).map(|n| {
            sapid::grid_item(n.to_string()).fill(controls::grid_demo_cell(n.to_string(), &theme))
        }))
}
