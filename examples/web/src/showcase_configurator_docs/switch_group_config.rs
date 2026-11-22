use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::group_playground(theme, "switch")
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

    pub fn field_setting(
        key: String,
        label: &'static str,
        value: Cage<String>,
        theme: &Theme,
    ) -> Box<dyn Widget> {
        Box::new(
            sapid::field(key)
                .theme(theme.clone())
                .label(label)
                .control(move |field| {
                    sapid::text_field()
                        .field(field)
                        .value(value)
                        .placeholder("Enter prop value")
                        .on_value_change(move |next| value.revise(|current| *current = next))
                }),
        )
    }

    pub fn group_playground(theme: Theme, key: &'static str) -> impl Widget {
        widget(move |scope| {
            let label = scope.cage(
                if key == "checkbox" {
                    "Select your favorite frameworks/libraries"
                } else {
                    "Select your favorite framework/library"
                }
                .to_owned(),
            );
            let description = scope.cage("This is anonymous".to_owned());
            let error = scope.cage(String::new());
            let required = scope.cage(true);
            let group_theme = theme.clone();
            let preview = sapid::field(format!("{key}-group-preview"))
                .theme(theme.clone())
                .label(label)
                .description(description)
                .error(error)
                .with_asterisk(required)
                .style(
                    Style::new()
                        .width(Length::Percent(100.))
                        .max_width(Limit::Px(420.)),
                )
                .control(move |field| -> Box<dyn Widget> {
                    let options = ["React", "Svelte", "Angular", "Vue"];
                    match key {
                        "radio" => Box::new(
                            sapid::radio_group("config-framework-radio")
                                .field(field)
                                .default_value("React")
                                .orientation(glory::render::Orientation::Horizontal)
                                .items(options.map(|v| sapid::radio(v).label(v))),
                        ),
                        "switch" => Box::new(
                            sapid::group().theme(group_theme.clone()).fill(
                                options
                                    .map(|v| {
                                        sapid::switch()
                                            .theme(group_theme.clone())
                                            .size(Size::Small)
                                            .label(v)
                                            .default_checked(v == "React")
                                    })
                                    .into_iter()
                                    .collect::<Vec<_>>(),
                            ),
                        ),
                        _ => Box::new(
                            sapid::checkbox_group("config-framework-check")
                                .field(field)
                                .class("site-choice-group")
                                .default_selected(["React".to_owned()])
                                .items(options.map(|v| sapid::checkbox_item(v, v))),
                        ),
                    }
                });
            view()
                .class("site-configurator site-field-configurator")
                .fill(playground(
                    Box::new(preview),
                    vec![
                        field_setting(format!("{key}-group-config-label"), "Label", label, &theme),
                        field_setting(
                            format!("{key}-group-config-description"),
                            "Description",
                            description,
                            &theme,
                        ),
                        field_setting(format!("{key}-group-config-error"), "Error", error, &theme),
                        switch("With asterisk", required, &theme),
                    ],
                ))
                .show_in(scope)
        })
    }
}
