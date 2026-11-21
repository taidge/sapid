use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let color = scope.cage("Blue".to_owned());
        let align = scope.cage(Some("start".to_owned()));
        let reverse = scope.cage(false);
        let opposite = scope.cage(false);
        let active = scope.cage(1.);
        let bullet = scope.cage(28.);
        let line = scope.cage(2.);
        let radius = scope.cage(12.);
        let palette = theme.clone();
        const EVENTS: [(&str, &str, &str, &str, sapid::Icon); 4] = [
            (
                "branch",
                "Branch created",
                "Mira started a branch for the offline map.",
                "2 hours ago",
                sapid::Icon::ChevronRight,
            ),
            (
                "commit",
                "Changes committed",
                "The new map tiles and route legend are ready.",
                "52 minutes ago",
                sapid::Icon::Circle,
            ),
            (
                "request",
                "Pull request opened",
                "The route is ready for someone else to try.",
                "34 minutes ago",
                sapid::Icon::ArrowRight,
            ),
            (
                "review",
                "Review received",
                "Alex checked the trail labels on a smaller screen.",
                "12 minutes ago",
                sapid::Icon::Check,
            ),
        ];
        let progress =
            active.map(|n| sapid::TimelineProgress::Active(EVENTS[*n as usize].0.to_owned()));
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
                .name("Map release history")
                .progress(progress)
                .style(Style::new().width(Length::Percent(100.)))
                .reverse_active(reverse.get())
                .bullet_size(bullet.get())
                .line_width(line.get())
                .bullet_radius(radius.get())
                .align(match align.get().as_deref() {
                    Some("end") => sapid::TimelineAlign::End,
                    Some("alternate") => sapid::TimelineAlign::Alternate,
                    _ => sapid::TimelineAlign::Start,
                });
            for (i, (key, title, description, time, icon)) in EVENTS.into_iter().enumerate() {
                let ink = theme.clone();
                let reversed = reverse.get();
                let marker = glory::ui::graphic()
                    .drawing(active.map(move |n| {
                        let done = if reversed {
                            i > *n as usize
                        } else {
                            i < *n as usize
                        };
                        icon.drawing(if done {
                            ink.contrasting_foreground(ink.accent, ink.on_accent)
                        } else {
                            ink.foreground
                        })
                    }))
                    .style(
                        Style::new()
                            .width(Length::Px(bullet.get() * 0.54))
                            .height(Length::Px(bullet.get() * 0.54)),
                    );
                let mut item = sapid::timeline_item(key)
                    .title(title)
                    .description(description)
                    .marker(marker);
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
            .class("site-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::color_choices(color, &theme),
                    controls::range(
                        "timeline-active",
                        "Active event",
                        active,
                        0.,
                        3.,
                        1.,
                        &theme,
                    ),
                    controls::switch("Reverse active", reverse, &theme),
                    controls::range("timeline-line", "Line width", line, 0., 8., 1., &theme),
                    controls::range(
                        "timeline-bullet",
                        "Bullet size",
                        bullet,
                        16.,
                        48.,
                        2.,
                        &theme,
                    ),
                    controls::radius_slider(radius, &theme),
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

    pub fn switch(label: &str, value: Cage<bool>, theme: &Theme) -> Box<dyn Widget> {
        Box::new(
            sapid::switch()
                .theme(theme.clone())
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
    pub fn radius_slider(value: Cage<f64>, theme: &Theme) -> Box<dyn Widget> {
        range("control-radius", "Radius", value, 0., 12., 1., theme)
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
        let draft = selected.map(|value| {
            COLORS
                .iter()
                .find(|(name, _)| *name == value)
                .map(|(_, color)| sapid::format_hex_color(*color, false))
                .unwrap_or_else(|| value.clone())
        });
        Box::new(
            sapid::stack().theme(theme.clone()).gap(8.0).fill((
                sapid::text()
                    .theme(theme.clone())
                    .class("site-control-label")
                    .content("Color"),
                sapid::group()
                    .theme(theme.clone())
                    .class("site-control-swatches")
                    .fill(
                        COLORS
                            .iter()
                            .map(|(name, color)| {
                                let active = selected.map(move |current| {
                                    if current == *name {
                                        "site-control-swatch is-active".to_owned()
                                    } else {
                                        "site-control-swatch".to_owned()
                                    }
                                });
                                sapid::button()
                                    .unstyled()
                                    .class(active)
                                    .name(format!("Use {name}"))
                                    .label("")
                                    .style(Style {
                                        background: Some(*color),
                                        ..Style::new()
                                    })
                                    .on_press(move || {
                                        selected.revise(|current| *current = (*name).to_owned())
                                    })
                            })
                            .collect::<Vec<_>>(),
                    ),
                sapid::field("control-custom-color")
                    .theme(theme.clone())
                    .label("Custom color")
                    .control(move |field| {
                        sapid::color_input("custom-accent")
                            .field(field)
                            .value(draft)
                            .on_value_change(move |next| selected.revise(|value| *value = next))
                    }),
            )),
        )
    }
}
