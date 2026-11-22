use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let shadow = scope.cage(true);
        let preview = {
            let theme = theme.clone();

            sapid::color_swatch(Color(255, 255, 255, 179))
                .theme(theme.clone())
                .name("Translucent white swatch")
                .side(28.)
                .round(true)
                .with_shadow(shadow)
        };
        view()
            .class(
                "site-configurator site-display-configurator site-swatch-shadow-play-configurator",
            )
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::switch("With shadow", shadow, &theme)],
            ))
            .show_in(scope)
    })
}
