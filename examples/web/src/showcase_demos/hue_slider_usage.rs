use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let hue = scope.cage(250.);
        sapid::stack()
            .theme(theme.clone())
            .class("site-channel-preview")
            .gap(4.)
            .style(
                Style::new()
                    .width(Length::Percent(100.))
                    .max_width(Limit::Px(300.)),
            )
            .fill((
                sapid::text()
                    .theme(theme.clone())
                    .content(hue.map(|v| format!("Hue value: {v:.0}"))),
                sapid::hue_slider("accent-hue")
                    .theme(theme.clone())
                    .name("Hue angle")
                    .value(hue)
                    .style(Style::new().width(Length::Percent(100.)))
                    .on_value_change(move |v| hue.revise(|old| *old = v)),
            ))
            .show_in(scope)
    })
}
