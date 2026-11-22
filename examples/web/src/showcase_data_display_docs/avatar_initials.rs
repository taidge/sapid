use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group()
        .theme(theme.clone())
        .justify(Justify::Center)
        .gap(12.)
        .fill(
            [
                ("John Davis", Color(34, 139, 230, 255)),
                ("Jane Moon", Color(190, 75, 219, 255)),
                ("Alex Lee", Color(132, 181, 49, 255)),
                ("Sam Chen", Color(245, 159, 0, 255)),
                ("Mike Johnson", Color(230, 73, 128, 255)),
                ("Karen King", Color(18, 184, 134, 255)),
                ("Tim Smith", Color(76, 110, 245, 255)),
            ]
            .map(|(name, ink)| {
                let initials = name
                    .split_whitespace()
                    .filter_map(|s| s.chars().next())
                    .collect::<String>();
                sapid::avatar(name)
                    .theme(theme.clone())
                    .fallback(initials)
                    .style(Style {
                        color: Some(ink),
                        background: Some(Color(ink.0, ink.1, ink.2, 26)),
                        ..Style::new()
                    })
            })
            .into_iter()
            .collect::<Vec<_>>(),
        )
}
