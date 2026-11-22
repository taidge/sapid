use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::choice_playground(theme, "switch")
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
}
