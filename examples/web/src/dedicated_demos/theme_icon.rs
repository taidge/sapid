use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let variant = scope.cage(Some("solid".to_owned()));
        let size = scope.cage(Some("md".to_owned()));
        let radius = scope.cage(6.);
        let color = scope.cage("Blue".to_owned());
        let contrast = scope.cage(true);
        let palette = theme.clone();
        let preview = each(
            scope.bond(move || vec![(variant.get(), size.get(), color.get(), contrast.get())]),
        )
        .key(Clone::clone)
        .children(move |_| {
            let (variant, size, name, contrast) =
                (variant.get(), size.get(), color.get(), contrast.get());
            let mut theme = controls::accent(palette.clone(), &name);
            let c = theme.accent;
            theme.accent_soft = glory::style::Color(c.0, c.1, c.2, 32);
            sapid::theme_icon(sapid::Icon::Check)
                .name("Project complete")
                .theme(theme)
                .size(controls::size_value(size.as_deref().unwrap_or("md")))
                .variant(match variant.as_deref() {
                    Some("soft") => sapid::ThemeIconVariant::Soft,
                    Some("outline") => sapid::ThemeIconVariant::Outline,
                    Some("transparent") => sapid::ThemeIconVariant::Transparent,
                    Some("white") => sapid::ThemeIconVariant::White,
                    Some("gradient") => sapid::ThemeIconVariant::Gradient,
                    _ => sapid::ThemeIconVariant::Solid,
                })
                .auto_contrast(contrast)
                .style(radius.map(|r| Style::new().radius(*r)))
        });
        view()
            .class("site-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::select(
                        "icon-variant",
                        "Variant",
                        variant,
                        &[
                            ("solid", "Filled"),
                            ("soft", "Light"),
                            ("outline", "Outline"),
                            ("transparent", "Transparent"),
                            ("white", "White"),
                            ("gradient", "Gradient"),
                        ],
                        &theme,
                    ),
                    controls::radius_slider(radius, &theme),
                    controls::size_slider(size, &theme),
                    controls::color_choices(color, &theme),
                    controls::switch("Auto contrast", contrast, &theme),
                ],
            ))
            .show_in(scope)
    })
}
