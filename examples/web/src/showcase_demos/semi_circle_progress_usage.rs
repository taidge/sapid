use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(40_f64);

        let direction = scope.cage(Some("ltr".to_owned()));
        let orientation = scope.cage(Some("up".to_owned()));
        let color = scope.cage("Blue".to_owned());
        let size = scope.cage(200_f64);
        let thickness = scope.cage(12_f64);
        let palette = theme.clone();
        let preview = each(scope.bond(move || {
            vec![(
                direction.get(),
                orientation.get(),
                color.get(),
                size.get().to_bits(),
                thickness.get().to_bits(),
            )]
        }))
        .key(Clone::clone)
        .children(move |_| {
            let theme = palette.clone();

            sapid::semi_circle_progress(value)
                .theme(theme.clone())
                .name("Monthly target")
                .size(size.get())
                .filled_segment_color(controls::accent(theme.clone(), &color.get()).accent)
                .fill_direction(if direction.get().as_deref() == Some("rtl") {
                    sapid::SemiCircleFillDirection::RightToLeft
                } else {
                    sapid::SemiCircleFillDirection::LeftToRight
                })
                .semi_orientation(if orientation.get().as_deref() == Some("down") {
                    sapid::SemiCircleOrientation::Down
                } else {
                    sapid::SemiCircleOrientation::Up
                })
                .thickness(thickness.get())
                .label("Label")
                .value_text(value.map(|v| format!("{v:.0} percent of the monthly target")))
                .style(Style {
                    font_size: Some(18.),
                    ..Style::new()
                })
        });
        view()
            .class("site-configurator site-display-configurator site-semi-circle-progress-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::option_segments(
                        "semi_circle_progress-direction",
                        "Fill direction",
                        direction,
                        &[("rtl", "Right to left"), ("ltr", "Left to right")],
                        &theme,
                    ),
                    controls::option_segments(
                        "semi_circle_progress-orientation",
                        "Orientation",
                        orientation,
                        &[("up", "Up"), ("down", "Down")],
                        &theme,
                    ),
                    controls::named_color_choices(
                        "semi-circle-fill-color",
                        "Filled segment color",
                        color.into(),
                        move |next| color.revise(|value| *value = next),
                        &theme,
                    ),
                    controls::range(
                        "semi_circle_progress-size",
                        "Size",
                        size,
                        120.,
                        450.,
                        1.,
                        &theme,
                    ),
                    controls::range(
                        "semi_circle_progress-thickness",
                        "Thickness",
                        thickness,
                        1.,
                        20.,
                        1.,
                        &theme,
                    ),
                    controls::range(
                        "semi_circle_progress-value",
                        "Value",
                        value,
                        0.,
                        100.,
                        1.,
                        &theme,
                    ),
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

    pub fn option_segments(
        key: &'static str,
        label: &'static str,
        value: Cage<Option<String>>,
        options: &'static [(&str, &str)],
        theme: &Theme,
    ) -> Box<dyn Widget> {
        Box::new(
            sapid::stack().theme(theme.clone()).gap(4.).fill((
                sapid::text()
                    .theme(theme.clone())
                    .class("site-control-label")
                    .content(label),
                sapid::segmented_control(key)
                    .theme(theme.clone())
                    .size(Size::Small)
                    .name(label)
                    .full_width(true)
                    .value(value.map(|selected| selected.clone().unwrap_or_default()))
                    .segments(
                        options
                            .iter()
                            .map(|&(key, caption)| sapid::segment(key).label(caption)),
                    )
                    .on_value_change(move |next| value.revise(|current| *current = Some(next))),
            )),
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
                        move |v| {
                            let decimals = if step.fract() == 0. {
                                0
                            } else if (step * 10.).fract() == 0. {
                                1
                            } else {
                                2
                            };
                            format!("{label} · {v:.decimals$}")
                        }
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
}
