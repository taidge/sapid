use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let range = HeatmapRange::new(
            CivilDate::parse_iso("2024-02-12").unwrap(),
            CivilDate::parse_iso("2024-04-21").unwrap(),
        )
        .unwrap();
        let counts = [
            0, 0, 0, 0, 3, 3, 0, 4, 4, 0, 4, 3, 0, 0, 0, 0, 2, 2, 1, 2, 1, 0, 0, 1, 0, 0, 3, 0, 1,
            2, 3, 2, 0, 0, 1, 0, 1, 1, 0, 0, 0, 4, 0, 1, 3, 1, 1, 2, 1, 0, 0, 4, 0, 1, 0, 0, 4, 0,
            3, 2, 3, 4, 3, 4, 3, 1, 0, 2, 2, 4,
        ];
        let data = HeatmapData::new(
            counts
                .into_iter()
                .enumerate()
                .map(|(offset, count)| (range.date(offset).unwrap(), f64::from(count))),
        )
        .unwrap();

        let size = scope.cage(10.);
        let radius = scope.cage(2.);
        let gap = scope.cage(1.);
        let palette = theme.clone();
        let sample =
            each(scope.bond(move || vec![(size.get() as u8, radius.get() as u8, gap.get() as u8)]))
                .key(Clone::clone)
                .children(move |entry| {
                    let (size, radius, gap) = entry.value.get();
                    heatmap("heatmap-cell-preview")
                        .theme(palette.clone())
                        .range(range)
                        .week_start(Weekday::Monday)
                        .with_month_labels(true)
                        .with_weekday_labels(true)
                        .weekday_labels(["Sun", "Mon", "", "Wed", "", "Fri", ""].map(str::to_owned))
                        .data(data.clone())
                        .colors(vec![
                            glory::style::Color(233, 236, 239, 255),
                            glory::style::Color(81, 207, 102, 77),
                            glory::style::Color(64, 192, 87, 115),
                            glory::style::Color(55, 178, 77, 191),
                            glory::style::Color(43, 138, 62, 255),
                        ])
                        .cell_size(f64::from(size))
                        .radius(f64::from(radius).min(f64::from(size) / 2.))
                        .gap(f64::from(gap))
                });
        view()
            .class("site-configurator site-chart-configurator site-heatmap-cell-demo")
            .fill(controls::playground(
                Box::new(sample),
                vec![
                    controls::range("rect-size", "Rect size", size, 6., 20., 1., &theme),
                    controls::range("rect-radius", "Rect radius", radius, 0., 20., 1., &theme),
                    controls::range("rect-gap", "Gap", gap, 0., 5., 1., &theme),
                ],
            ))
            .show_in(scope)
    })
}
