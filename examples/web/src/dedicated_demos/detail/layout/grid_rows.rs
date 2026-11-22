use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::grid("multiple-rows")
        .theme(theme.clone())
        .columns(12)
        .gap(16.)
        .items((1..=4).map(|i| {
            sapid::grid_item(i.to_string())
                .span(4)
                .fill(controls::grid_demo_cell(i.to_string(), &theme))
        }))
}
