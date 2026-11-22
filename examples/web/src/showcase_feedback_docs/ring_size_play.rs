use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(64_f64);

        let size = scope.cage(120_f64);
        let thickness = scope.cage(12_f64);
        let round = scope.cage(true);
        let palette = theme.clone();
        let preview =
            each(scope.bond(move || {
                vec![(size.get().to_bits(), thickness.get().to_bits(), round.get())]
            }))
            .key(Clone::clone)
            .children(move |_| {
                let theme = palette.clone();

                sapid::ring_progress(value)
                    .theme(theme.clone())
                    .name("Storage usage")
                    .size(size.get())
                    .round_caps(round.get())
                    .thickness(thickness.get())
                    .label(value.map(|v| format!("{v:.0}%")))
                    .value_text(value.map(|v| format!("{v:.0} percent of storage used")))
                    .style(
                        Style {
                            font_size: Some(22.),
                            ..Style::new()
                        }
                        .font_weight(500.),
                    )
            });
        view()
            .class("site-configurator site-display-configurator site-ring-progress-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::range("ring_progress-size", "Size", size, 80., 240., 1., &theme),
                    controls::range(
                        "ring_progress-thickness",
                        "Thickness",
                        thickness,
                        2.,
                        28.,
                        1.,
                        &theme,
                    ),
                    controls::switch("Round caps", round, &theme),
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
}
