use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let animated = scope.cage(true);
        let kind = scope.cage(Some("oval".to_owned()));
        let size = scope.cage(Some("md".to_owned()));
        let color = scope.cage("Blue".to_owned());
        let palette = theme.clone();
        let preview = each(scope.bond(move || vec![(kind.get(), size.get(), color.get())]))
            .key(Clone::clone)
            .children(move |_| {
                let theme = controls::accent(palette.clone(), &color.get());
                sapid::loader()
                    .theme(theme.clone())
                    .name("Loading workspace")
                    .animated(animated)
                    .color(theme.accent)
                    .size(controls::size_value(size.get().as_deref().unwrap_or("md")))
                    .kind(match kind.get().as_deref() {
                        Some("bars") => sapid::LoaderType::Bars,
                        Some("dots") => sapid::LoaderType::Dots,
                        _ => sapid::LoaderType::Oval,
                    })
            });
        view()
            .class("site-configurator site-display-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::color_choices(color, &theme),
                    controls::size_slider(size, &theme),
                    controls::option_segments(
                        "loader-type",
                        "Type",
                        kind,
                        &[("oval", "Oval"), ("bars", "Bars"), ("dots", "Dots")],
                        &theme,
                    ),
                    controls::checkbox("Animate loading indicator", animated, &theme),
                ],
            ))
            .show_in(scope)
    })
}
