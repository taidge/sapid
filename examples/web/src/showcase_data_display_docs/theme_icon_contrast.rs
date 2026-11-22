use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let mut lime = theme.clone();
    lime.accent = Color(169, 227, 75, 255);
    lime.on_accent = Color(255, 255, 255, 255);
    sapid::group()
        .theme(theme.clone())
        .justify(Justify::Center)
        .gap(16.)
        .fill(
            [false, true]
                .map(|auto| {
                    sapid::theme_icon(sapid::Icon::Fingerprint)
                        .theme(lime.clone())
                        .size(sapid::Size::Medium)
                        .style(
                            Style::new()
                                .width(Length::Px(34.))
                                .height(Length::Px(34.))
                                .radius(8.),
                        )
                        .variant(sapid::ThemeIconVariant::Solid)
                        .auto_contrast(auto)
                })
                .into_iter()
                .collect::<Vec<_>>(),
        )
}
