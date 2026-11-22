use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).fill(
        sapid::indicator()
            .theme(theme.clone())
            .diameter(16.)
            .offset(7.)
            .with_border(true)
            .color(Color(250, 82, 82, 255))
            .position(sapid::IndicatorPosition::BottomRight)
            .fill(portrait(theme.clone(), false, true)),
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
