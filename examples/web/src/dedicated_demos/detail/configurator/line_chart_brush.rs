use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let kind = CartesianKind::Line;
    widget(move |scope| {
        let range = scope.cage([0., 4.]);
        let options = scope.bond(move || {
            let [a, b] = range.get();
            CartesianOptions {
                brush: Some(ChartBrush {
                    first: a as usize,
                    last: b as usize,
                }),
                ..Default::default()
            }
        });
        sapid::stack()
            .theme(theme.clone())
            .gap(8.)
            .fill((
                controls::fruit_sales_chart(&theme, "line-chart-brush-plot", kind).options(options),
                sapid::range_slider("line-chart-brush-range")
                    .theme(theme.clone())
                    .min(0.)
                    .max(4.)
                    .step(1.)
                    .minimum_gap(1.)
                    .labels(["First visible day".into(), "Last visible day".into()])
                    .value(range)
                    .on_value_change(move |v| range.revise(|old| *old = v)),
            ))
            .show_in(scope)
    })
}
