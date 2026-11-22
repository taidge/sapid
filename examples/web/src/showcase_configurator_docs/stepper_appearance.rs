use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let active = scope.cage(1usize);
        let palette = theme.clone();
        let color = scope.cage("Blue".to_owned());
        let size = scope.cage(Some("md".to_owned()));
        let radius = scope.cage(32.);
        let preview = each(scope.bond(move || vec![(color.get(), size.get(), radius.get() as u8)]))
            .key(Clone::clone)
            .children(move |row| {
                let (color, size, radius) = row.value.get();
                sapid::stepper("step-appearance")
                    .theme(controls::accent(palette.clone(), &color))
                    .size(controls::size_value(size.as_deref().unwrap_or("md")))
                    .icon_radius(f64::from(radius))
                    .name("Configured steps")
                    .class("site-two-step-example")
                    .active(active)
                    .selectable(true)
                    .on_step_change(move |n| active.revise(|v| *v = n))
                    .steps(
                        [
                            ("one", "Step 1", "Create an account"),
                            ("two", "Step 2", "Verify email"),
                        ]
                        .map(|(key, label, description)| {
                            sapid::step(key).label(label).description(description)
                        }),
                    )
            });
        view()
            .class("site-configurator site-layout-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::color_choices(color, &theme),
                    controls::pixel_scale(
                        "step-radius",
                        "Radius",
                        radius,
                        [2., 4., 8., 16., 32.],
                        &theme,
                    ),
                    controls::size_slider(size, &theme),
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

    pub fn size_slider(value: Cage<Option<String>>, theme: &Theme) -> Box<dyn Widget> {
        scale_slider("stepper-appearance-size", "Size", value, theme)
    }

    pub fn scale_slider(
        key: &'static str,
        label: &'static str,
        value: Cage<Option<String>>,
        theme: &Theme,
    ) -> Box<dyn Widget> {
        let current = value.map(|current| match current.as_deref() {
            Some("xs") => 0.0,
            Some("sm") => 1.0,
            Some("lg") => 3.0,
            Some("xl") => 4.0,
            _ => 2.0,
        });
        let value_text = value.map(|current| {
            Some(
                match current.as_deref() {
                    Some("xs") => "Extra small",
                    Some("sm") => "Small",
                    Some("lg") => "Large",
                    Some("xl") => "Extra large",
                    _ => "Medium",
                }
                .to_owned(),
            )
        });
        Box::new(
            sapid::stack().theme(theme.clone()).gap(8.0).fill((
                sapid::text()
                    .theme(theme.clone())
                    .class("site-control-label")
                    .content(label),
                sapid::slider(key)
                    .theme(theme.clone())
                    .name(label)
                    .min(0.0)
                    .max(4.0)
                    .step(1.0)
                    .value(current)
                    .value_text(value_text)
                    .marks((0..5).map(|i| sapid::SliderMark::unlabeled(f64::from(i))))
                    .on_value_change(move |next| {
                        value.revise(|current| {
                            *current = Some(
                                match next.round() as u8 {
                                    0 => "xs",
                                    1 => "sm",
                                    3 => "lg",
                                    4 => "xl",
                                    _ => "md",
                                }
                                .to_owned(),
                            )
                        })
                    }),
            )),
        )
    }

    pub fn pixel_scale(
        key: &'static str,
        label: &'static str,
        value: Cage<f64>,
        stops: [f64; 5],
        theme: &Theme,
    ) -> Box<dyn Widget> {
        let current = value.map(move |px| {
            stops
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| (*px - **a).abs().total_cmp(&(*px - **b).abs()))
                .map_or(0., |(index, _)| index as f64)
        });
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
                    .value(current)
                    .marks((0..5).map(|i| sapid::SliderMark::unlabeled(f64::from(i))))
                    .on_value_change(move |index| {
                        value.revise(|px| *px = stops[index.round().clamp(0., 4.) as usize])
                    }),
            )),
        )
    }

    pub fn size_value(value: &str) -> Size {
        match value {
            "xs" => Size::ExtraSmall,
            "sm" => Size::Small,
            "lg" => Size::Large,
            "xl" => Size::ExtraLarge,
            _ => Size::Medium,
        }
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
            "stepper-appearance-custom-color",
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
}
