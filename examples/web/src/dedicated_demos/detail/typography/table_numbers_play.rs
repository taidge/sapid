use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let tabular = scope.cage(true);
        let palette = theme.clone();
        let preview = each(scope.bond(move || vec![(tabular.get(),)]))
            .key(Clone::clone)
            .children(move |_| {
                let theme = palette.clone();

                sapid::table("numbers-demo")
                    .theme(theme.clone())
                    .tabular_nums(tabular.get())
                    .head(sapid::table_head().row(sapid::table_row("heads").cells([
                        sapid::table_header_cell("name").text("Element"),
                        sapid::table_header_cell("mass").text("Atomic mass"),
                    ])))
                    .body(
                        sapid::table_body().rows(
                            [
                                ("carbon", "Carbon", "12.011"),
                                ("nitrogen", "Nitrogen", "14.007"),
                                ("oxygen", "Oxygen", "15.999"),
                            ]
                            .map(|(key, name, mass)| {
                                sapid::table_row(key).cells([
                                    sapid::table_row_header("name").text(name),
                                    sapid::table_cell("mass").text(mass),
                                ])
                            }),
                        ),
                    )
            });
        view()
            .class(
                "site-configurator site-display-configurator site-table-numbers-play-configurator",
            )
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::switch("Tabular nums", tabular, &theme)],
            ))
            .show_in(scope)
    })
}
