use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(Some("md".to_owned()));
        let palette = theme.clone();
        let preview = sapid::stack().theme(theme.clone()).gap(0.).fill((
            text().content("First line"),
            sapid::space()
                .theme(theme.clone())
                .style(value.map(move |v| {
                    Style::new().height(Length::Px(
                        palette.spacing(controls::size_value(v.as_deref().unwrap_or("md"))),
                    ))
                })),
            text().content("Second line"),
        ));
        view()
            .class("site-configurator site-layout-configurator site-space-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::scale_slider("space-height", "H", value, &theme)],
            ))
            .show_in(scope)
    })
}
