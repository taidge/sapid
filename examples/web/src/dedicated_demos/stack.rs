use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let align = scope.cage(Some("stretch".to_owned()));
        let justify = scope.cage(Some("center".to_owned()));
        let gap = scope.cage(Some("md".to_owned()));
        let palette = theme.clone();
        let layout = scope.bond(move || Style {
            align: Some(match align.get().as_deref() {
                Some("center") => Align::Center,
                Some("start") => Align::Start,
                Some("end") => Align::End,
                _ => Align::Stretch,
            }),
            justify: Some(match justify.get().as_deref() {
                Some("start") => Justify::Start,
                Some("end") => Justify::End,
                Some("between") => Justify::Between,
                Some("around") => Justify::Around,
                _ => Justify::Center,
            }),
            ..Style::new()
                .height(Length::Px(300.))
                .width(Length::Percent(100.))
                .gap(palette.spacing(controls::size_value(gap.get().as_deref().unwrap_or("md"))))
        });
        let stack = sapid::stack()
            .theme(theme.clone())
            .class("site-stack-example")
            .style(layout)
            .fill(Vec::from(["1", "2", "3"].map(|label| {
                sapid::button()
                    .theme(theme.clone())
                    .variant(sapid::ButtonVariant::Secondary)
                    .label(label)
            })));
        view()
            .class("site-configurator")
            .fill(controls::playground(
                Box::new(stack),
                vec![
                    controls::select(
                        "stack-align",
                        "Align",
                        align,
                        &[
                            ("stretch", "Stretch"),
                            ("center", "Center"),
                            ("start", "Flex-start"),
                            ("end", "Flex-end"),
                        ],
                        &theme,
                    ),
                    controls::select(
                        "stack-justify",
                        "Justify",
                        justify,
                        &[
                            ("center", "Center"),
                            ("start", "Flex-start"),
                            ("end", "Flex-end"),
                            ("between", "Space-between"),
                            ("around", "Space-around"),
                        ],
                        &theme,
                    ),
                    controls::scale_slider("stack-gap", "Gap", gap, &theme),
                ],
            ))
            .show_in(scope)
    })
}
