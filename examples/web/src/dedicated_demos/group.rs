use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let justify = scope.cage(Some("start".to_owned()));
        let gap = scope.cage(Some("md".to_owned()));
        let grow = scope.cage(false);
        let palette = theme.clone();
        let layout = scope.bond(move || Style {
            justify: Some(controls::layout_justify(
                justify.get().as_deref().unwrap_or("start"),
            )),
            ..Style::new()
                .width(Length::Percent(100.))
                .gap(palette.spacing(controls::size_value(gap.get().as_deref().unwrap_or("md"))))
        });
        let child = grow.map(|grow| {
            if *grow {
                Style {
                    grow: Some(1.),
                    ..Style::new().width(Length::Px(0.)).min_width(Length::Px(0.))
                }
            } else {
                Style::new()
            }
        });
        let preview = sapid::group()
            .theme(theme.clone())
            .style(layout)
            .fill(Vec::from(["First", "Second", "Third"].map(|label| {
                sapid::button()
                    .theme(theme.clone())
                    .variant(sapid::ButtonVariant::Secondary)
                    .style(child)
                    .label(label)
            })));
        view()
            .class("site-configurator site-layout-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::select(
                        "group-justify",
                        "Justify",
                        justify,
                        &[
                            ("start", "Flex-start"),
                            ("center", "Center"),
                            ("between", "Space-between"),
                            ("end", "Flex-end"),
                        ],
                        &theme,
                    ),
                    controls::scale_slider("group-gap", "Gap", gap, &theme),
                    controls::switch("Grow", grow, &theme),
                ],
            ))
            .show_in(scope)
    })
}
