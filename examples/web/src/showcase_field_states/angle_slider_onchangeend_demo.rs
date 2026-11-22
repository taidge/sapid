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
            let committed = scope.cage(0.);
            sapid::stack()
                .gap(12.)
                .fill((
                    sapid::angle_slider("end-angle")
                        .theme(theme.clone())
                        .name("Angle slider")
                        .value(angle)
                        .on_value_change(move |next| angle.revise(|v| *v = next))
                        .on_value_commit(move |next| committed.revise(|v| *v = next)),
                    text().content(angle.map(|v| format!("Current value: {v}°"))),
                    text().content(committed.map(|v| format!("End value: {v}°"))),
                ))
                .show_in(scope)
        }))
}
