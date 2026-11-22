use glory::prelude::*;
use sapid::charts::*;
fn values(values: &[(&str, &str, f64)]) -> PlotData {
    PlotData::Values(
        values
            .iter()
            .map(|(key, label, value)| ValueDatum::new(*key, *label, *value))
            .collect(),
    )
}
fn sample_data() -> PlotData {
    values(&[
        ("speed", "Speed", 80.0),
        ("quality", "Quality", 65.0),
        ("reliability", "Reliability", 90.0),
        ("support", "Support", 72.0),
    ])
}

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let grid = scope.cage(true);
        let angles = scope.cage(true);
        let radius = scope.cage(true);
        let tooltip = scope.cage(false);
        let dots = scope.cage(false);
        let options = scope.bond(move || PlotOptions {
            radar_parts: Some(RadarParts {
                grid: grid.get(),
                angles: angles.get(),
                radius: radius.get(),
                dots: dots.get(),
            }),
            tooltip: tooltip.get(),
            ..Default::default()
        });
        let preview = radar_chart("radar-parts")
            .theme(theme.clone())
            .data(sample_data())
            .frame(ChartFrame {
                width: 320.,
                height: 320.,
            })
            .options(options)
            .legend(false)
            .navigation(false)
            .data_table(false)
            .style(
                Style::new()
                    .width(Length::Px(300.))
                    .max_width(Limit::Percent(100.)),
            );
        let settings = vec![
            controls::switch("With polar grid", grid, &theme),
            controls::switch("With polar angle axis", angles, &theme),
            controls::switch("With polar radius axis", radius, &theme),
            controls::switch("With tooltip", tooltip, &theme),
            controls::switch("With dots", dots, &theme),
        ];
        view()
            .class("site-configurator site-chart-configurator site-compact-plot")
            .fill(controls::playground(Box::new(preview), settings))
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
}
