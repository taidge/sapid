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

#[allow(unused_imports)]
mod controls {
    use glory::prelude::*;
    use glory::style::{Color, FlexWrap};
    use sapid::{Size, Theme};

    pub fn playground(preview: Box<dyn Widget>, controls: Vec<Box<dyn Widget>>) -> Box<dyn Widget> {
        Box::new(
            view()
                .class("site-button-playground")
                .style(
                    Style::new()
                        .display(Display::Flex)
                        .flex_wrap(FlexWrap::Wrap)
                        .gap(24.0),
                )
                .fill((
                    col()
                        .class("site-playground-preview")
                        .style(Style {
                            grow: Some(1.0),
                            align: Some(Align::Center),
                            justify: Some(Justify::Center),
                            ..Style::new()
                                .display(Display::Flex)
                                .flex_direction(FlexDirection::Column)
                                .gap(16.0)
                                .min_width(Length::Px(180.0))
                        })
                        .fill(preview),
                    col()
                        .class("site-playground-controls")
                        .style(
                            Style::new()
                                .display(Display::Flex)
                                .flex_direction(FlexDirection::Column)
                                .gap(16.0)
                                .width(Length::Px(240.0))
                                .max_width(Limit::Percent(100.0)),
                        )
                        .fill(controls),
                )),
        )
    }

    pub fn switch(label: &str, value: Cage<bool>, theme: &Theme) -> Box<dyn Widget> {
        Box::new(
            sapid::switch()
                .theme(theme.clone())
                .size(Size::Small)
                .with_thumb_indicator(true)
                .label(label.to_owned())
                .checked(value)
                .on_checked_change(move |next| value.revise(|current| *current = next)),
        )
    }
}
