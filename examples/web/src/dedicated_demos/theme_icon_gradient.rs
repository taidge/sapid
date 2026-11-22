use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let from = scope.cage(Some("Blue".to_owned()));
        let to = scope.cage(Some("Grape".to_owned()));
        let angle = scope.cage(45.);
        let icon = sapid::theme_icon(sapid::Icon::Calendar)
            .theme(theme.clone())
            .name("Gradient calendar")
            .size(sapid::Size::ExtraLarge)
            .variant(sapid::ThemeIconVariant::Gradient)
            .color(glory::style::Color(255, 255, 255, 255))
            .style(scope.bond(move || {
                Style::new()
                    .radius(10.)
                    .background_image(controls::gradient_style(
                        from.get().as_deref(),
                        to.get().as_deref(),
                        angle.get(),
                    ))
            }));
        view()
            .class("site-configurator site-display-configurator")
            .fill(controls::playground(
                Box::new(icon),
                vec![controls::gradient_controls(from, to, angle, &theme)],
            ))
            .show_in(scope)
    })
}
