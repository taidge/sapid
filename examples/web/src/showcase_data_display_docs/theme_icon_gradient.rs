use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let from = scope.cage(Some("Blue".to_owned()));
        let to = scope.cage(Some("Grape".to_owned()));
        let angle = scope.cage(45.);
        let icon = sapid::theme_icon(sapid::Icon::Calendar)
            .theme(theme.clone())
            .name("Gradient calendar")
            .size(sapid::Size::ExtraLarge)
            .variant(sapid::ThemeIconVariant::Gradient)
            .color(glory::style::Color(255, 255, 255, 255))
            .style(scope.bond(move || {
                Style::new()
                    .radius(10.)
                    .background_image(controls::gradient_style(
                        from.get().as_deref(),
                        to.get().as_deref(),
                        angle.get(),
                    ))
            }));
        view()
            .class("site-configurator")
            .fill(controls::playground(
                Box::new(icon),
                vec![controls::gradient_controls(from, to, angle, &theme)],
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
        let field_key = format!("control-{key}");
        let select_key = format!("control-select-{key}");
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

    pub fn range(
        key: &str,
        label: &str,
        value: Cage<f64>,
        min: f64,
        max: f64,
        step: f64,
        theme: &Theme,
    ) -> Box<dyn Widget> {
        let label = label.to_owned();
        Box::new(
            sapid::stack().theme(theme.clone()).gap(8.).fill((
                sapid::text()
                    .theme(theme.clone())
                    .class("site-control-label")
                    .content(value.map({
                        let label = label.clone();
                        move |v| format!("{label} · {v:.0}")
                    })),
                sapid::slider(key.to_owned())
                    .theme(theme.clone())
                    .name(label)
                    .min(min)
                    .max(max)
                    .step(step)
                    .value(value)
                    .on_value_change(move |next| value.revise(|v| *v = next)),
            )),
        )
    }
    const COLORS: &[(&str, Color)] = &[
        ("Yellow", Color(250, 176, 5, 255)),
        ("Orange", Color(253, 126, 20, 255)),
        ("Red", Color(250, 82, 82, 255)),
        ("Pink", Color(230, 73, 128, 255)),
        ("Grape", Color(190, 75, 219, 255)),
        ("Violet", Color(121, 80, 242, 255)),
        ("Indigo", Color(76, 110, 245, 255)),
        ("Blue", Color(34, 139, 230, 255)),
        ("Cyan", Color(21, 170, 191, 255)),
        ("Teal", Color(18, 184, 134, 255)),
        ("Green", Color(64, 192, 87, 255)),
        ("Gray", Color(134, 142, 150, 255)),
    ];

    fn choice_color(name: &str) -> Option<Color> {
        COLORS
            .iter()
            .find(|(label, _)| *label == name)
            .map(|(_, color)| *color)
            .or_else(|| sapid::parse_hex_color(name).ok())
    }

    pub fn gradient_style(
        from: Option<&str>,
        to: Option<&str>,
        angle: f64,
    ) -> glory::style::LinearGradient {
        let stop = |name: Option<&str>| {
            choice_color(name.unwrap_or("Blue")).unwrap_or(Color(34, 139, 230, 255))
        };
        glory::style::LinearGradient {
            angle,
            stops: vec![stop(from), stop(to)],
        }
    }

    pub fn gradient_controls(
        from: Cage<Option<String>>,
        to: Cage<Option<String>>,
        angle: Cage<f64>,
        theme: &Theme,
    ) -> Box<dyn Widget> {
        const STOPS: &[(&str, &str)] = &[
            ("Blue", "Blue"),
            ("Grape", "Grape"),
            ("Orange", "Orange"),
            ("Green", "Green"),
            ("Pink", "Pink"),
            ("Teal", "Teal"),
        ];
        Box::new(sapid::stack().theme(theme.clone()).gap(12.).fill((
            select("gradient-start", "Gradient from", from, STOPS, theme),
            select("gradient-end", "Gradient to", to, STOPS, theme),
            range(
                "gradient-angle",
                "Gradient angle",
                angle,
                0.,
                360.,
                5.,
                theme,
            ),
        )))
    }
}
