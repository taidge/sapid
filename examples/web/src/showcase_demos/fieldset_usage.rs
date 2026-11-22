use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let variant = scope.cage("default".to_owned());
        let radius = scope.cage(1.);
        let palette = theme.clone();
        let appearance = scope.bond(move || {
            let mut style = Style::new().radius(controls::step_radius(radius.get()));
            if variant.get() == "filled" {
                style.background = Some(palette.disabled);
                style.border_color = Some(palette.disabled);
            } else if variant.get() == "unstyled" {
                style.background = Some(glory::style::Color(0, 0, 0, 0));
                style.border_color = Some(glory::style::Color(0, 0, 0, 0));
            }
            style
        });
        let preview_theme = theme.clone();
        let preview = sapid::fieldset("profile")
            .theme(preview_theme.clone())
            .legend("Personal information")
            .style(scope.bond(move || {
                appearance
                    .get()
                    .width(Length::Percent(100.))
                    .max_width(Limit::Px(420.))
            }))
            .fill((
                sapid::field("profile-name")
                    .theme(preview_theme.clone())
                    .label("Your name")
                    .control(|field| sapid::text_field().field(field).placeholder("Your name")),
                sapid::field("profile-email")
                    .theme(preview_theme.clone())
                    .label("Email address")
                    .control(|field| {
                        sapid::text_field()
                            .field(field)
                            .placeholder("you@example.com")
                    }),
            ));
        view()
            .class("site-configurator site-field-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::segment_setting(
                        "fieldset-variant",
                        "Variant",
                        variant,
                        &[
                            ("default", "Default"),
                            ("filled", "Filled"),
                            ("unstyled", "Unstyled"),
                        ],
                        &theme,
                    ),
                    controls::field_scale("fieldset-radius".into(), "Radius", radius, &theme),
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

    pub fn field_scale(
        key: String,
        label: &'static str,
        step: Cage<f64>,
        theme: &Theme,
    ) -> Box<dyn Widget> {
        const NAMES: [&str; 5] = ["xs", "sm", "md", "lg", "xl"];
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
                    .marks((0..5).map(|i| sapid::SliderMark::unlabeled(f64::from(i))))
                    .value_text(step.map(|i| Some(NAMES[*i as usize].to_owned())))
                    .value(step)
                    .on_value_change(move |next| step.revise(|value| *value = next)),
            )),
        )
    }

    pub fn segment_setting(
        key: &'static str,
        label: &'static str,
        value: Cage<String>,
        options: &'static [(&'static str, &'static str)],
        theme: &Theme,
    ) -> Box<dyn Widget> {
        Box::new(
            sapid::segmented_control(key)
                .theme(theme.clone())
                .size(Size::Small)
                .class("site-variant-control")
                .label(label)
                .full_width(true)
                .value(value)
                .segments(
                    options
                        .iter()
                        .map(|&(key, label)| sapid::segment(key).label(label)),
                )
                .on_value_change(move |next| value.revise(|current| *current = next)),
        )
    }

    pub fn step_radius(index: f64) -> f64 {
        [2., 4., 8., 16., 32.][index as usize]
    }
}
