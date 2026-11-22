use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let variant = scope.cage(Some("filled".to_owned()));
        let color = scope.cage("Blue".to_owned());
        let size = scope.cage(Some("md".to_owned()));
        let radius = scope.cage(12.);
        let from = scope.cage(Some("Blue".to_owned()));
        let to = scope.cage(Some("Grape".to_owned()));
        let angle = scope.cage(45.);
        let gradient = variant.map(|v| v.as_deref() == Some("gradient"));
        let palette = theme.clone();
        let style = scope.bond(move || {
            let mut style = Style::new().radius(radius.get());
            if gradient.get() {
                style = style.background_image(controls::gradient_style(
                    from.get().as_deref(),
                    to.get().as_deref(),
                    angle.get(),
                ));
            }
            style
        });
        let badge = each(scope.bond(move || vec![(variant.get(), color.get(), size.get())]))
            .key(Clone::clone)
            .children(move |_| {
                use sapid::BadgeVariant::*;
                let appearance = match variant.get().as_deref() {
                    Some("light") => Soft,
                    Some("outline") => Outline,
                    Some("dot") => Dot,
                    Some("transparent") => Transparent,
                    Some("default") => Default,
                    Some("white") => White,
                    Some("gradient") => Gradient,
                    _ => Solid,
                };
                sapid::badge()
                    .theme(palette.clone())
                    .content("Verified")
                    .size(controls::size_value(size.get().as_deref().unwrap_or("md")))
                    .color(controls::accent(palette.clone(), &color.get()).accent)
                    .variant(appearance)
                    .style(style)
            });
        let palette = theme.clone();
        let gradient_theme = theme.clone();
        view()
            .class("site-configurator site-display-configurator")
            .fill(controls::playground(
                Box::new(badge),
                vec![
                    controls::select(
                        "badge-variant",
                        "Variant",
                        variant,
                        &[
                            ("filled", "Filled"),
                            ("light", "Light"),
                            ("outline", "Outline"),
                            ("dot", "Dot"),
                            ("transparent", "Transparent"),
                            ("default", "Default"),
                            ("white", "White"),
                            ("gradient", "Gradient"),
                        ],
                        &theme,
                    ),
                    Box::new(when(
                        variant.map(|v| v.as_deref() != Some("gradient")),
                        move || controls::color_choices(color, &palette),
                    )),
                    Box::new(when(gradient, move || {
                        controls::gradient_controls(from, to, angle, &gradient_theme)
                    })),
                    controls::size_slider(size, &theme),
                    controls::radius_slider(radius, &theme),
                ],
            ))
            .show_in(scope)
    })
}
