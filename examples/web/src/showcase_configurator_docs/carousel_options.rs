use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let current = scope.cage(0usize);
        let orientation = scope.cage("horizontal".to_owned());
        let gap = scope.cage(2.);
        let offset = scope.cage(1.);
        let size = scope.cage(26.);
        let controls = scope.cage(true);
        let indicators = scope.cage(false);
        let palette = theme.clone();
        let preview = each(scope.bond(move || {
            vec![(
                orientation.get(),
                gap.get() as usize,
                offset.get() as usize,
                size.get() as u8,
                controls.get(),
                indicators.get(),
            )]
        }))
        .key(Clone::clone)
        .children(move |entry| {
            let (axis, gap, offset, size, controls, indicators) = entry.value.get();
            let blue = palette.accent;
            sapid::carousel()
                .name("Options slides")
                .theme(palette.clone())
                .count(5usize)
                .viewport_height(200.)
                .slide_fraction(0.7)
                .overscan(2)
                .orientation(if axis == "vertical" {
                    glory::render::Orientation::Vertical
                } else {
                    glory::render::Orientation::Horizontal
                })
                .slide_gap([10., 12., 16., 20., 32.][gap])
                .controls_offset([10., 12., 16., 20., 32.][offset])
                .control_size(f64::from(size))
                .with_controls(controls)
                .with_indicators(indicators)
                .slide_align(Align::Start)
                .current(current)
                .on_current_change(move |next| current.revise(|old| *old = next))
                .render_slide(move |index| {
                    view()
                        .class("site-carousel-number")
                        .style(Style {
                            background: Some(blue),
                            color: Some(glory::style::Color(255, 255, 255, 255)),
                            ..Style::new()
                        })
                        .fill(text().content((index + 1).to_string()))
                })
        });
        view()
            .class("site-configurator site-carousel-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::segmented_choice(
                        "carousel-orientation",
                        "Orientation",
                        orientation,
                        &[("horizontal", "Horizontal"), ("vertical", "Vertical")],
                        &theme,
                    ),
                    controls::field_scale("carousel-gap".into(), "Slide gap", gap, &theme),
                    controls::field_scale(
                        "carousel-offset".into(),
                        "Controls offset",
                        offset,
                        &theme,
                    ),
                    controls::range("carousel-size", "Control size", size, 14., 40., 1., &theme),
                    controls::switch("With controls", controls, &theme),
                    controls::switch("With indicators", indicators, &theme),
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
}
