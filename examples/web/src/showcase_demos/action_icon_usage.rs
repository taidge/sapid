use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let presses = scope.cage(0);
        let color = scope.cage("Blue".to_owned());
        let size = scope.cage(Some("md".to_owned()));
        let radius = scope.cage(8.);
        let variant = scope.cage(Some("filled".to_owned()));
        let palette = theme.clone();
        let action = each(scope.bond(move || vec![(color.get(), size.get(), variant.get())]))
            .key(Clone::clone)
            .children(move |_| {
                let mut theme = controls::accent(palette.clone(), &color.get());
                let c = theme.accent;
                theme.accent_soft = glory::style::Color(c.0, c.1, c.2, 28);
                let choice = variant.get();
                let variant = match choice.as_deref() {
                    Some("filled") => sapid::ActionIconVariant::Filled,
                    Some("outline") => sapid::ActionIconVariant::Outline,
                    Some("default") => sapid::ActionIconVariant::Default,
                    Some("subtle" | "transparent" | "white") => sapid::ActionIconVariant::Subtle,
                    _ => sapid::ActionIconVariant::Light,
                };
                let ink = if variant == sapid::ActionIconVariant::Filled {
                    theme.on_accent
                } else {
                    theme.accent_text
                };
                let white = choice.as_deref() == Some("white");
                let transparent = choice.as_deref() == Some("transparent");
                sapid::action_icon("Confirm selection")
                    .theme(theme)
                    .size(controls::size_value(size.get().as_deref().unwrap_or("md")))
                    .variant(variant)
                    .auto_contrast(false)
                    .class(if transparent {
                        "site-action-transparent"
                    } else {
                        ""
                    })
                    .style(radius.map(move |r| {
                        let mut style = Style::new().radius(*r);
                        if white {
                            style.background = Some(Color(255, 255, 255, 255))
                        }
                        style
                    }))
                    .icon(
                        glory::ui::graphic().drawing(sliders(ink)).style(
                            Style::new()
                                .width(Length::Percent(70.))
                                .height(Length::Percent(70.)),
                        ),
                    )
                    .on_press(move || presses.revise(|n| *n += 1))
            });
        let response_theme = theme.clone();
        let preview = sapid::stack().theme(theme.clone()).gap(8.).fill((
            action,
            when(presses.map(|n| *n > 0), move || {
                sapid::text()
                    .theme(response_theme.clone())
                    .content(presses.map(|n| format!("Confirmed selections: {n}")))
            }),
        ));
        view()
            .class("site-configurator site-action-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::select(
                        "action-variant",
                        "Variant",
                        variant,
                        &[
                            ("filled", "Filled"),
                            ("light", "Light"),
                            ("default", "Default"),
                            ("subtle", "Subtle"),
                            ("transparent", "Transparent"),
                            ("white", "White"),
                            ("outline", "Outline"),
                        ],
                        &theme,
                    ),
                    controls::color_choices(color, &theme),
                    controls::size_slider(size, &theme),
                    controls::pixel_scale(
                        "action_icon-radius",
                        "Radius",
                        radius,
                        [2., 4., 8., 16., 32.],
                        &theme,
                    ),
                ],
            ))
            .show_in(scope)
    })
}
fn sliders(color: Color) -> glory::render::GraphicProps {
    use glory::render::{GraphicPoint, PathCommand as Path};
    let p = GraphicPoint::new;
    let mut drawing = sapid::Icon::Check.drawing(color);
    drawing.commands = vec![
        Path::Move(p(2., 7.)),
        Path::Line(p(7., 7.)),
        Path::Move(p(11., 7.)),
        Path::Line(p(22., 7.)),
        Path::Move(p(2., 17.)),
        Path::Line(p(13., 17.)),
        Path::Move(p(17., 17.)),
        Path::Line(p(22., 17.)),
    ];
    for (x, y) in [(9., 7.), (15., 17.)] {
        drawing.commands.push(Path::Move(p(x + 2., y)));
        for i in 1..=12 {
            let a = f64::from(i) * std::f64::consts::TAU / 12.;
            drawing
                .commands
                .push(Path::Line(p(x + 2. * a.cos(), y + 2. * a.sin())))
        }
        drawing.commands.push(Path::Close);
    }
    drawing
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

    pub fn size_slider(value: Cage<Option<String>>, theme: &Theme) -> Box<dyn Widget> {
        scale_slider("control-size", "Size", value, theme)
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
}
