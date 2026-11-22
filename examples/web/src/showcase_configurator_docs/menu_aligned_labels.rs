use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let align = scope.cage(Some("with-indicators".to_owned()));
        let pin = scope.cage(true);
        let archived = scope.cage(false);
        let sort = scope.cage("newest".to_owned());
        let indicator = align.map(|value| {
            if value.as_deref() == Some("none") {
                "site-menu-indicator-collapsed".to_owned()
            } else {
                "site-menu-indicator-reserved".to_owned()
            }
        });
        let plain = |key, label| {
            sapid::menu_item(key)
                .label(label)
                .class(align.map(|value| {
                    if value.as_deref() == Some("all") {
                        "site-menu-align-label".to_owned()
                    } else {
                        String::new()
                    }
                }))
                .close_on_select(false)
        };
        let preview = sapid::dropdown_menu("menu-indicators")
            .open(true)
            .inline_panel()
            .theme(theme.clone())
            .width(240.)
            .label("Toggle menu")
            .item(plain("detail", "View details"))
            .item(plain("copy", "Duplicate"))
            .separator("choices")
            .item(
                sapid::menu_checkbox("pinned")
                    .label("Pinned")
                    .class(indicator)
                    .checked(pin)
                    .on_checked_change(move |next| pin.revise(|v| *v = next)),
            )
            .item(
                sapid::menu_checkbox("archive")
                    .label("Archived")
                    .class(indicator)
                    .checked(archived)
                    .on_checked_change(move |next| archived.revise(|v| *v = next)),
            )
            .separator("sort-options")
            .radio_group(
                sapid::menu_radio_group("sort")
                    .label("Sort order")
                    .value(sort)
                    .on_value_change(move |next| sort.revise(|v| *v = next))
                    .items([
                        sapid::menu_radio_item("newest", "Newest first").class(indicator),
                        sapid::menu_radio_item("oldest", "Oldest first").class(indicator),
                    ]),
            );
        view()
            .class("site-configurator site-action-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::select(
                    "menu-indicator-spacing",
                    "Align items labels",
                    align,
                    &[
                        ("with-indicators", "With-indicators"),
                        ("all", "All"),
                        ("none", "None"),
                    ],
                    &theme,
                )],
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

    pub fn select(
        key: &str,
        label: &str,
        value: Cage<Option<String>>,
        options: &'static [(&'static str, &'static str)],
        theme: &Theme,
    ) -> Box<dyn Widget> {
        let field_key = format!("menu-aligned-labels-{key}");
        let select_key = format!("menu-aligned-labels-select-{key}");
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
}
