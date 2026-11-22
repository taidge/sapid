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

pub fn segmented_choice(
    key: &'static str,
    label: &'static str,
    value: Cage<String>,
    options: &'static [(&str, &str)],
    theme: &Theme,
) -> Box<dyn Widget> {
    Box::new(
        sapid::segmented_control(key)
            .theme(theme.clone())
            .size(Size::Small)
            .label(label)
            .full_width(true)
            .value(value)
            .segments(
                options
                    .iter()
                    .map(|&(key, caption)| sapid::segment(key).label(caption)),
            )
            .on_value_change(move |next| value.revise(|current| *current = next)),
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

pub fn checkbox(label: &str, value: Cage<bool>, theme: &Theme) -> Box<dyn Widget> {
    Box::new(
        sapid::checkbox()
            .theme(theme.clone())
            .label(label.to_owned())
            .checked(value)
            .on_checked_change(move |next| value.revise(|current| *current = next)),
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

pub fn text_field(label: &str, value: Cage<String>, theme: &Theme) -> Box<dyn Widget> {
    Box::new(
        sapid::field(format!("control-{label}"))
            .theme(theme.clone())
            .label(label.to_owned())
            .control(move |field| {
                sapid::text_field()
                    .field(field)
                    .placeholder("Enter prop value")
                    .value(value)
                    .on_value_change(move |next| value.revise(|current| *current = next))
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

pub fn radius_slider(value: Cage<f64>, theme: &Theme) -> Box<dyn Widget> {
    range("control-radius", "Radius", value, 0., 12., 1., theme)
}

pub fn variant(value: Cage<Option<String>>, theme: &Theme) -> Box<dyn Widget> {
    let current = value.map(|value| value.as_deref().unwrap_or("default").to_owned());
    Box::new(
        sapid::stack().theme(theme.clone()).gap(8.0).fill((
            sapid::text()
                .theme(theme.clone())
                .class("site-control-label")
                .content("Variant"),
            sapid::segmented_control("control-variant")
                .theme(theme.clone())
                .class("site-variant-control")
                .name("Variant")
                .value(current)
                .segments([
                    sapid::segment("default").label("Default"),
                    sapid::segment("filled").label("Filled"),
                    sapid::segment("unstyled").label("Unstyled"),
                ])
                .on_value_change(move |next| value.revise(|current| *current = Some(next))),
        )),
    )
}

pub fn size(value: Cage<Option<String>>, theme: &Theme) -> Box<dyn Widget> {
    select(
        "size",
        "Size",
        value,
        &[
            ("xs", "Extra small"),
            ("sm", "Small"),
            ("md", "Medium"),
            ("lg", "Large"),
            ("xl", "Extra large"),
        ],
        theme,
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

pub fn mark_color(theme: &Theme, name: &str) -> Color {
    if theme.scheme == sapid::ColorScheme::Dark {
        return choice_color(name).unwrap_or(theme.accent);
    }
    match name {
        "Gray" => Color(233, 236, 239, 255),
        "Red" => Color(255, 201, 201, 255),
        "Pink" => Color(252, 194, 215, 255),
        "Grape" => Color(238, 190, 250, 255),
        "Violet" => Color(208, 191, 255, 255),
        "Indigo" => Color(186, 200, 255, 255),
        "Blue" => Color(165, 216, 255, 255),
        "Cyan" => Color(153, 233, 242, 255),
        "Teal" => Color(150, 242, 215, 255),
        "Green" => Color(178, 242, 187, 255),
        "Lime" => Color(216, 245, 162, 255),
        "Yellow" => Color(255, 236, 153, 255),
        "Orange" => Color(255, 216, 168, 255),
        _ => choice_color(name).unwrap_or(theme.accent),
    }
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
    let stop = |key, label, value: Cage<Option<String>>| {
        named_color_choices(
            key,
            label,
            value.map(|v| v.clone().unwrap_or_default()).into(),
            move |next| value.revise(|current| *current = Some(next)),
            theme,
        )
    };
    Box::new(sapid::stack().theme(theme.clone()).gap(12.).fill((
        stop("gradient-start", "Gradient from", from),
        stop("gradient-end", "Gradient to", to),
        range(
            "gradient-angle",
            "Gradient degree",
            angle,
            0.,
            360.,
            1.,
            theme,
        ),
    )))
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

pub fn multiline_playground_style(
    scope: &Scope,
    style: Bond<Style>,
    monospace: bool,
) -> Bond<Style> {
    scope.bond(move || {
        let mut appearance = style.get();
        let size = appearance.font_size.unwrap_or(14.) - if monospace { 2. } else { 0. };
        appearance.font_size = Some(size);
        appearance.line_height = Some(size * 1.55);
        appearance.padding.get_or_insert(Edges {
            left: 12.,
            right: 12.,
            top: 5.5,
            bottom: 5.5,
        });
        appearance
    })
}

pub fn field_playground<W: Widget>(
    theme: Theme,
    key: &'static str,
    default_label: &'static str,
    default_description: &'static str,
    build: impl Fn(sapid::FieldControl, Bond<Style>) -> W + Clone + 'static,
) -> impl Widget {
    picker_field_playground(
        theme,
        key,
        default_label,
        default_description,
        None,
        false,
        move |field, style, _| build(field, style),
    )
}

pub fn picker_field_playground<W: Widget>(
    theme: Theme,
    key: &'static str,
    default_label: &'static str,
    default_description: &'static str,
    default_placeholder: Option<&'static str>,
    default_asterisk: bool,
    build: impl Fn(sapid::FieldControl, Bond<Style>, Cage<String>) -> W + Clone + 'static,
) -> impl Widget {
    widget(move |scope| {
        let variant = scope.cage("default".to_owned());
        let size = scope.cage(1.);
        let radius = scope.cage(2.);
        let label = scope.cage(default_label.to_owned());
        let description = scope.cage(default_description.to_owned());
        let error = scope.cage(String::new());
        let placeholder = scope.cage(default_placeholder.unwrap_or_default().to_owned());
        let disabled = scope.cage(false);
        let asterisk = scope.cage(default_asterisk);
        let palette = theme.clone();
        let style = scope.bond(move || {
            let index = size.get() as usize;
            let font_size = [12., 14., 16., 18., 20.][index];
            let transparent = Color(0, 0, 0, 0);
            let mut appearance = Style {
                font_size: Some(font_size),
                line_height: Some(font_size * 1.4),
                ..Style::new()
                    .width(Length::Percent(100.))
                    .min_width(Length::Px(0.))
                    .min_height(Length::Px([30., 36., 42., 50., 60.][index]))
                    .radius([2., 4., 8., 16., 32.][radius.get() as usize])
            };
            match variant.get().as_str() {
                "filled" => {
                    appearance.background = Some(if palette.disabled == palette.input {
                        palette.border
                    } else {
                        palette.disabled
                    });
                    appearance.border_color = Some(transparent);
                }
                "unstyled" => {
                    appearance.background = Some(transparent);
                    appearance.border_color = Some(transparent);
                }
                _ => {}
            }
            if !error.get().is_empty() {
                appearance.border_color = Some(palette.danger);
            }
            appearance
        });
        let build = build.clone();
        let preview = sapid::field(key)
            .theme(theme.clone())
            .enabled(disabled.map(|flag| !*flag))
            .label(label)
            .description(description)
            .error(error)
            .with_asterisk(asterisk)
            .style(
                Style::new()
                    .width(Length::Percent(100.))
                    .max_width(Limit::Px(340.))
                    .gap(2.),
            )
            .control(move |field| build(field, style, placeholder));
        let variant_control = sapid::segmented_control(format!("{key}-variant"))
            .theme(theme.clone())
            .label("Variant")
            .class("site-variant-control")
            .size(Size::Small)
            .full_width(true)
            .value(variant)
            .segments([
                sapid::segment("default").label("Default"),
                sapid::segment("filled").label("Filled"),
                sapid::segment("unstyled").label("Unstyled"),
            ])
            .on_value_change(move |next| variant.revise(|current| *current = next));
        let mut settings: Vec<Box<dyn Widget>> = vec![
            Box::new(variant_control),
            field_scale(format!("{key}-size"), "Size", size, &theme),
            field_scale(format!("{key}-radius"), "Radius", radius, &theme),
            field_setting(format!("{key}-label"), "Label", label, &theme),
            switch("With asterisk", asterisk, &theme),
            field_setting(
                format!("{key}-description"),
                "Description",
                description,
                &theme,
            ),
            field_setting(format!("{key}-error"), "Error", error, &theme),
        ];
        if default_placeholder.is_some() {
            settings.insert(
                0,
                field_setting(
                    format!("{key}-placeholder"),
                    "Placeholder",
                    placeholder,
                    &theme,
                ),
            );
            settings.push(switch("Disabled", disabled, &theme));
        }
        view()
            .class("site-configurator site-field-configurator")
            .fill(playground(Box::new(preview), settings))
            .show_in(scope)
    })
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

pub fn chart_curve(value: &str) -> sapid::charts::ChartCurve {
    use sapid::charts::ChartCurve::*;
    match value {
        "bump" => Bump,
        "natural" => Natural,
        "monotone" => Monotone,
        "step" => Step,
        "stepBefore" => StepBefore,
        "stepAfter" => StepAfter,
        _ => Linear,
    }
}
pub fn chart_curve_control(
    key: &str,
    value: Cage<Option<String>>,
    theme: &Theme,
) -> Box<dyn Widget> {
    select(
        key,
        "Curve type",
        value,
        &[
            ("bump", "Bump"),
            ("linear", "Linear"),
            ("natural", "Natural"),
            ("monotone", "Monotone"),
            ("step", "Step"),
            ("stepBefore", "StepBefore"),
            ("stepAfter", "StepAfter"),
        ],
        theme,
    )
}
pub fn chart_axes(value: &str) -> sapid::charts::ChartAxes {
    use sapid::charts::ChartAxes::*;
    match value {
        "x" => X,
        "y" => Y,
        "xy" => Both,
        _ => None,
    }
}
pub fn cartesian_playground(
    theme: Theme,
    key: &'static str,
    kind: sapid::charts::CartesianKind,
    data: sapid::charts::ChartData,
) -> impl Widget {
    use sapid::charts::*;
    widget(move |scope| {
        let curve = scope.cage(Some("linear".to_owned()));
        let grid = scope.cage("x".to_owned());
        let ticks = scope.cage("y".to_owned());
        let x_axis = scope.cage(true);
        let y_axis = scope.cage(true);
        let dots = scope.cage(kind != CartesianKind::Bar);
        let gradient = scope.cage(kind == CartesianKind::Area);
        let options = scope.bond(move || CartesianOptions {
            domain: if matches!(kind, CartesianKind::Line | CartesianKind::Area) {
                Some(ChartDomain::new(0., 3600.).unwrap())
            } else {
                None
            },
            presentation: ChartPresentation {
                edge_categories: true,
                curve: chart_curve(curve.get().as_deref().unwrap_or("linear")),
                grid: chart_axes(&grid.get()),
                tick_lines: chart_axes(&ticks.get()),
                x_axis: x_axis.get(),
                y_axis: y_axis.get(),
                dots: dots.get(),
                area_gradient: gradient.get(),
                fill_opacity: 0.2,
                ..Default::default()
            },
            ..Default::default()
        });
        let mut palette = ChartTheme::from_theme(&theme);
        palette.series[..3].copy_from_slice(&[
            Color(76, 110, 245, 255),
            Color(34, 139, 230, 255),
            Color(18, 184, 134, 255),
        ]);
        let preview = chart(key)
            .kind(kind)
            .theme(theme.clone())
            .name("Monthly series")
            .frame(ChartFrame {
                width: 520.,
                height: 300.,
            })
            .chart_theme(palette)
            .options(options)
            .data(data.clone())
            .data_table(false)
            .legend(false)
            .navigation(false);
        let axes = &[("x", "x"), ("y", "y"), ("xy", "xy"), ("none", "none")];
        let mut settings = Vec::new();
        if kind != CartesianKind::Bar {
            settings.push(chart_curve_control("curve-type", curve, &theme));
        }
        settings.push(segmented_choice(
            "primary-ticks",
            "Tick line",
            ticks,
            axes,
            &theme,
        ));
        settings.push(segmented_choice(
            "primary-grid",
            "Grid axis",
            grid,
            axes,
            &theme,
        ));
        if kind == CartesianKind::Area {
            settings.push(switch("With gradient", gradient, &theme));
        }
        settings.push(switch("With xaxis", x_axis, &theme));
        settings.push(switch("With yaxis", y_axis, &theme));
        if kind != CartesianKind::Bar {
            settings.push(switch("With dots", dots, &theme));
        }
        view()
            .class("site-configurator site-chart-configurator")
            .fill(playground(Box::new(preview), settings))
            .show_in(scope)
    })
}
pub fn choice_playground(theme: Theme, key: &'static str) -> impl Widget {
    widget(move |scope| {
        let chip = key == "chip";
        let radio = key == "radio";
        let toggle = key == "switch";
        let label = scope.cage(
            match key {
                "chip" => "Awesome chip",
                "radio" => "I cannot be unchecked",
                _ => "I agree to sell my privacy",
            }
            .to_owned(),
        );
        let description = scope.cage(String::new());
        let error = scope.cage(String::new());
        let position = scope.cage("right".to_owned());
        let color = scope.cage("Blue".to_owned());
        let variant = scope.cage("filled".to_owned());
        let size = scope.cage(1.);
        let radius = scope.cage(if chip || toggle { 4. } else { 1. });
        let disabled = scope.cage(false);
        let mixed = scope.cage(false);
        let marks = scope.cage(true);
        let checked = scope.cage(true);
        let palette = theme.clone();
        let preview =
            each(scope.bond(move || vec![(size.get() as usize, marks.get(), color.get())]))
                .key(Clone::clone)
                .children(move |item| {
                    let (index, with_marks, selected_color) = item.value.get();
                    let component_size = step_size(index as f64);

                    let field_theme = accent(palette.clone(), &selected_color);
                    let shade = palette.clone();
                    widget(move |scope| {
                        let classes = scope.bond(move || {
                            format!(
                                "site-choice-input site-choice-{} site-choice-{}",
                                position.get(),
                                variant.get()
                            )
                        });
                        let palette = shade.clone();
                        let appearance = scope.bond(move || {
                            let accent = accent(palette.clone(), &color.get()).accent;
                            Style {
                                border_color: Some(accent),
                                color: Some(accent),
                                ..Style::new().radius(step_radius(radius.get()))
                            }
                        });
                        let state = scope.bond(move || {
                            if mixed.get() {
                                sapid::CheckState::Mixed
                            } else {
                                sapid::CheckState::from(checked.get())
                            }
                        });
                        let field = sapid::field(format!("choice-config-{key}-preview-field"))
                            .theme(field_theme.clone())
                            .size(component_size)
                            .description(description)
                            .error(error)
                            .enabled(disabled.map(|v| !*v));
                        field
                            .control(move |field| -> Box<dyn Widget> {
                                if chip {
                                    Box::new(
                                        sapid::chip()
                                            .field(field)
                                            .class(classes)
                                            .style(appearance)
                                            .label(label)
                                            .checked(checked)
                                            .on_checked_change(move |next| {
                                                checked.revise(|v| *v = next)
                                            }),
                                    )
                                } else if radio {
                                    Box::new(
                                        sapid::radio_group("radio-preview")
                                            .field(field)
                                            .name("Radio preview")
                                            .default_value("option")
                                            .radio(
                                                sapid::radio("option")
                                                    .class(classes)
                                                    .style(appearance)
                                                    .label(label),
                                            ),
                                    )
                                } else if toggle {
                                    Box::new(
                                        sapid::switch()
                                            .field(field)
                                            .class(classes)
                                            .style(appearance)
                                            .label(label)
                                            .with_thumb_indicator(with_marks)
                                            .checked(checked)
                                            .on_checked_change(move |next| {
                                                checked.revise(|v| *v = next)
                                            }),
                                    )
                                } else {
                                    Box::new(
                                        sapid::mixed_checkbox()
                                            .field(field)
                                            .class(classes)
                                            .style(appearance)
                                            .label(label)
                                            .state(state)
                                            .on_state_change(move |next| {
                                                mixed.revise(|v| *v = false);
                                                checked.revise(|v| {
                                                    *v = next == sapid::CheckState::Checked
                                                })
                                            }),
                                    )
                                }
                            })
                            .show_in(scope)
                    })
                });
        let mut settings = Vec::new();
        if chip || toggle {
            settings.push(color_choices(color, &theme));
        }
        if toggle {
            settings.push(switch("With thumb indicator", marks, &theme));
        }
        if !chip {
            settings.extend([
                segment_setting(
                    "choice-position",
                    "Label position",
                    position,
                    &[("right", "Right"), ("left", "Left")],
                    &theme,
                ),
                field_setting(format!("choice-config-{key}-label"), "Label", label, &theme),
                field_setting(
                    format!("choice-config-{key}-description"),
                    "Description",
                    description,
                    &theme,
                ),
                field_setting(format!("choice-config-{key}-error"), "Error", error, &theme),
            ]);
        }
        if radio {
            settings.push(field_scale(
                format!("choice-config-{key}-size"),
                "Size",
                size,
                &theme,
            ));
        }
        if !chip && !toggle {
            settings.push(color_choices(color, &theme));
        }
        if !toggle {
            let variants: &'static [(&str, &str)] = if chip {
                &[
                    ("filled", "Filled"),
                    ("outline", "Outline"),
                    ("light", "Light"),
                ]
            } else {
                &[("filled", "Filled"), ("outline", "Outline")]
            };
            settings.push(segment_setting(
                "choice-variant",
                "Variant",
                variant,
                variants,
                &theme,
            ));
        }
        if !radio {
            if key == "checkbox" {
                settings.push(field_scale(
                    format!("choice-config-{key}-radius"),
                    "Radius",
                    radius,
                    &theme,
                ));
            }
            settings.push(field_scale(
                format!("choice-config-{key}-size"),
                "Size",
                size,
                &theme,
            ));
            if key != "checkbox" {
                settings.push(field_scale(
                    format!("choice-config-{key}-radius"),
                    "Radius",
                    radius,
                    &theme,
                ));
            }
        }
        if !radio && !chip {
            settings.push(switch("Disabled", disabled, &theme));
        }
        if key == "checkbox" {
            settings.push(switch("Indeterminate", mixed, &theme));
        }
        view()
            .class("site-configurator site-field-configurator")
            .fill(playground(Box::new(preview), settings))
            .show_in(scope)
    })
}

pub fn chart_example_data() -> sapid::charts::ChartData {
    use sapid::charts::*;
    ChartData {
        categories: ["Jan", "Feb", "Mar", "Apr", "May", "Jun"]
            .into_iter()
            .map(|month| ChartCategory::new(month, month))
            .collect(),
        series: vec![
            ChartSeries::new("apples", "Apples", [12., 18., 14., 20., 17., 24.].map(Some)),
            ChartSeries::new("pears", "Pears", [8., 10., 9., 12., 11., 13.].map(Some))
                .kind(ChartSeriesKind::Bar),
        ],
    }
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
                                        .value_text(value.map(|v| Some(format!("{v:.0} percent"))))
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
                                                .pointer_events(glory::style::PointerEvents::None)
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
pub fn group_playground(theme: Theme, key: &'static str) -> impl Widget {
    widget(move |scope| {
        let label = scope.cage(if key == "checkbox" { "Select your favorite frameworks/libraries" } else { "Select your favorite framework/library" }.to_owned());
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

pub fn color_detail(
    theme: Theme,
    key: &'static str,
    input: bool,
    prop: &'static str,
) -> impl Widget {
    widget(move |scope| {
        let formats = &[
            ("hex", "HEX"),
            ("hexa", "HEXA"),
            ("rgb", "RGB"),
            ("rgba", "RGBA"),
            ("hsl", "HSL"),
            ("hsla", "HSLA"),
        ];
        let selected = scope.cage(Some("hex".to_owned()));
        let count = scope.cage(7.);
        let size = scope.cage(1.);
        let initial = if prop == "Format" {
            glory::style::Color(197, 216, 89, 255)
        } else {
            glory::style::Color(34, 139, 230, 255)
        };
        let color = scope.cage(sapid::Hsva::from_rgba(initial));
        let palette = theme.clone();
        let preview = each(
            scope.bond(move || vec![(selected.get(), count.get() as usize, size.get() as usize)]),
        )
        .key(Clone::clone)
        .children(move |item| -> Box<dyn Widget> {
            let (keyed, columns, index) = item.value.get();
            let format = match keyed.as_deref() {
                Some("hexa") => sapid::ColorFormat::Hexa,
                Some("rgb") => sapid::ColorFormat::Rgb,
                Some("rgba") => sapid::ColorFormat::Rgba,
                Some("hsl") => sapid::ColorFormat::Hsl,
                Some("hsla") => sapid::ColorFormat::Hsla,
                _ => sapid::ColorFormat::Hex,
            };
            let swatches = [
                "#25262b", "#868e96", "#fa5252", "#e64980", "#be4bdb", "#7950f2", "#4c6ef5",
                "#228be6", "#15aabf", "#12b886", "#40c057", "#82c91e", "#fab005", "#fd7e14",
            ];
            let formatted = color.map(move |v| {
                sapid::format_color(v.to_rgba().expect("accepted picker color"), format)
            });
            if input && prop == "Format" {
                Box::new(
                    sapid::field(format!("{key}-field"))
                        .theme(palette.clone())
                        .label("Choose color format")
                        .style(
                            Style::new()
                                .width(Length::Percent(100.))
                                .max_width(Limit::Px(340.)),
                        )
                        .control(move |field| {
                            sapid::color_input(key)
                                .field(field)
                                .format(format)
                                .value(formatted)
                                .on_value_change(move |next| {
                                    if let Ok(v) = sapid::parse_color(&next, format) {
                                        color.revise(|old| *old = sapid::Hsva::from_rgba(v))
                                    }
                                })
                        }),
                )
            } else {
                let mut picker = sapid::color_picker(key)
                    .theme(palette.clone())
                    .size(step_size(index as f64))
                    .plane_size(if prop == "Swatches per row" { 240. } else { [180., 200., 240., 280., 320.][index] })
                    .plane_height([100., 110., 120., 140., 160.][index])
                    .name("Choose color")
                    .value(color)
                    .show_alpha(format.alpha() || prop == "Size")
                    .on_value_change(move |next| color.revise(|v| *v = next));
                if prop != "Format" {
                    picker = picker.swatches(swatches).swatches_per_row(columns);
                }
                let mut stack = sapid::stack().theme(palette.clone()).gap(8.).fill(picker);
                if prop == "Format" {
                    stack = stack.fill(sapid::text().theme(palette.clone()).content(formatted));
                }
                Box::new(stack)
            }
        });
        let control = match prop {
            "Format" => select(key, "Format", selected, formats, &theme),
            "Size" => field_scale(format!("{key}-size"), "Size", size, &theme),
            _ => range(
                &format!("{key}-columns"),
                "Swatches per row",
                count,
                2.,
                14.,
                1.,
                &theme,
            ),
        };
        view()
            .class("site-configurator site-field-configurator")
            .fill(playground(Box::new(preview), vec![control]))
            .show_in(scope)
    })
}
pub fn check_icon_playground(theme: Theme, key: &'static str, multiple: bool) -> impl Widget {
    widget(move |scope| {
        let active = scope.cage(true);
        let aligned = scope.cage(false);
        let position = scope.cage("left".to_owned());
        let selected = scope.cage(Some("React".to_owned()));
        let selected_many = scope.cage(vec!["React".to_owned()]);
        let opened = scope.cage(true);
        let palette = theme.clone();
        let preview = each(scope.bond(move || vec![(active.get(), aligned.get(), position.get())]))
            .key(Clone::clone)
            .children(move |item| -> Box<dyn Widget> {
                let (active, aligned, position) = item.value.get();
                let position = if !active {
                    sapid::SelectCheckPosition::Hidden
                } else if position == "left" {
                    sapid::SelectCheckPosition::Left
                } else {
                    sapid::SelectCheckPosition::Right
                };
                let palette = palette.clone();
                Box::new(
                    sapid::field(format!("{key}-field"))
                        .theme(palette.clone())
                        .class("site-check-options-field")
                        .label("Control check icon")
                        .control(move |field| -> Box<dyn Widget> {
                            let choices =
                                ["React", "Angular", "Svelte", "Vue"].map(|v| sapid::select_option(v, v));
                            let style = Style::new()
                                .width(Length::Percent(100.))
                                .max_width(Limit::Px(340.));
                            if multiple {
                                Box::new(
                                    sapid::multi_select(key)
                                        .theme(palette.clone())
                                        .field(field)
                                        .placeholder("Pick value")
                                        .style(style)
                                        .options(choices)
                                        .check_position(position)
                                        .with_aligned_labels(aligned)
                                        .value(selected_many)
                                        .open(opened)
                                        .on_open_change(move |next| opened.revise(|v| *v = next))
                                        .on_value_change(move |next| selected_many.revise(|v| *v = next)),
                                )
                            } else {
                                Box::new(
                                    sapid::select(key)
                                        .theme(palette.clone())
                                        .field(field)
                                        .placeholder("Pick value")
                                        .style(style)
                                        .options(choices)
                                        .check_position(position)
                                        .with_aligned_labels(aligned)
                                        .value(selected)
                                        .open(opened)
                                        .on_open_change(move |next| opened.revise(|v| *v = next))
                                        .on_value_change(move |next| selected.revise(|v| *v = next)),
                                )
                            }
                        }),
                )
            });
        view()
            .class("site-configurator site-field-configurator")
            .fill(playground(
                Box::new(preview),
                vec![
                    switch("With check icon", active, &theme),
                    switch("With aligned labels", aligned, &theme),
                    segment_setting(
                        "check-icon-side",
                        "Check icon position",
                        position,
                        &[("left", "Left"), ("right", "Right")],
                        &theme,
                    ),
                ],
            ))
            .show_in(scope)
    })
}

pub fn segment_example_data() -> sapid::charts::PlotData {
    use sapid::charts::*;
    PlotData::values(
        [
            ("usa", "USA", 400.),
            ("india", "India", 300.),
            ("japan", "Japan", 100.),
            ("other", "Other", 200.),
        ]
        .map(|(key, label, value)| ValueDatum::new(key, label, value)),
    )
}
pub fn chart_palette(theme: &Theme) -> sapid::charts::ChartTheme {
    let mut palette = sapid::charts::ChartTheme::from_theme(theme);
    palette.series[..4].copy_from_slice(&[
        Color(76, 110, 245, 255),
        Color(250, 176, 5, 255),
        Color(18, 184, 134, 255),
        Color(134, 142, 150, 255),
    ]);
    palette
}

pub fn fruit_sales_chart(
    theme: &Theme,
    key: &'static str,
    kind: sapid::charts::CartesianKind,
) -> sapid::charts::Chart {
    use sapid::charts::*;
    let data = fruit_sales_data(kind);
    let mut palette = ChartTheme::from_theme(theme);
    palette.series[..3].copy_from_slice(&[
        Color(76, 110, 245, 255),
        Color(34, 139, 230, 255),
        Color(18, 184, 134, 255),
    ]);
    chart(key)
        .theme(theme.clone())
        .kind(kind)
        .chart_theme(palette)
        .frame(ChartFrame {
            width: 640.,
            height: 300.,
        })
        .data(data)
        .data_table(false)
        .legend(false)
        .navigation(false)
}
pub fn fruit_sales_data(kind: sapid::charts::CartesianKind) -> sapid::charts::ChartData {
    use sapid::charts::*;
    let categories = ["Mar 22", "Mar 23", "Mar 24", "Mar 25", "Mar 26"]
        .into_iter()
        .map(|v| ChartCategory::new(v.to_ascii_lowercase().replace(' ', "-"), v))
        .collect();
    let mut series = vec![
        ChartSeries::new(
            "apples",
            "Apples",
            [2890., 2756., 3322., 3470., 3129.].map(Some),
        ),
        ChartSeries::new(
            "oranges",
            "Oranges",
            [2338., 2103., 986., 2108., 1726.].map(Some),
        ),
        ChartSeries::new(
            "tomatoes",
            "Tomatoes",
            [2452., 2402., 1821., 2809., 2290.].map(Some),
        ),
    ];
    if kind == CartesianKind::Composite {
        series[1].kind = ChartSeriesKind::Area;
        series[2].kind = ChartSeriesKind::Bar;
    }
    ChartData { categories, series }
}

pub fn reference_sales_data(kind:sapid::charts::CartesianKind) -> sapid::charts::ChartData {
    use sapid::charts::*;
    if kind == CartesianKind::Bar {
        return ChartData {
            categories: ["January","February","March","April","May","June"].map(|month| ChartCategory::new(month.to_lowercase(),month)).into(),
            series: vec![
                ChartSeries::new("smartphones","Smartphones",[1200.,1900.,400.,1000.,800.,750.].map(Some)),
                ChartSeries::new("laptops","Laptops",[900.,1200.,1000.,200.,1400.,600.].map(Some)),
                ChartSeries::new("tablets","Tablets",[200.,400.,200.,800.,1200.,1000.].map(Some)),
            ],
        };
    }
    let mut data = fruit_sales_data(kind);
    if kind == CartesianKind::Composite {
        let mut oranges=ChartSeries::new("oranges","Oranges",[723.,1887.,986.,987.,403.].map(Some));
        oranges.kind=ChartSeriesKind::Area;
        let apples=ChartSeries::new("apples","Apples",[2301.,2181.,1821.,2764.,1821.].map(Some));
        let tomatoes=data.series.pop().unwrap();
        data.series=vec![tomatoes,oranges,apples];
    }
    data
}
pub fn reference_sales_palette(theme:&Theme,kind:sapid::charts::CartesianKind) -> sapid::charts::ChartTheme {
    use sapid::charts::*;
    let mut palette = ChartTheme::from_theme(theme);
    palette.series[..3].copy_from_slice(&match kind {
        CartesianKind::Composite => [Color(18,120,255,51),Color(230,119,0,255),Color(224,49,49,255)],
        CartesianKind::Bar => [Color(121,80,242,255),Color(34,139,230,255),Color(18,184,134,255)],
        _ => [Color(76,110,245,255),Color(34,139,230,255),Color(18,184,134,255)],
    });
    palette
}
pub fn reference_sales_chart(theme:&Theme,key:&'static str,kind:sapid::charts::CartesianKind) -> sapid::charts::Chart {
    fruit_sales_chart(theme,key,kind).data(reference_sales_data(kind)).chart_theme(reference_sales_palette(theme,kind))
}

pub fn schedule_date(value: &str) -> sapid::CivilDate {
    use sapid::prelude::*;
    CivilDate::parse_iso(value).unwrap()
}
pub fn schedule_time(value: &str) -> sapid::LocalDateTime {
    use sapid::prelude::*;
    parse_local_date_time(value).unwrap()
}
pub fn schedule_period(view: sapid::ScheduleView) -> sapid::ScheduleWindow {
    use sapid::prelude::*;
    match view {
        ScheduleView::Day => ScheduleWindow::days(schedule_date("2026-11-04"), 1),
        ScheduleView::Week | ScheduleView::Agenda => {
            ScheduleWindow::days(schedule_date("2026-11-02"), 7)
        }
        ScheduleView::Month | ScheduleView::MobileMonth => {
            ScheduleWindow::month(YearMonth::new(2026, 11).unwrap())
        }
        ScheduleView::Year => ScheduleWindow::year(CivilYear::new(2026).unwrap()),
    }
    .unwrap()
}
pub fn schedule_example(view: sapid::ScheduleView) -> sapid::ScheduleData {
    use sapid::prelude::*;
    ScheduleData::new(
        ScheduleZone::Civil,
        schedule_period(view),
        [
            ScheduleEvent::new(
                ScheduleKey::new("planning").unwrap(),
                "Project planning",
                ScheduleInterval::new(
                    schedule_time("2026-11-04T09:00"),
                    schedule_time("2026-11-04T10:30"),
                )
                .unwrap(),
            )
            .unwrap()
            .color(glory::style::Color(34, 139, 230, 255)),
            ScheduleEvent::new(
                ScheduleKey::new("review").unwrap(),
                "Design review",
                ScheduleInterval::new(
                    schedule_time("2026-11-04T09:30"),
                    schedule_time("2026-11-04T11:00"),
                )
                .unwrap(),
            )
            .unwrap()
            .color(glory::style::Color(121, 80, 242, 255)),
            ScheduleEvent::new(
                ScheduleKey::new("standup").unwrap(),
                "Daily standup",
                ScheduleInterval::new(
                    schedule_time("2026-11-02T11:30"),
                    schedule_time("2026-11-02T12:00"),
                )
                .unwrap(),
            )
            .unwrap()
            .color(glory::style::Color(64, 192, 87, 255))
            .recurring(
                ScheduleRecurrence::new(RecurrenceFrequency::Daily, 1, RecurrenceEnd::Count(7))
                    .unwrap(),
            )
            .unwrap()
            .except_dates([schedule_date("2026-11-05")])
            .unwrap(),
            ScheduleEvent::new(
                ScheduleKey::new("release").unwrap(),
                "Release day",
                ScheduleInterval::all_day(schedule_date("2026-11-04"), schedule_date("2026-11-05"))
                    .unwrap(),
            )
            .unwrap()
            .color(glory::style::Color(250, 82, 82, 255))
            .all_day()
            .unwrap(),
        ],
    )
    .unwrap()
}

pub fn meeting_rooms(view: sapid::ResourceScheduleView, all_day: bool, recurring: bool) -> sapid::ResourceScheduleData {
    use sapid::prelude::*;
    let view = match view { ResourceScheduleView::Day=>ScheduleView::Day, ResourceScheduleView::Week=>ScheduleView::Week, ResourceScheduleView::Month=>ScheduleView::Month };
    let rooms = ScheduleResources::new([("tokyo","Meeting room: Tokyo"), ("paris","Meeting room: Paris"), ("new-york","Meeting room: New York"), ("london","Meeting room: London")].map(|(id,label)|ScheduleResource::new(ScheduleKey::new(id).unwrap(),label).unwrap())).unwrap();
    let mut events = [("Team Standup","tokyo","09:00","09:30", Color(34,139,230,255)),
("Sprint Planning","tokyo","10:00","11:30", Color(64,192,87,255)),
("Client Call","paris","09:30","10:30", Color(121,80,242,255)),
("Design Review","paris","13:00","14:00", Color(253,126,20,255)),
("1:1 Meeting","new-york","11:00","11:30", Color(21,170,191,255)),
("Workshop","new-york","14:00","16:00", Color(230,73,128,255)),
("Architecture Review","london","10:00","11:00", Color(250,82,82,255)),
("Retrospective","london","15:00","16:00", Color(190,75,219,255))].into_iter().enumerate().map(|(i,(title,room,start,end,color))| {
        let interval = ScheduleInterval::new(schedule_time(&format!("2026-11-04T{start}")),schedule_time(&format!("2026-11-04T{end}"))).unwrap();
        let event = ScheduleEvent::new(ScheduleKey::new(format!("booking-{i}")).unwrap(),title,interval).unwrap().color(color).resource(ScheduleKey::new(room).unwrap());
        if recurring && i==0 { event.recurring(ScheduleRecurrence::new(RecurrenceFrequency::Daily,1,RecurrenceEnd::Count(14)).unwrap()).unwrap() } else { event }
    }).collect::<Vec<_>>();
    if all_day {
        events.push(ScheduleEvent::new(ScheduleKey::new("conference").unwrap(),"Conference",ScheduleInterval::all_day(schedule_date("2026-11-04"),schedule_date("2026-11-06")).unwrap()).unwrap().all_day().unwrap().resource(ScheduleKey::new("paris").unwrap()).color(Color(253,126,20,255)));
    }
    ResourceScheduleData::new(ScheduleData::new(ScheduleZone::Civil,schedule_period(view),events).unwrap(),rooms,ScheduleResourceWindow::new(0,4).unwrap()).unwrap()
}

pub fn change_booking_window(data: Cage<sapid::ResourceScheduleData>, request: sapid::ResourceScheduleRequest) {
    use sapid::prelude::*;
    data.revise(|current| {
        *current = ResourceScheduleData::new(ScheduleData::new(request.zone,request.window,current.data().events().iter().cloned()).unwrap(),current.resources().clone(),request.resource_window).unwrap();
    });
}

pub fn authentication(theme: sapid::Theme, interactive: bool) -> impl Widget {
    widget(move |scope| {
        let register = scope.cage(true);
        let accepted = scope.cage(true);
        let email = scope.cage(String::new());
        let password = scope.cage(String::new());
        let confirmation = scope.cage(String::new());
        let result = scope.cage(String::new());
        let first = theme.clone();
        let last = theme.clone();
        let mail_theme = theme.clone();
        let pass_theme = theme.clone();
        let extra_theme = theme.clone();
        let actions_theme = theme.clone();
        let prefix = if interactive {
            "modal-reference"
        } else {
            "loader-reference"
        };
        sapid::stack()
            .theme(theme.clone())
            .class("site-reference-auth")
            .gap(12.)
            .fill((
                when(register, move || {
                    sapid::group()
                        .theme(first.clone())
                        .class("site-reference-auth-names")
                        .fill((
                            sapid::field(format!("{prefix}-first"))
                                .theme(first.clone())
                                .label("First name")
                                .required(true)
                                .control(|field| {
                                    sapid::text_field()
                                        .field(field)
                                        .placeholder("Your first name")
                                }),
                            sapid::field(format!("{prefix}-last"))
                                .theme(last.clone())
                                .label("Last name")
                                .required(true)
                                .control(|field| {
                                    sapid::text_field()
                                        .field(field)
                                        .placeholder("Your last name")
                                }),
                        ))
                }),
                sapid::field(format!("{prefix}-email"))
                    .theme(mail_theme.clone())
                    .label("Email")
                    .required(true)
                    .control(move |field| {
                        sapid::text_field()
                            .field(field)
                            .left_section(text().content("@"))
                            .placeholder("Your email")
                            .value(email)
                            .on_value_change(move |s| email.revise(|v| *v = s))
                    }),
                sapid::field(format!("{prefix}-password"))
                    .theme(pass_theme.clone())
                    .label("Password")
                    .required(true)
                    .control(move |field| {
                        sapid::password_input()
                            .field(field)
                            .placeholder("Password")
                            .value(password)
                            .on_value_change(move |s| password.revise(|v| *v = s))
                    }),
                when(register, move || {
                    sapid::stack().theme(extra_theme.clone()).gap(12.).fill((
                        sapid::field(format!("{prefix}-confirm"))
                            .theme(extra_theme.clone())
                            .label("Confirm Password")
                            .required(true)
                            .control(move |field| {
                                sapid::password_input()
                                    .field(field)
                                    .placeholder("Confirm password")
                                    .value(confirmation)
                                    .on_value_change(move |s| confirmation.revise(|v| *v = s))
                            }),
                        sapid::checkbox()
                            .theme(extra_theme.clone())
                            .checked(accepted)
                            .on_checked_change(move |v| accepted.revise(|o| *o = v))
                            .label("I agree to sell my soul and privacy to this corporation"),
                    ))
                }),
                when(interactive, move || {
                    sapid::group()
                        .theme(actions_theme.clone())
                        .class("site-reference-auth-actions")
                        .fill((
                            sapid::button()
                                .theme(actions_theme.clone())
                                .variant(sapid::ButtonVariant::Subtle)
                                .label(register.map(|r| {
                                    if *r {
                                        "Have an account? Login".to_owned()
                                    } else {
                                        "Don't have an account? Register".to_owned()
                                    }
                                }))
                                .on_press(move || {
                                    register.revise(|v| *v = !*v);
                                    result.revise(|v| v.clear())
                                }),
                            sapid::button()
                                .theme(actions_theme.clone())
                                .label(register.map(|r| {
                                    if *r {
                                        "Register".to_owned()
                                    } else {
                                        "Login".to_owned()
                                    }
                                }))
                                .on_press(move || {
                                    result.revise(|v| {
                                        *v = if !email.get().contains('@') {
                                            "Enter a valid email address."
                                        } else if password.get().is_empty() {
                                            "Enter a password."
                                        } else if register.get()
                                            && (password.get() != confirmation.get()
                                                || !accepted.get())
                                        {
                                            "Check your password confirmation and terms."
                                        } else {
                                            "Details accepted."
                                        }
                                        .to_owned()
                                    })
                                }),
                        ))
                }),
                when(result.map(|v| !v.is_empty()), move || {
                    text().content(result)
                }),
            ))
            .show_in(scope)
    })
}

pub fn sample_timeline_events(week: bool) -> sapid::ScheduleData {
    use sapid::prelude::*;
    let start = CivilDate::new(2026, 11, if week { 16 } else { 22 }).unwrap();
    let mut events = Vec::new();
    for (key, title, day_offset, from, to, color) in [
        ("standup", "Morning Standup", 0, "09:00", "09:30", Color(34,139,230,255)),
        ("meeting", "Team Meeting", 0, "10:00", "11:30", Color(64,192,87,255)),
        ("lunch", "Lunch Break", 0, "12:00", "13:00", Color(253,126,20,255)),
        ("review", "Code Review", 1, "14:00", "15:00", Color(121,80,242,255)),
        ("client", "Client Call", 1, "15:30", "16:30", Color(21,170,191,255)),
        ("planning", "Project Planning", 2, "10:00", "12:00", Color(230,73,128,255)),
    ] {
        if !week && day_offset == 2 {continue;}
        let date = start.checked_add_days(if week {day_offset} else {0}).unwrap();
        let begin = parse_local_date_time(&format!("{date}T{from}")).unwrap();
        let end = parse_local_date_time(&format!("{date}T{to}")).unwrap();
        events.push(ScheduleEvent::new(ScheduleKey::new(key).unwrap(), title, ScheduleInterval::new(begin,end).unwrap()).unwrap().color(color));
    }
    let all_day = ScheduleInterval::all_day(start, start.checked_add_days(1).unwrap()).unwrap();
    events.push(ScheduleEvent::new(ScheduleKey::new("conference").unwrap(), "All Day Conference", all_day).unwrap().color(Color(250,82,82,255)).all_day().unwrap());
    ScheduleData::new(ScheduleZone::Civil,ScheduleWindow::days(start,if week {7} else {1}).unwrap(),events).unwrap()
}
