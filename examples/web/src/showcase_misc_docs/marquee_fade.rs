use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(14.).fill(
        [
            ("Default fade (5%)", "sapid-marquee-fade-horizontal"),
            ("Larger fade (15%)", "sapid-demo-marquee-large-fade"),
            ("Custom fade color", "sapid-demo-marquee-color-fade"),
            ("No fade", ""),
        ]
        .map(|(label, class)| {
            sapid::stack().theme(theme.clone()).gap(6.).fill((
                sapid::text()
                    .theme(theme.clone())
                    .size(sapid::Size::Small)
                    .content(label),
                sapid::marquee()
                    .theme(theme.clone())
                    .text("Colored Sapid logos")
                    .visual_content(|| logo_strip(false, false))
                    .class(class)
                    .gap(16.)
                    .repeat(4),
            ))
        })
        .into_iter()
        .collect::<Vec<_>>(),
    )
}
