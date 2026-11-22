use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let rows = [
            ("6", "Carbon", "C", "12.011"),
            ("7", "Nitrogen", "N", "14.007"),
            ("39", "Yttrium", "Y", "88.906"),
            ("56", "Barium", "Ba", "137.33"),
            ("58", "Cerium", "Ce", "140.12"),
        ];
        let mut head =
            sapid::table_row("headers").cell(sapid::table_header_cell("select").text(""));
        for (i, label) in ["Element position", "Element name", "Symbol", "Atomic mass"]
            .into_iter()
            .enumerate()
        {
            head = head.cell(sapid::table_header_cell(format!("head-{i}")).text(label));
        }
        let body =
            sapid::table_body().rows(rows.into_iter().map(|(number, name, symbol, mass)| {
                let selected = scope.cage(false);
                let palette = theme.clone();
                let mut row = sapid::table_row(format!("element-{number}"))
                    .class(selected.map(|on| {
                        if *on {
                            "sapid-demo-selected-element".to_owned()
                        } else {
                            String::new()
                        }
                    }))
                    .cell(sapid::table_cell("select").content(move || {
                        sapid::checkbox()
                            .theme(palette.clone())
                            .name(format!("Select {name}"))
                            .checked(selected)
                            .on_checked_change(move |v| selected.revise(|s| *s = v))
                    }));
                for (i, value) in [number, name, symbol, mass].into_iter().enumerate() {
                    row = row.cell(sapid::table_cell(format!("value-{i}")).text(value));
                }
                row
            }));
        sapid::table("selectable-periodic-elements")
            .theme(theme.clone())
            .class("sapid-demo-periodic-table")
            .with_row_borders(true)
            .head(sapid::table_head().row(head))
            .body(body)
            .show_in(scope)
    })
}
