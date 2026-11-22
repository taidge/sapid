use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let shown = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .align(Align::Center)
            .gap(16.)
            .fill((
                sapid::indicator()
                    .theme(theme.clone())
                    .position(sapid::IndicatorPosition::TopRight)
                    .diameter(12.)
                    .radius(12.)
                    .visible(shown)
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
                sapid::button()
                    .theme(theme.clone())
                    .label("Toggle indicator")
                    .on_press(move || shown.revise(|v| *v = !*v)),
            ))
            .show_in(scope)
    })
}
