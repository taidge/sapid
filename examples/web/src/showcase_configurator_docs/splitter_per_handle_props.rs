use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let locked = scope.cage(true);
        let nested_theme = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Lock first separator")
                    .checked(locked)
                    .on_checked_change(move |v| locked.revise(|s| *s = v)),
                sapid::splitter("reference-outer-split")
                    .theme(theme.clone())
                    .name("Resize first pane")
                    .class("site-splitter-colors")
                    .enabled(locked.map(|v| !*v))
                    .with_grip(locked.map(|v| !*v))
                    .min(15.)
                    .max(70.)
                    .default_value(30.)
                    .style(
                        Style::new()
                            .height(Length::Px(200.))
                            .width(Length::Percent(100.)),
                    )
                    .first(|| {
                        view()
                            .class("site-splitter-first")
                            .fill(text().content("First pane"))
                    })
                    .second(move || {
                        sapid::splitter("reference-inner-split")
                            .theme(nested_theme.clone())
                            .name("Resize second pane")
                            .class("site-splitter-colors")
                            .min(15.)
                            .max(85.)
                            .default_value(57.)
                            .with_grip(true)
                            .style(
                                Style::new()
                                    .height(Length::Percent(100.))
                                    .width(Length::Percent(100.)),
                            )
                            .first(|| {
                                view()
                                    .class("site-splitter-second")
                                    .fill(text().content("Second pane"))
                            })
                            .second(|| {
                                view()
                                    .class("site-splitter-third")
                                    .fill(text().content("Third pane"))
                            })
                    }),
            ))
            .show_in(scope)
    })
}
