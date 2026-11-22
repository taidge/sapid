use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let count = scope.cage(0);
        sapid::group()
            .theme(theme.clone())
            .gap(8.)
            .fill((
                sapid::accordion("composed-disclosure")
                    .theme(theme.clone())
                    .item(
                        sapid::accordion_item("project")
                            .label("Project settings")
                            .content(|| text().content("Workspace access settings")),
                    ),
                sapid::button()
                    .theme(theme.clone())
                    .label("Save")
                    .on_press(move || count.revise(|n| *n += 1)),
                sapid::text()
                    .theme(theme.clone())
                    .content(count.map(|n| format!("Saved {n} times"))),
            ))
            .show_in(scope)
    })
}
