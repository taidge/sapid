use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let color = scope.cage("Blue".to_owned());
        let base = theme.clone();

        let palette = scope.bond(move || {
            let mut palette = ChartTheme::from_theme(&base);
            palette.grid = base.disabled;
            palette
                .series
                .fill(controls::accent(base.clone(), &color.get()).accent);
            palette
        });
        let preview = plot("radial-bar-chart-change-color", PlotKind::RadialBar)
            .theme(theme.clone())
            .chart_theme(palette)
            .frame(ChartFrame {
                width: 320.,
                height: 320.,
            })
            .data(PlotData::values(
                [
                    ("unknown", "Unknown", 6.67),
                    ("age-50-up", "50+", 2.63),
                    ("age-40", "40-49", 8.63),
                    ("age-35", "35-39", 8.22),
                    ("age-30", "30-34", 15.69),
                    ("age-25", "25-29", 26.69),
                    ("age-18", "18-24", 31.47),
                ]
                .map(|(key, age, count)| ValueDatum::new(key, age, count)),
            ))
            .legend(false)
            .data_table(false)
            .navigation(false)
            .style(scope.bond(move || {
                Style::new()
                    .width(Length::Px(220.))
                    .max_width(Limit::Percent(100.))
            }));
        view()
            .class("site-configurator site-chart-configurator site-compact-plot site-radial-color-demo")
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::named_color_choices(
                    "radial-bar-chart-change-color-color",
                    "Color",
                    color.into(),
                    move |next| color.revise(|current| *current = next),
                    &theme,
                )],
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
