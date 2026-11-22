use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let variant = scope.cage(Some("light".to_owned()));
        let radius = scope.cage(18_f64);
        let size = scope.cage(Some("md".to_owned()));
        let color = scope.cage("Blue".to_owned());
        let photo = scope.cage(false);
        let palette = theme.clone();
        let preview =
            each(scope.bond(move || vec![(variant.get(), size.get(), color.get(), photo.get())]))
                .key(Clone::clone)
                .children(move |_| {
                    let theme = palette.clone();
                    let tint = controls::accent(theme.clone(), &color.get()).accent;
                    let transparent = Color(0, 0, 0, 0);
                    let (fill, edge, ink) = match variant.get().as_deref() {
                        Some("filled") => (
                            tint,
                            transparent,
                            theme.contrasting_foreground(tint, theme.on_accent),
                        ),
                        Some("outline") => (transparent, tint, tint),
                        Some("transparent") => (transparent, transparent, tint),
                        Some("white") => (Color(255, 255, 255, 255), transparent, tint),
                        Some("default") => (theme.surface, theme.border, theme.foreground),
                        _ => (Color(tint.0, tint.1, tint.2, 32), transparent, tint),
                    };
                    let avatar = sapid::avatar("Ava Patel")
                        .theme(theme.clone())
                        .size(controls::size_value(size.get().as_deref().unwrap_or("md")))
                        .fallback("AP")
                        .style(radius.map(move |r| {
                            Style {
                                background: Some(fill),
                                color: Some(ink),
                                ..Style::new()
                                    .radius(*r)
                                    .border_color(edge)
                                    .border_style(BorderStyle::Solid)
                                    .border_width(Edges::all(1.))
                            }
                        }));
                    if photo.get() {
                        avatar.source(format!(
                            "{}/team-member-photo.png",
                            option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets")
                        ))
                    } else {
                        avatar
                    }
                });
        view()
            .class(
                "site-configurator site-display-configurator site-avatar-variant-play-configurator",
            )
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::select(
                        "avatar_variant_play-variant",
                        "Variant",
                        variant,
                        &[
                            ("filled", "Filled"),
                            ("light", "Light"),
                            ("outline", "Outline"),
                            ("transparent", "Transparent"),
                            ("white", "White"),
                            ("default", "Default"),
                        ],
                        &theme,
                    ),
                    controls::pixel_scale(
                        "avatar_variant_play-radius",
                        "Radius",
                        radius,
                        [2., 4., 8., 16., 32.],
                        &theme,
                    ),
                    controls::size_slider(size, &theme),
                    controls::color_choices(color, &theme),
                    controls::switch("Src", photo, &theme),
                ],
            ))
            .show_in(scope)
    })
}
