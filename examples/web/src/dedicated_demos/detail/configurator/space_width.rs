use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(Some("md".to_owned()));
        let preview = sapid::group().theme(theme.clone()).gap(0.).fill((
            text().content("First part"),
            sapid::space().style(value.map(|v| {
                let width = match controls::size_value(v.as_deref().unwrap_or("md")) {
                    sapid::Size::ExtraSmall => 10.,
                    sapid::Size::Small => 12.,
                    sapid::Size::Medium => 16.,
                    sapid::Size::Large => 20.,
                    sapid::Size::ExtraLarge => 32.,
                };
                Style::new().width(Length::Px(width))
            })),
            text().content("Second part"),
        ));
        view()
            .class("site-configurator site-layout-configurator site-space-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::scale_slider("space-width", "W", value, &theme)],
            ))
            .show_in(scope)
    })
}
