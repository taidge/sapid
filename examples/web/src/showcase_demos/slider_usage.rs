use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::slider_playground(theme, "slider")
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

    const COLORS: &[(&str, Color)] = &[
        ("Gray", Color(134, 142, 150, 255)),
        ("Red", Color(250, 82, 82, 255)),
        ("Pink", Color(230, 73, 128, 255)),
        ("Grape", Color(190, 75, 219, 255)),
        ("Violet", Color(121, 80, 242, 255)),
        ("Indigo", Color(76, 110, 245, 255)),
        ("Blue", Color(34, 139, 230, 255)),
        ("Cyan", Color(21, 170, 191, 255)),
        ("Teal", Color(18, 184, 134, 255)),
        ("Green", Color(64, 192, 87, 255)),
        ("Lime", Color(130, 201, 30, 255)),
        ("Yellow", Color(250, 176, 5, 255)),
        ("Orange", Color(253, 126, 20, 255)),
    ];

    fn choice_color(name: &str) -> Option<Color> {
        COLORS
            .iter()
            .find(|(label, _)| *label == name)
            .map(|(_, color)| *color)
            .or_else(|| sapid::parse_hex_color(name).ok())
    }

    pub fn accent(mut theme: Theme, name: &str) -> Theme {
        if let Some(color) = choice_color(name) {
            theme.accent = color;
            theme.accent_text = color;
            theme.focus_ring = color;
            theme.input_focus = color;
        }
        theme
    }

    pub fn color_choices(selected: Cage<String>, theme: &Theme) -> Box<dyn Widget> {
        named_color_choices(
            "control-custom-color",
            "Color",
            selected.into(),
            move |next| selected.revise(|value| *value = next),
            theme,
        )
    }

    pub fn named_color_choices(
        key: &'static str,
        label: &'static str,
        selected: glory::Vary<String>,
        set: impl Fn(String) + Copy + 'static,
        theme: &Theme,
    ) -> Box<dyn Widget> {
        let palette = theme.clone();
        Box::new(widget(move |scope| {
            let source = selected.clone();
            let selected = scope.bond(move || source.get());
            let draft = scope.bond(move || {
                let value = selected.get();
                choice_color(&value)
                    .map(|color| sapid::format_hex_color(color, false))
                    .unwrap_or(value)
            });
            let custom = scope.cage(false);
            let mut chips: Vec<Box<dyn Widget>> = COLORS
                .iter()
                .map(|(name, color)| {
                    let active = scope.bond(move || {
                        if selected.get() == *name {
                            "site-control-swatch is-active".to_owned()
                        } else {
                            "site-control-swatch".to_owned()
                        }
                    });
                    Box::new(
                        sapid::button()
                            .unstyled()
                            .class(active)
                            .name(if label == "Color" {
                                format!("Use {name}")
                            } else {
                                format!("{label}: use {name}")
                            })
                            .label("")
                            .style(Style {
                                background: Some(*color),
                                ..Style::new()
                            })
                            .on_press(move || set((*name).to_owned())),
                    ) as Box<dyn Widget>
                })
                .collect();
            chips.push(Box::new(
                sapid::button()
                    .unstyled()
                    .class("site-control-swatch site-custom-color-trigger")
                    .name(if label == "Color" {
                        "Custom color".to_owned()
                    } else {
                        format!("{label}: custom color")
                    })
                    .label("")
                    .on_press(move || custom.revise(|v| *v = !*v)),
            ));
            let editor_theme = palette.clone();
            sapid::stack()
                .theme(palette.clone())
                .gap(4.)
                .fill((
                    sapid::text()
                        .theme(palette.clone())
                        .class("site-control-label")
                        .content(label),
                    sapid::group()
                        .theme(palette.clone())
                        .class("site-control-swatches")
                        .fill(chips),
                    when(custom, move || {
                        sapid::field(key)
                            .theme(editor_theme.clone())
                            .label("Custom color")
                            .control(move |field| {
                                sapid::color_input(format!("{key}-picker"))
                                    .field(field)
                                    .value(draft)
                                    .on_value_change(set)
                            })
                    }),
                ))
                .show_in(scope)
        }))
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

    pub fn step_size(index: f64) -> Size {
        [
            Size::ExtraSmall,
            Size::Small,
            Size::Medium,
            Size::Large,
            Size::ExtraLarge,
        ][index as usize]
    }
    pub fn step_radius(index: f64) -> f64 {
        [2., 4., 8., 16., 32.][index as usize]
    }

    pub fn slider_playground(theme: Theme, key: &'static str) -> impl Widget {
        widget(move |scope| {
            let range = key == "range-slider";
            let color = scope.cage("Blue".to_owned());
            let size = scope.cage(2.);
            let radius = scope.cage(4.);
            let hover = scope.cage(true);
            let always = scope.cage(false);
            let value = scope.cage(40.);
            let pair = scope.cage([20., 60.]);
            let palette = theme.clone();
            let preview = each(scope.bond(move || vec![(color.get(), size.get() as u8)]))
                .key(Clone::clone)
                .children(move |item| {
                    let (color, index) = item.value.get();
                    let palette = accent(palette.clone(), &color);
                    widget(move |scope| {
                        let options = scope.bond(move || {
                            format!(
                                "site-slider-preview{}{}",
                                if hover.get() {
                                    " site-slider-hover"
                                } else {
                                    ""
                                },
                                if always.get() {
                                    " site-slider-always"
                                } else {
                                    ""
                                }
                            )
                        });
                        let appearance = radius.map(|i| {
                            Style::new()
                                .width(Length::Percent(100.))
                                .radius(step_radius(*i))
                        });
                        let marks = [
                            sapid::SliderMark::new(20., "20%"),
                            sapid::SliderMark::new(50., "50%"),
                            sapid::SliderMark::new(80., "80%"),
                        ];
                        let slider: Box<dyn Widget> = if range {
                            Box::new(
                                sapid::range_slider("price-range")
                                    .theme(palette.clone())
                                    .size(step_size(index.into()))
                                    .labels(["Minimum price".into(), "Maximum price".into()])
                                    .minimum_gap(10.)
                                    .value(pair)
                                    .class("site-slider-track")
                                    .style(appearance)
                                    .marks(marks)
                                    .on_value_change(move |v| pair.revise(|current| *current = v)),
                            )
                        } else {
                            Box::new(
                                view()
                                    .style(
                                        Style::new()
                                            .position(Position::Relative)
                                            .width(Length::Percent(100.)),
                                    )
                                    .fill((
                                        sapid::slider("volume")
                                            .theme(palette.clone())
                                            .size(step_size(index.into()))
                                            .name("Volume")
                                            .value(value)
                                            .class("site-slider-track")
                                            .value_text(
                                                value.map(|v| Some(format!("{v:.0} percent"))),
                                            )
                                            .style(appearance)
                                            .marks(marks)
                                            .on_value_change(move |v| {
                                                value.revise(|current| *current = v)
                                            }),
                                        text()
                                            .class("site-slider-label")
                                            .style(value.map(|v| {
                                                Style::new()
                                                    .position(Position::Absolute)
                                                    .left(Length::Percent(*v))
                                                    .pointer_events(
                                                        glory::style::PointerEvents::None,
                                                    )
                                            }))
                                            .content(value.map(|v| format!("{v:.0}"))),
                                    )),
                            )
                        };
                        view()
                            .class(options)
                            .style(
                                Style::new()
                                    .width(Length::Percent(100.))
                                    .max_width(Limit::Px(400.)),
                            )
                            .fill(slider)
                            .show_in(scope)
                    })
                });
            view()
                .class("site-configurator site-field-configurator")
                .fill(playground(
                    Box::new(preview),
                    vec![
                        color_choices(color, &theme),
                        field_scale(format!("{key}-config-size"), "Size", size, &theme),
                        field_scale(format!("{key}-config-radius"), "Radius", radius, &theme),
                        switch("Show label on hover", hover, &theme),
                        switch("Label always on", always, &theme),
                    ],
                ))
                .show_in(scope)
        })
    }
}
