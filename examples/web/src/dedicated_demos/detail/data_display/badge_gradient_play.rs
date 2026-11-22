use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let from = scope.cage(Some("Blue".to_owned()));
        let to = scope.cage(Some("Grape".to_owned()));
        let angle = scope.cage(45_f64);
        let example = sapid::badge()
            .theme(theme.clone())
            .variant(sapid::BadgeVariant::Gradient)
            .content("Gradient badge")
            .style(scope.bond(move || {
                Style::new().background_image(controls::gradient_style(
                    from.get().as_deref(),
                    to.get().as_deref(),
                    angle.get(),
                ))
            }));
        view()
            .class("site-configurator site-display-configurator")
            .fill(controls::playground(
                Box::new(example),
                vec![controls::gradient_controls(from, to, angle, &theme)],
            ))
            .show_in(scope)
    })
}
