use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let kind = CartesianKind::Line;
    let mut data = controls::fruit_sales_data(kind);
    for (item, label) in data
        .series
        .iter_mut()
        .zip(["Apple sales", "Orange sales", "Tomato sales"])
    {
        item.label = label.to_owned();
    }
    controls::fruit_sales_chart(&theme, "line-chart-series-labels", kind)
        .data(data)
        .legend(true)
        .legend_above_plot(true)
}
