use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let color = scope.cage("Yellow".to_owned());
        let palette = theme.clone();
        let preview = each(scope.bond(move || vec![(color.get(),)]))
            .key(Clone::clone)
            .children(move |_| {
                let theme = palette.clone();
                let tint = controls::mark_color(&theme, &color.get());
                glory::ui::view().fill((
                    sapid::text()
                        .theme(theme.clone())
                        .inline(true)
                        .content("Highlight "),
                    sapid::mark()
                        .theme(theme.clone())
                        .style(Style {
                            background: Some(tint),
                            color: Some(theme.contrasting_foreground(tint, theme.foreground)),
                            ..Style::new()
                        })
                        .content("this chunk"),
                    sapid::text()
                        .theme(theme.clone())
                        .inline(true)
                        .content(" of the text"),
                ))
            });
        view()
            .class("site-configurator site-display-configurator site-mark-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::color_choices(color, &theme)],
            ))
            .show_in(scope)
    })
}
