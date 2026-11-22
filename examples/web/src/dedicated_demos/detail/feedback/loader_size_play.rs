use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let size = scope.cage(Some("md".to_owned()));
        let palette = theme.clone();
        let preview = each(scope.bond(move || vec![(size.get(),)]))
            .key(Clone::clone)
            .children(move |_| {
                let theme = palette.clone();

                sapid::loader()
                    .theme(theme.clone())
                    .size(controls::size_value(size.get().as_deref().unwrap_or("md")))
                    .name("Loading preview")
            });
        view()
            .class("site-configurator site-display-configurator site-loader-size-play-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::size_slider(size, &theme)],
            ))
            .show_in(scope)
    })
}
