use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::color_detail(
        theme,
        "color-input-swatches-example",
        true,
        "Swatches per row",
    )
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
        let field_key = format!("swatches-{key}");
        let select_key = format!("swatches-select-{key}");
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
            let preview =
                each(scope.bond(move || {
                    vec![(selected.get(), count.get() as usize, size.get() as usize)]
                }))
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
                        "#25262b", "#868e96", "#fa5252", "#e64980", "#be4bdb", "#7950f2",
                        "#4c6ef5", "#228be6", "#15aabf", "#12b886", "#40c057", "#82c91e",
                        "#fab005", "#fd7e14",
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
                            .plane_size(if prop == "Swatches per row" {
                                240.
                            } else {
                                [180., 200., 240., 280., 320.][index]
                            })
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
                            stack =
                                stack.fill(sapid::text().theme(palette.clone()).content(formatted));
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
}
