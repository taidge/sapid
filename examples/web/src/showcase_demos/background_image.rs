use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let radius = scope.cage(12_f64);
        let preview = {
            let theme = theme.clone();

            let source = format!(
                "{}/mountain-evening.svg",
                option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets")
            );
            sapid::background_image(source)
        .theme(theme.clone())
        .name("Mountain evening background")
        .width(Length::Percent(100.))
        .height(Length::Px(220.))
        .style(radius.map(|r|Style::new().radius(*r)))
        .fill(
            glory::ui::view()
                .style(Style {
                    background: Some(Color(0, 12, 24, 130)),
                    ..Style::new()
                        .height(Length::Px(220.))
                        .padding(Edges::all(24.))
                })
                .fill(
                    sapid::stack().theme(theme.clone()).gap(12.).fill((
                        sapid::title(3)
                            .theme(theme.clone())
                            .style(Style {
                                color: Some(Color(255, 255, 255, 255)),
                                ..Style::new()
                            })
                            .content("Make room for a new view"),
                        sapid::text()
                            .theme(theme)
                            .style(Style {
                                color: Some(Color(255, 255, 255, 255)),
                                ..Style::new()
                            })
                            .content("A responsive cover image with readable foreground content."),
                    )),
                ),
        )
        };
        view()
            .class("site-configurator site-display-configurator site-background-image-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::pixel_scale(
                    "background_image-radius",
                    "Radius",
                    radius,
                    [2., 4., 8., 16., 32.],
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
}
