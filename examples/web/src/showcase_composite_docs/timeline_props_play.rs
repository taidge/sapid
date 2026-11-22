use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let color = scope.cage("Blue".to_owned());
        let align = scope.cage(Some("start".to_owned()));
        let reverse = scope.cage(false);
        let opposite = scope.cage(false);
        let active = scope.cage(1.);
        let bullet = scope.cage(25.);
        let line = scope.cage(4.);
        let radius = scope.cage(32.);
        let palette = theme.clone();
        const EVENTS: [(&str, &str, &str, &str); 4] = [
            (
                "branch",
                "New branch",
                "You've created new branch fix-notifications from master",
                "2 hours ago",
            ),
            (
                "commit",
                "Commits",
                "You've pushed 23 commits to fix-notifications branch",
                "52 minutes ago",
            ),
            (
                "request",
                "Pull request",
                "You've submitted a pull request Fix incorrect notification message (#187)",
                "34 minutes ago",
            ),
            (
                "review",
                "Code review",
                "Robert Gluesticker left a code review on your pull request",
                "12 minutes ago",
            ),
        ];
        let progress = active.map(|n| {
            if *n < 0. {
                sapid::TimelineProgress::Pending
            } else {
                sapid::TimelineProgress::Active(EVENTS[*n as usize].0.to_owned())
            }
        });
        let config = scope.bond(move || {
            vec![(
                color.get(),
                align.get(),
                reverse.get(),
                opposite.get(),
                bullet.get() as u8,
                line.get() as u8,
                radius.get() as u8,
            )]
        });
        let preview = each(config).key(Clone::clone).children(move |_| {
            let theme = controls::accent(palette.clone(), &color.get());
            let mut list = sapid::timeline("map-history")
                .theme(theme.clone())
                .name("Repository activity")
                .progress(progress)
                .style(
                    Style::new()
                        .width(Length::Percent(100.))
                        .max_width(Limit::Px(320.)),
                )
                .reverse_active(reverse.get())
                .bullet_size(bullet.get())
                .line_width(line.get())
                .bullet_radius(radius.get())
                .align(match align.get().as_deref() {
                    Some("end") => sapid::TimelineAlign::End,
                    Some("alternate") => sapid::TimelineAlign::Alternate,
                    _ => sapid::TimelineAlign::Start,
                });
            for (i, (key, title, description, time)) in EVENTS.into_iter().enumerate() {
                let mut item = sapid::timeline_item(key)
                    .title(title)
                    .description(description);
                if i == 2 {
                    item = item.line_style(glory::style::BorderStyle::Dashed);
                }
                if opposite.get() {
                    item = item.opposite(
                        sapid::text()
                            .theme(theme.clone())
                            .variant(sapid::TextVariant::Small)
                            .content(time),
                    );
                } else {
                    item = item.timestamp(time);
                }
                list = list.item(item);
            }
            list
        });
        view()
            .class("site-configurator site-display-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::color_choices(color, &theme),
                    controls::range("timeline-active", "Active", active, -1., 3., 1., &theme),
                    controls::switch("Reverse active", reverse, &theme),
                    controls::range("timeline-line", "Line width", line, 1., 6., 1., &theme),
                    controls::range(
                        "timeline-bullet",
                        "Bullet size",
                        bullet,
                        18.,
                        40.,
                        1.,
                        &theme,
                    ),
                    controls::pixel_scale(
                        "timeline-radius",
                        "Radius",
                        radius,
                        [2., 4., 8., 16., 32.],
                        &theme,
                    ),
                    controls::select(
                        "timeline-align",
                        "Align",
                        align,
                        &[
                            ("start", "Start"),
                            ("end", "End"),
                            ("alternate", "Alternate"),
                        ],
                        &theme,
                    ),
                    controls::switch("Opposite timestamps", opposite, &theme),
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

    pub fn select(
        key: &str,
        label: &str,
        value: Cage<Option<String>>,
        options: &'static [(&'static str, &'static str)],
        theme: &Theme,
    ) -> Box<dyn Widget> {
        let field_key = format!("timeline-props-play-{key}");
        let select_key = format!("timeline-props-play-select-{key}");
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
            "timeline-props-play-custom-color",
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
