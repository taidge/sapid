use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let axis = scope.cage(Some("horizontal".to_owned()));
        let variant = scope.cage(Some("filled".to_owned()));
        let color = scope.cage("Blue".to_owned());
        let contrast = scope.cage(false);
        let size = scope.cage(Some("md".to_owned()));
        let radius = scope.cage(8_f64);
        let border = scope.cage(true);
        let palette = theme.clone();
        let preview = each(scope.bond(move || {
            vec![(
                axis.get(),
                variant.get(),
                color.get(),
                contrast.get(),
                size.get(),
            )]
        }))
        .key(Clone::clone)
        .children(move |_| {
            let theme = palette.clone();
            let theme = controls::accent(theme, &color.get());
            let size = controls::size_value(size.get().as_deref().unwrap_or("md"));
            let tone = if variant.get().as_deref() == Some("light") {
                sapid::ButtonVariant::Light
            } else {
                sapid::ButtonVariant::Primary
            };
            let edge = theme.border;

            widget(move |scope| {
                let alignment = scope.cage("Align left");
                let format =
                    [("Bold", "𝐁"), ("Italic", "𝐼"), ("Underline", "U̲")].map(|(name, glyph)| {
                        let value = scope.cage(false);
                        sapid::button()
                            .name(name)
                            .label(glyph)
                            .variant(tone)
                            .auto_contrast(contrast.get())
                            .pressed(value)
                            .style(radius.map(|r| Style::new().radius(*r)))
                            .on_press(move || value.revise(|v| *v = !*v))
                    });
                let align = [
                    ("Align left", "⇤"),
                    ("Align center", "≡"),
                    ("Align right", "⇥"),
                ]
                .map(|(name, glyph)| {
                    sapid::button()
                        .name(name)
                        .label(glyph)
                        .variant(tone)
                        .auto_contrast(contrast.get())
                        .pressed(alignment.map(move |v| *v == name))
                        .style(radius.map(|r| Style::new().radius(*r)))
                        .on_press(move || alignment.revise(|v| *v = name))
                });
                sapid::toolbar("configurable-toolbar")
                    .name("Text formatting options")
                    .theme(theme.clone())
                    .size(size)
                    .orientation(if axis.get().as_deref() == Some("vertical") {
                        glory::render::Orientation::Vertical
                    } else {
                        glory::render::Orientation::Horizontal
                    })
                    .style(scope.bond(move || {
                        Style::new()
                            .radius(radius.get())
                            .border_width(Edges::all(if border.get() { 1. } else { 0. }))
                            .border_style(BorderStyle::Solid)
                            .border_color(edge)
                    }))
                    .group(sapid::toolbar_group("format").buttons(format))
                    .group(sapid::toolbar_group("alignment").buttons(align))
                    .show_in(scope)
            })
        });
        view()
            .class(
                "site-configurator site-display-configurator site-toolbar-style-play-configurator",
            )
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::option_segments(
                        "toolbar_style_play-axis",
                        "Orientation",
                        axis,
                        &[("horizontal", "Horizontal"), ("vertical", "Vertical")],
                        &theme,
                    ),
                    controls::option_segments(
                        "toolbar_style_play-variant",
                        "Variant",
                        variant,
                        &[("filled", "Filled"), ("light", "Light")],
                        &theme,
                    ),
                    controls::color_choices(color, &theme),
                    controls::switch("Auto contrast", contrast, &theme),
                    controls::size_slider(size, &theme),
                    controls::pixel_scale(
                        "toolbar_style_play-radius",
                        "Radius",
                        radius,
                        [2., 4., 8., 16., 32.],
                        &theme,
                    ),
                    controls::switch("With border", border, &theme),
                ],
            ))
            .show_in(scope)
    })
}
