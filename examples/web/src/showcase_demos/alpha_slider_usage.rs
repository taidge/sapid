use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let alpha = scope.cage(0.55);
        let color = theme.accent;
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
                    .content(alpha.map(|v| format!("Alpha value: {v:.2}"))),
                sapid::alpha_slider("accent-opacity")
                    .theme(theme.clone())
                    .name("Accent opacity")
                    .color(color)
                    .value(alpha)
                    .style(Style::new().width(Length::Percent(100.)))
                    .on_value_change(move |v| alpha.revise(|old| *old = v)),
            ))
            .show_in(scope)
    })
}
