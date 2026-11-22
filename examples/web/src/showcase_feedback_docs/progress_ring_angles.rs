use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group()
        .theme(theme.clone())
        .justify(Justify::Center)
        .gap(24.)
        .fill(
            [
                (0., "0° (right)"),
                (90., "90° (bottom)"),
                (180., "180° (left)"),
                (270., "270° (top)"),
            ]
            .map(|(angle, label)| {
                sapid::stack()
                    .theme(theme.clone())
                    .align(Align::Center)
                    .gap(4.)
                    .fill((
                        sapid::ring_progress(25.)
                            .theme(theme.clone())
                            .size(80.)
                            .thickness(8.)
                            .sections([sapid::ProgressSection::new(25., Color(18, 184, 134, 255))])
                            .start_angle(angle),
                        sapid::text().theme(theme.clone()).content(label),
                    ))
            })
            .into_iter()
            .collect::<Vec<_>>(),
        )
}
