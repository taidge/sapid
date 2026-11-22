use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let from = scope.cage(Some("Blue".to_owned()));
        let to = scope.cage(Some("Grape".to_owned()));
        let angle = scope.cage(45_f64);
        let palette = theme.clone();
        let example = each(scope.bond(move || vec![(from.get(), to.get(), angle.get().to_bits())]))
            .key(Clone::clone)
            .children(move |_| {
                sapid::text()
                    .theme(palette.clone())
                    .size(sapid::Size::ExtraLarge)
                    .weight(800.)
                    .gradient(controls::gradient_style(
                        from.get().as_deref(),
                        to.get().as_deref(),
                        angle.get(),
                    ))
                    .content("Gradient text")
            });
        view()
            .class("site-configurator site-display-configurator")
            .fill(controls::playground(
                Box::new(example),
                vec![controls::gradient_controls(from, to, angle, &theme)],
            ))
            .show_in(scope)
    })
}
