use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let counts = scope.cage([0_u32; 4]);
        let mut previous: Option<[bool; 4]> = None;
        let colors = [
            theme.accent,
            Color(47, 158, 68, 255),
            Color(230, 119, 0, 255),
            Color(156, 54, 181, 255),
        ];
        let badges = sapid::group()
            .theme(theme.clone())
            .justify(Justify::Center)
            .gap(12.)
            .fill(
                ["Top", "Bottom", "Left", "Right"]
                    .into_iter()
                    .enumerate()
                    .map(|(i, label)| {
                        sapid::badge()
                            .theme(theme.clone())
                            .variant(sapid::BadgeVariant::Solid)
                            .color(colors[i])
                            .content(counts.map(move |n| format!("{label}: {}", n[i])))
                    })
                    .collect::<Vec<_>>(),
            );
        let content = theme.clone();
        let viewport = sapid::scroll_area("boundary-reading")
            .name("Scroll to a boundary")
            .theme(theme.clone())
            .axes(glory::render::ScrollAxes::Both)
            .style(
                Style::new()
                    .width(Length::Px(300.))
                    .max_width(Limit::Percent(100.))
                    .height(Length::Px(200.)),
            )
            .on_metrics_change(move |metrics| {
                if metrics.viewport_width <= 0. || metrics.viewport_height <= 0. {
                    return;
                }
                let edges = [
                    metrics.y <= 0.,
                    metrics.y >= metrics.content_height - metrics.viewport_height - 1.,
                    metrics.x <= 0.,
                    metrics.x >= metrics.content_width - metrics.viewport_width - 1.,
                ];
                if let Some(before) = previous
                    && before != edges
                {
                    counts.revise(|totals| {
                        for i in 0..4 {
                            if edges[i] && !before[i] {
                                totals[i] += 1;
                            }
                        }
                    });
                }
                previous = Some(edges);
            })
            .content(move |_| {
                let lines = (1..=50)
                    .map(|index| {
                        sapid::text().theme(content.clone()).content(format!(
                            "Line {index} - This is a long line that requires horizontal scrolling"
                        ))
                    })
                    .collect::<Vec<_>>();
                sapid::stack()
                    .theme(content.clone())
                    .gap(0.)
                    .style(Style::new().width(Length::Px(600.)))
                    .fill(lines)
            });
        sapid::stack()
            .align(Align::Center)
            .gap(16.)
            .fill((badges, viewport))
            .show_in(scope)
    })
}
