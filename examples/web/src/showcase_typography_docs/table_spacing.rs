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

    pub fn pixel_scale(
        key: &'static str,
        label: &'static str,
        value: Cage<f64>,
        stops: [f64; 5],
        theme: &Theme,
    ) -> Box<dyn Widget> {
        let current = value.map(move |px| {
            stops
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| (*px - **a).abs().total_cmp(&(*px - **b).abs()))
                .map_or(0., |(index, _)| index as f64)
        });
        Box::new(
            sapid::stack().theme(theme.clone()).gap(4.).fill((
                sapid::text()
                    .theme(theme.clone())
                    .class("site-control-label")
                    .content(label),
                sapid::slider(key)
                    .theme(theme.clone())
                    .name(label)
                    .min(0.)
                    .max(4.)
                    .step(1.)
                    .value(current)
                    .marks((0..5).map(|i| sapid::SliderMark::unlabeled(f64::from(i))))
                    .on_value_change(move |index| {
                        value.revise(|px| *px = stops[index.round().clamp(0., 4.) as usize])
                    }),
            )),
        )
    }
}
