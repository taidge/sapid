use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        use std::hash::{BuildHasher, RandomState};
        let value = scope.cage(500_f64);
        let random = RandomState::new();
        let rows = [(200, "200ms"), (600, "600ms (default)"), (1200, "1200ms")].map(
            |(duration, label)| {
                sapid::stack().theme(theme.clone()).gap(0.).fill((
                    sapid::text()
                        .theme(theme.clone())
                        .font_size(14.)
                        .color(theme.muted)
                        .content(label),
                    sapid::rolling_number()
                        .theme(theme.clone())
                        .value(value)
                        .duration_millis(duration)
                        .style(Style {
                            font_size: Some(28.),
                            ..Style::new()
                        }),
                ))
            },
        );
        let action = sapid::button()
            .theme(theme.clone())
            .label("Random")
            .class("site-rolling-random")
            .on_press(move || {
                let prior = value.get() as u64;
                let next = (prior + 1 + random.hash_one(prior) % 999) % 1000;
                value.revise(|number| *number = next as f64);
            });
        sapid::stack()
            .theme(theme.clone())
            .align(Align::Start)
            .gap(16.)
            .fill((
                sapid::stack()
                    .theme(theme.clone())
                    .gap(16.)
                    .fill(rows.into_iter().collect::<Vec<_>>()),
                action,
            ))
            .show_in(scope)
    })
}
