use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let selected = scope.cage("one".to_owned());
        let palette = theme.clone();
        let grow = scope.cage(false);
        let justify = scope.cage(Some("start".to_owned()));
        let preview =
            each(scope.bond(move || vec![grow.get()]))
                .key(Clone::clone)
                .children(move |e| {
                    sapid::tabs("aligned-tabs")
                        .theme(palette.clone())
                        .name("Tab alignment")
                        .grow(e.value.get())
                        .class(justify.map(|v| {
                            format!("site-tabs-justify-{}", v.as_deref().unwrap_or("start"))
                        }))
                        .value(selected)
                        .on_value_change(move |s| selected.revise(|v| *v = s))
                        .tab(sapid::tab("one").label("First tab"))
                        .tab(sapid::tab("two").label("Second tab"))
                        .tab(sapid::tab("three").label("Third tab"))
                });
        let panel = view()
            .class("site-configurator site-action-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::switch("Grow", grow, &theme),
                    controls::select(
                        "tabs-justify",
                        "Justify",
                        justify,
                        &[
                            ("start", "Flex-start"),
                            ("center", "Center"),
                            ("end", "Flex-end"),
                            ("between", "Space-between"),
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
        let field_key = format!("tabs-position-{key}");
        let select_key = format!("tabs-position-select-{key}");
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
