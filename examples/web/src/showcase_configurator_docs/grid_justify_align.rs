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

    pub fn select(
        key: &str,
        label: &str,
        value: Cage<Option<String>>,
        options: &'static [(&'static str, &'static str)],
        theme: &Theme,
    ) -> Box<dyn Widget> {
        let field_key = format!("grid-justify-align-{key}");
        let select_key = format!("grid-justify-align-select-{key}");
        Box::new(
            sapid::field(field_key)
                .theme(theme.clone())
                .label(label.to_owned())
                .control(move |field| {
                    let mut select = sapid::native_select(select_key.clone()).field(field);
                    for (value, label) in options {
                        select = select.option(sapid::select_option(*value, *label));
                    }
                    select.value(value).on_value_change(move |next| {
                        if next.is_some() {
                            value.revise(|current| *current = next)
                        }
                    })
                }),
        )
    }

    pub fn layout_justify(value: &str) -> Justify {
        match value {
            "center" => Justify::Center,
            "end" => Justify::End,
            "between" => Justify::Between,
            "around" => Justify::Around,
            "evenly" => Justify::Evenly,
            _ => Justify::Start,
        }
    }

    pub fn grid_demo_cell(label: String, theme: &Theme) -> impl Widget {
        let (background, color) = if theme.scheme == sapid::ColorScheme::Dark {
            (theme.surface, theme.muted)
        } else {
            (
                glory::style::Color(231, 245, 255, 255),
                glory::style::Color(34, 139, 230, 255),
            )
        };
        view()
            .style(Style {
                justify: Some(Justify::Center),
                align: Some(Align::Center),
                background: Some(background),
                ..Style::new()
                    .display(Display::Flex)
                    .padding(glory::style::Edges::all(16.))
                    .height(Length::Percent(100.))
            })
            .fill(
                text()
                    .style(Style {
                        font_size: Some(20.),
                        color: Some(color),
                        ..Style::new().font_weight(700.)
                    })
                    .content(label),
            )
    }

    pub fn layout_align(value: &str) -> Align {
        match value {
            "center" => Align::Center,
            "end" => Align::End,
            "stretch" => Align::Stretch,
            _ => Align::Start,
        }
    }
}
