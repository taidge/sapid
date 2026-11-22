use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let checked = scope.cage(Vec::<String>::new());
        let options = [
            (
                "mantine/core",
                "Core components library: inputs, buttons, overlays, etc.",
            ),
            (
                "mantine/hooks",
                "Collection of reusable hooks for React applications.",
            ),
            ("mantine/notifications", "Notifications system"),
        ];
        let mut choices: Vec<Box<dyn Widget>> = Vec::new();
        for (name, description) in options {
            choices.push(Box::new(
                sapid::checkbox()
                    .theme(theme.clone())
                    .name(format!("{name} {description}"))
                    .class("site-package-choice site-choice-input")
                    .checked(checked.map(move |v| v.iter().any(|s| s == name)))
                    .on_checked_change(move |on| {
                        checked.revise(|v| {
                            v.retain(|s| s != name);
                            if on {
                                v.push(name.into());
                            }
                        })
                    }),
            ));
        }
        let copy: Vec<Box<dyn Widget>> = options
            .into_iter()
            .map(|(name, description)| {
                Box::new(view().class("site-package-copy").fill((
                    text().class("site-package-title").content(name),
                    text().class("site-package-note").content(description),
                ))) as Box<dyn Widget>
            })
            .collect();
        sapid::stack()
            .gap(10.)
            .class("site-field-state-example site-package-group")
            .style(
                Style::new()
                    .width(Length::Percent(100.))
                    .max_width(Limit::Px(320.)),
            )
            .fill((
                text()
                    .class("site-package-label")
                    .content("Pick packages to install"),
                text()
                    .class("site-package-description")
                    .content("Choose all packages that you will need in your application"),
                view().class("site-package-card-frame").fill((
                    view()
                        .role(glory::render::Role::Group)
                        .name("Pick packages to install")
                        .class("site-package-choices")
                        .fill(choices),
                    view().class("site-package-copy-list").fill(copy),
                )),
                text()
                    .class("site-package-current")
                    .content(checked.map(|v| {
                        format!(
                            "CurrentValue: {}",
                            if v.is_empty() {
                                "–".into()
                            } else {
                                v.join(", ")
                            }
                        )
                    })),
            ))
            .show_in(scope)
    })
}
