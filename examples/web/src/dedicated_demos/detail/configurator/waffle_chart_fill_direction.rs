use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let direction = scope.cage(Some("left-to-right".to_owned()));
        let options = scope.bond(move || PlotOptions {
            waffle_direction: match direction.get().as_deref() {
                Some("right-to-left") => WaffleDirection::RightToLeft,
                Some("top-to-bottom") => WaffleDirection::TopToBottom,
                Some("bottom-to-top") => WaffleDirection::BottomToTop,
                _ => WaffleDirection::LeftToRight,
            },
            ..Default::default()
        });
        let palette = controls::chart_palette(&theme);
        let preview = plot("waffle-chart-fill-direction", PlotKind::Waffle)
            .theme(theme.clone())
            .chart_theme(palette)
            .options(options)
            .frame(ChartFrame {
                width: 320.,
                height: 320.,
            })
            .data(controls::segment_example_data())
            .legend(false)
            .data_table(false)
            .navigation(false)
            .style(scope.bond(move || {
                Style::new()
                    .width(Length::Px(200.))
                    .max_width(Limit::Percent(100.))
            }));
        view()
            .class("site-configurator site-chart-configurator site-compact-plot")
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::select(
                    "waffle-chart-fill-direction",
                    "Fill direction",
                    direction,
                    &[
                        ("left-to-right", "left-to-right"),
                        ("right-to-left", "right-to-left"),
                        ("top-to-bottom", "top-to-bottom"),
                        ("bottom-to-top", "bottom-to-top"),
                    ],
                    &theme,
                )],
            ))
            .show_in(scope)
    })
}
