use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let lime = Color(148, 216, 45, 255);
    sapid::group().theme(theme.clone()).gap(12.).fill(
        [("Default", false), ("Auto contrast", true)]
            .map(|(name, auto)| {
                sapid::badge()
                    .theme(theme.clone())
                    .variant(sapid::BadgeVariant::Solid)
                    .color(lime)
                    .style(Style {
                        color: Some(if auto {
                            theme.contrasting_foreground(lime, theme.on_accent)
                        } else {
                            theme.on_accent
                        }),
                        ..Style::new()
                    })
                    .content(name)
            })
            .into_iter()
            .collect::<Vec<_>>(),
    )
}
