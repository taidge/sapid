use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let count = scope.cage(0);
        let color = scope.cage("Blue".to_owned());
        let size = scope.cage(Some("sm".to_owned()));
        let radius = scope.cage(8.);
        let variant = scope.cage(Some("filled".to_owned()));
        let palette = theme.clone();
        let button = each(scope.bond(move || vec![(color.get(), size.get(), variant.get())]))
            .key(Clone::clone)
            .children(move |entry| {
                let (color, size, variant) = entry.value.get();
                let mut theme = controls::accent(palette.clone(), &color);
                let ink = theme.accent;
                theme.accent_soft = glory::style::Color(ink.0, ink.1, ink.2, 32);
                let style_variant = match variant.as_deref() {
                    Some("default") => sapid::ButtonVariant::Secondary,
                    Some("light") => sapid::ButtonVariant::Light,
                    Some("outline") => sapid::ButtonVariant::Outline,
                    Some("subtle" | "transparent" | "white") => sapid::ButtonVariant::Subtle,
                    _ => sapid::ButtonVariant::Primary,
                };
                let white = variant.as_deref() == Some("white");
                let transparent = variant.as_deref() == Some("transparent");
                sapid::button()
                    .theme(theme)
                    .variant(style_variant)
                    .size(controls::size_value(size.as_deref().unwrap_or("md")))
                    .class(if transparent {
                        "site-action-transparent"
                    } else {
                        ""
                    })
                    .style(radius.map(move |r| {
                        let mut style = Style::new().radius(*r);
                        if white {
                            style.background = Some(glory::style::Color(255, 255, 255, 255));
                        }
                        style
                    }))
                    .name("Filled button")
                    .label("Button")
                    .auto_contrast(false)
                    .on_press(move || count.revise(|v| *v += 1))
            });
        let response_theme = theme.clone();
        let preview = sapid::stack().theme(theme.clone()).gap(12.).fill((
            button,
            when(count.map(|n| *n > 0), move || {
                sapid::text()
                    .theme(response_theme.clone())
                    .content(count.map(|v| format!("Button presses: {v}")))
            }),
        ));
        view()
            .class("site-configurator site-action-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::select(
                        "button-variant",
                        "Variant",
                        variant,
                        &[
                            ("default", "Default"),
                            ("filled", "Filled"),
                            ("light", "Light"),
                            ("outline", "Outline"),
                            ("subtle", "Subtle"),
                            ("transparent", "Transparent"),
                            ("white", "White"),
                        ],
                        &theme,
                    ),
                    controls::color_choices(color, &theme),
                    controls::size_slider(size, &theme),
                    controls::pixel_scale(
                        "button-radius",
                        "Radius",
                        radius,
                        [2., 4., 8., 16., 32.],
                        &theme,
                    ),
                ],
            ))
            .show_in(scope)
    })
}
