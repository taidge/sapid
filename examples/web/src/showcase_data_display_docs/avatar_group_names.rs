use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group()
        .theme(theme.clone())
        .justify(Justify::Center)
        .fill(
            view().class("sapid-demo-avatar-tips").fill(
                [
                    ("Alice", "team-member-photo.png"),
                    ("Ben", "menu-user-avatar.png"),
                    ("Claire", "team-member-photo.png"),
                    ("Dan and Elise", ""),
                ]
                .into_iter()
                .enumerate()
                .map(|(index, (name, photo))| {
                    sapid::tooltip(format!("avatar-tip-{index}"))
                        .theme(theme.clone())
                        .text(name)
                        .target(move |target| {
                            let button = glory::ui::button()
                                .name(name)
                                .class("sapid-demo-avatar-tip-trigger");
                            let button = if photo.is_empty() {
                                button.fill(text().content("+2"))
                            } else {
                                button.fill(
                                    glory::ui::image()
                                        .alt(name)
                                        .source(format!(
                                            "{}/{}",
                                            option_env!("SAPID_SHOWCASE_MEDIA")
                                                .unwrap_or("/assets"),
                                            photo
                                        ))
                                        .style(
                                            Style::new()
                                                .width(Length::Px(34.))
                                                .height(Length::Px(34.))
                                                .radius(17.),
                                        ),
                                )
                            };
                            target.button(button)
                        })
                })
                .collect::<Vec<_>>(),
            ),
        )
}
