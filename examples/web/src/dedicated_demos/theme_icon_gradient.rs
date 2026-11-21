use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let from = scope.cage(Some("Blue".to_owned()));
        let to = scope.cage(Some("Grape".to_owned()));
        let angle = scope.cage(45.);
        let palette = theme.clone();
        let icon = sapid::theme_icon(sapid::Icon::Calendar)
            .theme(theme.clone())
            .name("Gradient calendar")
            .size(sapid::Size::ExtraLarge)
            .variant(sapid::ThemeIconVariant::Gradient)
            .color(glory::style::Color(255, 255, 255, 255))
            .style(scope.bond(move || {
                let stop = |value: Option<String>| {
                    controls::accent(palette.clone(), value.as_deref().unwrap_or("Blue")).accent
                };
                Style::new()
                    .radius(10.)
                    .background_image(glory::style::LinearGradient {
                        angle: angle.get(),
                        stops: vec![stop(from.get()), stop(to.get())],
                    })
            }));
        const STOPS: &[(&str, &str)] = &[
            ("Blue", "Blue"),
            ("Grape", "Grape"),
            ("Orange", "Orange"),
            ("Green", "Green"),
            ("Pink", "Pink"),
            ("Teal", "Teal"),
        ];
        view()
            .class("site-configurator")
            .fill(controls::playground(
                Box::new(icon),
                vec![
                    controls::select("gradient-start", "Gradient from", from, STOPS, &theme),
                    controls::select("gradient-end", "Gradient to", to, STOPS, &theme),
                    controls::range(
                        "gradient-angle",
                        "Gradient angle",
                        angle,
                        0.,
                        360.,
                        5.,
                        &theme,
                    ),
                ],
            ))
            .show_in(scope)
    })
}
