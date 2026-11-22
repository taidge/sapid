use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(64_f64);

        let size = scope.cage(120_f64);
        let thickness = scope.cage(12_f64);
        let round = scope.cage(true);
        let palette = theme.clone();
        let preview =
            each(scope.bond(move || {
                vec![(size.get().to_bits(), thickness.get().to_bits(), round.get())]
            }))
            .key(Clone::clone)
            .children(move |_| {
                let theme = palette.clone();

                sapid::ring_progress(value)
                    .theme(theme.clone())
                    .name("Storage usage")
                    .size(size.get())
                    .round_caps(round.get())
                    .thickness(thickness.get())
                    .label(value.map(|v| format!("{v:.0}%")))
                    .value_text(value.map(|v| format!("{v:.0} percent of storage used")))
                    .style(
                        Style {
                            font_size: Some(22.),
                            ..Style::new()
                        }
                        .font_weight(500.),
                    )
            });
        view()
            .class("site-configurator site-display-configurator site-ring-progress-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::range("ring_progress-size", "Size", size, 80., 240., 1., &theme),
                    controls::range(
                        "ring_progress-thickness",
                        "Thickness",
                        thickness,
                        2.,
                        28.,
                        1.,
                        &theme,
                    ),
                    controls::switch("Round caps", round, &theme),
                ],
            ))
            .show_in(scope)
    })
}
