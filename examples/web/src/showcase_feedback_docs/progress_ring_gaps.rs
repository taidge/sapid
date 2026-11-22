use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let sections = [
        sapid::ProgressSection::new(40., Color(21, 170, 191, 255)),
        sapid::ProgressSection::new(15., Color(253, 126, 20, 255)),
        sapid::ProgressSection::new(15., Color(190, 75, 219, 255)),
    ];
    sapid::stack()
        .theme(theme.clone())
        .align(Align::Center)
        .gap(16.)
        .fill(
            [(0., "No gap (default)"), (5., "5° gap"), (10., "10° gap")]
                .map(|(gap, label)| {
                    sapid::stack()
                        .theme(theme.clone())
                        .align(Align::Center)
                        .gap(4.)
                        .fill((
                            sapid::text().theme(theme.clone()).content(label),
                            sapid::ring_progress(70.)
                                .theme(theme.clone())
                                .size(112.)
                                .thickness(12.)
                                .sections(sections)
                                .section_gap(gap),
                        ))
                })
                .into_iter()
                .collect::<Vec<_>>(),
        )
}
