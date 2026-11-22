use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let size = scope.cage(Some("md".to_owned()));
        let lines = scope.cage(2_f64);
        let palette = theme.clone();
        let preview = each(scope.bond(move || vec![(size.get(), lines.get().to_bits())])).key(Clone::clone).children(move |_| { let theme=palette.clone();

sapid::text().theme(theme.clone()).size(controls::size_value(size.get().as_deref().unwrap_or("md"))).line_clamp(lines.get() as u16).style(Style::new().max_width(Limit::Px(320.))).content("A thoughtful review connects decisions to the people using the project. Give the team time to explore alternatives, test assumptions and clarify the details together.")
});
        view()
            .class("site-configurator site-display-configurator site-text-clamp-play-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::size_slider(size, &theme),
                    controls::range(
                        "text_clamp_play-lines",
                        "Line clamp",
                        lines,
                        1.,
                        5.,
                        1.,
                        &theme,
                    ),
                ],
            ))
            .show_in(scope)
    })
}
