use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group()
        .theme(theme.clone())
        .justify(Justify::Center)
        .fill(
            sapid::avatar_group("avatar-group-count")
                .theme(theme.clone())
                .max_visible(3)
                .overlap(10.)
                .name("Team members")
                .avatars(
                    [
                        "Alice", "Ben", "Claire", "Dan", "Elise", "Finn", "Greta", "Hugo",
                    ]
                    .into_iter()
                    .take(8)
                    .enumerate()
                    .map(|(i, name)| {
                        let photo = if i % 2 == 0 {
                            "team-member-photo.png"
                        } else {
                            "menu-user-avatar.png"
                        };
                        (
                            name.to_lowercase(),
                            sapid::avatar(name).theme(theme.clone()).source(format!(
                                "{}/{}",
                                option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets"),
                                photo
                            )),
                        )
                    }),
                ),
        )
}
