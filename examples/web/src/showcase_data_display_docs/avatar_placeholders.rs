use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group()
        .theme(theme.clone())
        .justify(Justify::Center)
        .gap(16.)
        .fill((
            sapid::avatar("User")
                .theme(theme.clone())
                .fallback_icon(sapid::Icon::User),
            sapid::avatar("Blue user")
                .theme(theme.clone())
                .fallback_icon(sapid::Icon::User)
                .style(Style {
                    background: Some(theme.accent),
                    color: Some(Color(255, 255, 255, 255)),
                    ..Style::new()
                }),
            sapid::avatar("Vernon Ross")
                .theme(theme.clone())
                .fallback("VR")
                .style(Style {
                    background: Some(Color(255, 227, 227, 255)),
                    color: Some(Color(224, 49, 49, 255)),
                    ..Style::new()
                }),
            sapid::avatar("Favorite")
                .theme(theme)
                .fallback_icon(sapid::Icon::Star),
        ))
}
