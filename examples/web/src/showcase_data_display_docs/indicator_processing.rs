use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group()
        .theme(theme.clone())
        .justify(Justify::Center)
        .fill(
            sapid::indicator()
                .theme(theme.clone())
                .position(sapid::IndicatorPosition::TopRight)
                .diameter(12.)
                .radius(12.)
                .color(Color(250, 82, 82, 255))
                .processing(true)
                .fill(
                    sapid::avatar("Portrait")
                        .theme(theme.clone())
                        .shape(sapid::AvatarShape::Square)
                        .source(format!(
                            "{}/menu-user-avatar.png",
                            option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets")
                        ))
                        .style(
                            Style::new()
                                .width(Length::Px(56.))
                                .height(Length::Px(56.))
                                .radius(4.),
                        ),
                ),
        )
}
