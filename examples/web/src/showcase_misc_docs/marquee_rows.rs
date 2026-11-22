use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(8.).fill(
        [
            sapid::MarqueeDirection::Left,
            sapid::MarqueeDirection::Right,
        ]
        .into_iter()
        .map(|direction| {
            sapid::marquee()
                .visual_content(|| logo_strip(false, false))
                .theme(theme.clone())
                .text("Sapid logos in eight colors")
                .direction(direction)
                .style(
                    Style::new()
                        .width(Length::Percent(100.))
                        .max_width(Limit::Percent(100.)),
                )
        })
        .collect::<Vec<_>>(),
    )
}
