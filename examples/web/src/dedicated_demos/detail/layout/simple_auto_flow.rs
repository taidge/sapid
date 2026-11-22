use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .theme(theme.clone())
        .gap(16.)
        .fill(Vec::from(["auto-fill", "auto-fit"].map(|flow| {
            sapid::stack().theme(theme.clone()).gap(6.).fill((
                sapid::text().theme(theme.clone()).content(flow),
                sapid::simple_grid(format!("flow-{flow}"))
                    .theme(theme.clone())
                    .class(format!("site-simple-{flow}"))
                    .columns(2)
                    .gap(16.)
                    .items((1..=2).map(|i| {
                        sapid::grid_item(i.to_string())
                            .fill(controls::grid_demo_cell(i.to_string(), &theme))
                    })),
            ))
        })))
}
