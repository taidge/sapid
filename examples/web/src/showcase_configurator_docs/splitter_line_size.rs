use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let thickness = scope.cage(2.);
        let preview = sapid::splitter("line-splitter")
            .theme(theme.clone())
            .name("Resize example panes")
            .class("site-splitter-colors")
            .grip_size(52., 10.)
            .style(
                Style::new()
                    .width(Length::Percent(100.))
                    .height(Length::Px(200.)),
            )
            .thickness(12.)
            .line_size(thickness)
            .first(|| {
                view()
                    .class("site-splitter-first")
                    .fill(text().content("First pane"))
            })
            .second(|| {
                view()
                    .class("site-splitter-second")
                    .fill(text().content("Second pane"))
            });
        let panel = view()
            .class("site-configurator site-action-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::range(
                    "splitter-line-size",
                    "Line size",
                    thickness,
                    1.,
                    10.,
                    1.,
                    &theme,
                )],
            ));
        panel.show_in(scope)
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
