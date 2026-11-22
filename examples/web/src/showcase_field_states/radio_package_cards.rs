use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let selected = scope.cage(String::new());
        let opts = [
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
        let mut group = sapid::radio_group("package-radio-choices")
            .theme(theme.clone())
            .name("Pick one package to install")
            .class("site-package-choices")
            .value(selected)
            .on_value_change(move |v| selected.revise(|s| *s = v));
        for (name, description) in opts {
            group = group.radio(
                sapid::radio(name.replace('/', "-"))
                    .name(format!("@{name} {description}"))
                    .class("site-package-choice site-choice-input"),
            );
        }
        let copy: Vec<Box<dyn Widget>> = opts
            .into_iter()
            .map(|(name, description)| {
                Box::new(
                    view().class("site-package-copy").fill((
                        text()
                            .class("site-package-title")
                            .content(format!("@{name}")),
                        text().class("site-package-note").content(description),
                    )),
                ) as Box<dyn Widget>
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
                    .content("Pick one package to install"),
                text()
                    .class("site-package-description")
                    .content("Choose a package that you will need in your application"),
                view()
                    .class("site-package-card-frame")
                    .fill((group, view().class("site-package-copy-list").fill(copy))),
                text()
                    .class("site-package-current")
                    .content(selected.map(|s| {
                        format!(
                            "CurrentValue: {}",
                            if s.is_empty() {
                                "–".into()
                            } else {
                                format!("@{}", s.replacen("-", "/", 1))
                            }
                        )
                    })),
            ))
            .show_in(scope)
    })
}
