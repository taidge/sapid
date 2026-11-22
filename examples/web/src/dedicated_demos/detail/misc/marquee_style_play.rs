use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let reverse = scope.cage(false);
        let hover = scope.cage(false);
        let axis = scope.cage(Some("horizontal".to_owned()));
        let repeat = scope.cage(4_f64);
        let duration = scope.cage(40000_f64);
        let gap = scope.cage(16_f64);
        let fade = scope.cage(true);
        let palette = theme.clone();
        let preview = each(scope.bond(move || {
            vec![(
                reverse.get(),
                axis.get(),
                repeat.get().to_bits(),
                gap.get().to_bits(),
            )]
        }))
        .key(Clone::clone)
        .children(move |_| {
            let theme = palette.clone();
            let vertical = axis.get().as_deref() == Some("vertical");
            let direction = match (vertical, reverse.get()) {
                (true, true) => sapid::MarqueeDirection::Down,
                (true, false) => sapid::MarqueeDirection::Up,
                (false, true) => sapid::MarqueeDirection::Right,
                _ => sapid::MarqueeDirection::Left,
            };
            let labels = [
                "Research", "Design", "Build", "Review", "Publish", "Measure", "Learn", "Improve",
            ];
            let message = labels.join(if vertical { "\n\n" } else { "     •     " });
            widget(move |scope| {
                let inside = scope.cage(false);
                sapid::marquee()
                    .theme(theme.clone())
                    .direction(direction)
                    .text(message.clone()).visual_content(move||crate::showcase_misc_docs::marquee_logos::logo_strip(vertical,true))
                    .period_millis(duration.map(|v| *v as u32))
                    .gap(gap.get())
                    .repeat(repeat.get() as usize)
                    .class(fade.map(move |v| {
                        if !*v {
                            ""
                        } else if vertical {
                            "sapid-marquee-fade-vertical"
                        } else {
                            "sapid-marquee-fade-horizontal"
                        }
                        .to_owned()
                    }))
                    .paused(scope.bond(move || hover.get() && inside.get()))
                    .on(glory::render::EventKind::PointerWithin, move |event| {
                        if let glory::render::Event::PointerWithin { phase, .. } = event {
                            inside.revise(|v| *v = phase != glory::render::PointerPhase::Leave);
                        }
                    })
                    .style(
                        Style::new()
                            .width(Length::Percent(100.))
                            .max_width(Limit::Px(400.))
                            .height(if vertical {
                                Length::Px(120.)
                            } else {
                                Length::Auto
                            }),
                    )
                    .show_in(scope)
            })
        });
        view()
            .class(
                "site-configurator site-display-configurator site-marquee-style-play-configurator",
            )
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::switch("Reverse", reverse, &theme),
                    controls::switch("Pause on hover", hover, &theme),
                    controls::option_segments(
                        "marquee_style_play-axis",
                        "Orientation",
                        axis,
                        &[("horizontal", "Horizontal"), ("vertical", "Vertical")],
                        &theme,
                    ),
                    controls::range(
                        "marquee_style_play-repeat",
                        "Repeat",
                        repeat,
                        1.,
                        10.,
                        1.,
                        &theme,
                    ),
                    controls::range(
                        "marquee_style_play-duration",
                        "Duration",
                        duration,
                        5000.,
                        60000.,
                        1000.,
                        &theme,
                    ),
                    controls::range("marquee_style_play-gap", "Gap", gap, 0., 48., 4., &theme),
                    controls::switch("Fade edges", fade, &theme),
                ],
            ))
            .show_in(scope)
    })
}
