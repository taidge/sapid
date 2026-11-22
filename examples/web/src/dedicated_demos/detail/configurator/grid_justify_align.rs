use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let justify = scope.cage(Some("start".to_owned()));
        let align = scope.cage(Some("start".to_owned()));
        let layout = scope.bond(move || Style {
            justify: Some(controls::layout_justify(
                justify.get().as_deref().unwrap_or("start"),
            )),
            align: Some(controls::layout_align(
                align.get().as_deref().unwrap_or("start"),
            )),
            ..Style::new()
                .width(Length::Percent(100.))
                .min_height(Length::Px(120.))
                .grid_columns(glory::style::GridTracks::Tracks(
                    vec![glory::style::GridTrack::Percent(24.); 3],
                ))
        });
        let preview = sapid::grid("aligned-grid")
            .theme(theme.clone())
            .columns(3)
            .style(layout)
            .items((1..=3).map(|i| {
                sapid::grid_item(i.to_string()).fill(
                    view()
                        .style(Style::new().height(Length::Px([80., 120., 100.][i - 1])))
                        .fill(controls::grid_demo_cell(i.to_string(), &theme)),
                )
            }));
        let panel = view()
            .class("site-configurator site-action-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::select(
                        "grid-justify",
                        "Justify",
                        justify,
                        &[
                            ("start", "Flex-start"),
                            ("center", "Center"),
                            ("end", "Flex-end"),
                            ("between", "Space-between"),
                            ("around", "Space-around"),
                        ],
                        &theme,
                    ),
                    controls::select(
                        "grid-align",
                        "Align",
                        align,
                        &[
                            ("start", "Flex-start"),
                            ("center", "Center"),
                            ("end", "Flex-end"),
                        ],
                        &theme,
                    ),
                ],
            ));
        panel.show_in(scope)
    })
}
