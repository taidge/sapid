use glory::prelude::*;
use glory::style::FlexWrap;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let gap = scope.cage(Some("md".to_owned()));
        let justify = scope.cage(Some("start".to_owned()));
        let align = scope.cage(Some("start".to_owned()));
        let direction = scope.cage(Some("row".to_owned()));
        let wrap = scope.cage(Some("wrap".to_owned()));
        let palette = theme.clone();
        let layout = scope.bond(move || Style {
            justify: Some(controls::layout_justify(
                justify.get().as_deref().unwrap_or("start"),
            )),
            align: Some(controls::layout_align(
                align.get().as_deref().unwrap_or("start"),
            )),
            background: Some(Color(0, 0, 0, 77)),
            ..Style::new()
                .min_height(Length::Px(50.))
                .width(Length::Percent(100.))
                .flex_direction(
                    if direction
                        .get()
                        .as_deref()
                        .unwrap_or("row")
                        .starts_with("column")
                    {
                        FlexDirection::Column
                    } else {
                        FlexDirection::Row
                    },
                )
                .flex_wrap(if wrap.get().as_deref() == Some("nowrap") {
                    FlexWrap::NoWrap
                } else {
                    FlexWrap::Wrap
                })
                .gap(palette.spacing(controls::size_value(gap.get().as_deref().unwrap_or("md"))))
        });
        let classes = scope.bond(move || {
            format!(
                "site-flex-{} site-flex-{}",
                direction.get().as_deref().unwrap_or("row"),
                wrap.get().as_deref().unwrap_or("wrap")
            )
        });
        let preview = sapid::flex()
            .theme(theme.clone())
            .class(classes)
            .style(layout)
            .fill(Vec::from(["Button 1", "Button 2", "Button 3"].map(
                |label| sapid::button().theme(theme.clone()).label(label),
            )));
        view()
            .class("site-configurator site-layout-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::scale_slider("flex-gap", "Gap", gap, &theme),
                    controls::select(
                        "flex-justify",
                        "Justify",
                        justify,
                        &[
                            ("start", "Flex-start"),
                            ("center", "Center"),
                            ("end", "Flex-end"),
                        ],
                        &theme,
                    ),
                    controls::select(
                        "flex-align",
                        "Align",
                        align,
                        &[
                            ("start", "Flex-start"),
                            ("center", "Center"),
                            ("end", "Flex-end"),
                        ],
                        &theme,
                    ),
                    controls::select(
                        "flex-direction",
                        "Direction",
                        direction,
                        &[
                            ("row", "Row"),
                            ("column", "Column"),
                            ("row-reverse", "Row-reverse"),
                            ("column-reverse", "Column-reverse"),
                        ],
                        &theme,
                    ),
                    controls::select(
                        "flex-wrap",
                        "Wrap",
                        wrap,
                        &[
                            ("wrap", "Wrap"),
                            ("nowrap", "Nowrap"),
                            ("wrap-reverse", "Wrap-reverse"),
                        ],
                        &theme,
                    ),
                ],
            ))
            .show_in(scope)
    })
}
