use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::grid("column-offset")
        .theme(theme.clone())
        .columns(12)
        .gap(16.)
        .items([1, 4, 10].into_iter().enumerate().map(|(i, column)| {
            sapid::grid_item(i.to_string())
                .span(3)
                .column(column)
                .fill(controls::grid_demo_cell((i + 1).to_string(), &theme))
        }))
}
