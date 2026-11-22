use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::grid("more-columns")
        .theme(theme.clone())
        .columns(24)
        .gap(16.)
        .items([12, 6, 6].into_iter().enumerate().map(|(i, span)| {
            sapid::grid_item(i.to_string())
                .span(span)
                .fill(controls::grid_demo_cell((i + 1).to_string(), &theme))
        }))
}
