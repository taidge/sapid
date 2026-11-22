use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let thickness = scope.cage(2.);
        let preview = sapid::burger()
            .theme(theme.clone())
            .size(sapid::Size::ExtraLarge)
            .labels("Show menu", "Hide menu")
            .line_size(thickness);
        let panel = view()
            .class("site-configurator site-action-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::range(
                    "burger-line-size",
                    "Line size",
                    thickness,
                    1.,
                    10.,
                    1.,
                    &theme,
                )],
            ));
        panel.show_in(scope)
    })
}
