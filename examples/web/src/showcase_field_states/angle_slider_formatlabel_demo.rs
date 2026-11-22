use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .theme(theme.clone())
        .class("site-field-state-example")
        .gap(16.)
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .fill(widget(move |scope| {
            let angle = scope.cage(0.);
            let label = angle.map(|v| format!("{v}°"));
            view()
                .class("site-degree-dial")
                .fill((
                    sapid::angle_slider("degree-label")
                        .theme(theme.clone())
                        .name("Angle slider")
                        .value(angle)
                        .value_text(angle.map(|v| Some(format!("{v}°"))))
                        .on_value_change(move |next| angle.revise(|v| *v = next)),
                    text().class("site-degree-value").content(label),
                ))
                .show_in(scope)
        }))
}
