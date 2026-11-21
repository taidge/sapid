use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let color = scope.cage("Blue".to_owned());
        let radius = scope.cage(8.);
        let icon_size = scope.cage(36.);
        let palette = theme.clone();
        let preview = each(color.map(|v| vec![v.clone()])).key(Clone::clone).children(move |_| {
            let theme = controls::accent(palette.clone(), &color.get());
            let c = theme.accent;
            sapid::blockquote().theme(theme.clone()).icon(sapid::Icon::Info).icon_size(icon_size)
                .style(radius.map(move |r| Style {background:Some(glory::style::Color(c.0,c.1,c.2,24)),..Style::new().radius(*r).padding(glory::style::Edges::all(28.))}))
                .fill(sapid::text().theme(theme.clone()).content("Good design gives people room to focus on what matters. Leave enough space around the work for the idea to be heard."))
                .citation(sapid::text().theme(theme).variant(sapid::TextVariant::Small).content("— Ava Patel, Mountain studio design notes"))
        });
        view()
            .class("site-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::color_choices(color, &theme),
                    controls::radius_slider(radius, &theme),
                    controls::range(
                        "quote-icon-size",
                        "Icon size",
                        icon_size,
                        16.,
                        64.,
                        2.,
                        &theme,
                    ),
                ],
            ))
            .show_in(scope)
    })
}
