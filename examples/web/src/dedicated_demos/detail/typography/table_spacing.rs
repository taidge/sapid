use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let horizontal = scope.cage(10_f64);
        let vertical = scope.cage(10_f64);
        let palette = theme.clone();
        let preview =
            each(scope.bond(move || vec![(horizontal.get().to_bits(), vertical.get().to_bits())]))
                .key(Clone::clone)
                .children(move |_| {
                    sapid::table("elements-spacing")
                        .theme(palette.clone())
                        .with_row_borders(true)
                        .horizontal_spacing(horizontal.get())
                        .vertical_spacing(vertical.get())
                        .head(sapid::table_head().row(sapid::table_row("head").cells([
                            sapid::table_header_cell("position").text("Position"),
                            sapid::table_header_cell("name").text("Name"),
                            sapid::table_header_cell("symbol").text("Symbol"),
                        ])))
                        .body(
                            sapid::table_body().rows(
                                [
                                    ("6", "Carbon", "C"),
                                    ("7", "Nitrogen", "N"),
                                    ("39", "Yttrium", "Y"),
                                    ("56", "Barium", "Ba"),
                                    ("58", "Cerium", "Ce"),
                                ]
                                .map(|(position, name, symbol)| {
                                    sapid::table_row(symbol).cells([
                                        sapid::table_cell("position").text(position),
                                        sapid::table_cell("name").text(name),
                                        sapid::table_cell("symbol").text(symbol),
                                    ])
                                }),
                            ),
                        )
                });
        let stops = [10., 12., 16., 20., 32.];
        view()
            .class("site-configurator site-display-configurator site-table-spacing-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::pixel_scale(
                        "table_spacing-horizontal",
                        "Horizontal spacing",
                        horizontal,
                        stops,
                        &theme,
                    ),
                    controls::pixel_scale(
                        "table_spacing-vertical",
                        "Vertical spacing",
                        vertical,
                        stops,
                        &theme,
                    ),
                ],
            ))
            .show_in(scope)
    })
}
