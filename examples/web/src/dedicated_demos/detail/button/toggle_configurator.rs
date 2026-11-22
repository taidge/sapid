use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let variant = scope.cage("filled".to_owned());
        let color = scope.cage("Blue".to_owned());
        let size = scope.cage(Some("md".to_owned()));
        let radius = scope.cage(8.);
        let auto = scope.cage(false);
        let disabled = scope.cage(false);
        let pressed = scope.cage(false);
        let palette = theme.clone();
        let preview =
            each(scope.bond(move || vec![(variant.get(), color.get(), size.get(), auto.get())]))
                .key(Clone::clone)
                .children(move |r| {
                    let (variant, color, size, auto) = r.value.get();
                    let mut theme = controls::accent(palette.clone(), &color);
                    let ink = theme.accent;
                    if variant == "light" {
                        theme.accent = Color(ink.0, ink.1, ink.2, 32);
                        theme.on_accent = ink;
                    } else if auto {
                        theme.on_accent =
                            theme.contrasting_foreground(ink, Color(255, 255, 255, 255));
                    } else {
                        theme.on_accent = Color(255, 255, 255, 255);
                    }
                    sapid::toggle()
                        .theme(theme)
                        .size(controls::size_value(size.as_deref().unwrap_or("md")))
                        .style(radius.map(|r| Style::new().radius(*r)))
                        .enabled(disabled.map(|v| !*v))
                        .label("B")
                        .name("Bold")
                        .class("site-toggle-bold")
                        .pressed(pressed)
                        .on_pressed_change(move |next| pressed.revise(|v| *v = next))
                });
        view()
            .class("site-configurator site-action-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::segmented_choice(
                        "toggle-variant",
                        "Variant",
                        variant,
                        &[("filled", "Filled"), ("light", "Light")],
                        &theme,
                    ),
                    controls::color_choices(color, &theme),
                    controls::size_slider(size, &theme),
                    controls::pixel_scale(
                        "toggle_configurator-radius",
                        "Radius",
                        radius,
                        [2., 4., 8., 16., 32.],
                        &theme,
                    ),
                    controls::switch("Auto contrast", auto, &theme),
                    controls::switch("Disabled", disabled, &theme),
                ],
            ))
            .show_in(scope)
    })
}
