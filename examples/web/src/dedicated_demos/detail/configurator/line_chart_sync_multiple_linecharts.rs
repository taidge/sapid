use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let kind = CartesianKind::Line;
    widget(move |scope| {
        let selected = scope.cage(None::<ChartSelection>);
        let connected = |key| {
            controls::fruit_sales_chart(&theme, key, kind)
                .selection(selected)
                .pointer_selection(true)
                .on_selection_change(move |v| selected.revise(|old| *old = v))
        };
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                connected("line-chart-sync-top"),
                connected("line-chart-sync-bottom"),
            ))
            .show_in(scope)
    })
}
