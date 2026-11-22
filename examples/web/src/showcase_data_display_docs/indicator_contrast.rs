use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group()
        .theme(theme.clone())
        .justify(Justify::Center)
        .gap(16.)
        .fill(
            [false, true, false, true]
                .into_iter()
                .enumerate()
                .map(|(i, value)| {
                    sapid::indicator()
                        .theme(theme.clone())
                        .count(99)
                        .color(if i < 2 {
                            Color(192, 235, 117, 255)
                        } else {
                            Color(11, 114, 133, 255)
                        })
                        .auto_contrast(value)
                        .diameter(16.)
                        .with_border(false)
                        .fill(portrait(theme.clone(), i % 2 == 1, true))
                })
                .collect::<Vec<_>>(),
        )
}

fn portrait(theme: sapid::Theme, alternate: bool, round: bool) -> impl Widget {
    let media = option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets");
    let file = if alternate {
        "team-member-photo.png"
    } else {
        "menu-user-avatar.png"
    };
    sapid::avatar("Portrait")
        .theme(theme)
        .source(format!("{media}/{file}"))
        .shape(if round {
            sapid::AvatarShape::Circle
        } else {
            sapid::AvatarShape::Square
        })
        .style(
            Style::new()
                .width(Length::Px(56.))
                .height(Length::Px(56.))
                .radius(if round { 28. } else { 4. }),
        )
}
