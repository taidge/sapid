use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let loading = scope.cage(false);
        let visible = scope.cage(true);
        let close = scope.cage(true);
        let border = scope.cage(false);
        let color = scope.cage("Blue".to_owned());
        let radius = scope.cage(8_f64);
        let title = scope.cage("Project saved".to_owned());
        let children =
            scope.cage("The latest project settings are available to your team.".to_owned());
        let edge = theme.border;
        let appearance = scope.bond(move || {
            Style::new()
                .radius(radius.get())
                .max_width(Limit::Px(400.))
                .border_style(BorderStyle::Solid)
                .border_width(Edges::all(if border.get() { 1. } else { 0. }))
                .border_color(edge)
        });
        let palette = theme.clone();
        let preview = each(scope.bond(move || vec![(close.get(), color.get(), title.get())]))
            .key(Clone::clone)
            .children(move |_| {
                let notice = sapid::notification("project-saved")
                    .theme(palette.clone())
                    .optional_content(children)
                    .color(controls::accent(palette.clone(), &color.get()).accent)
                    .style(appearance)
                    .loading(loading)
                    .visible(visible)
                    .closable(close.get())
                    .close_label("Dismiss project notification")
                    .on_close(move || visible.revise(|v| *v = false));
                let notice = if title.get().trim().is_empty() {
                    notice
                } else {
                    notice.title(title.get())
                };
                let theme = palette.clone();
                sapid::stack().theme(palette.clone()).gap(0.).fill((
                    notice,
                    when(visible.map(|v| !*v), move || {
                        sapid::button()
                            .theme(theme.clone())
                            .label("Restore project notification")
                            .on_press(move || visible.revise(|v| *v = true))
                    }),
                ))
            });
        view()
            .class("site-configurator site-display-configurator site-notification-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::switch("Loading", loading, &theme),
                    controls::switch("With close button", close, &theme),
                    controls::switch("With border", border, &theme),
                    controls::color_choices(color, &theme),
                    controls::pixel_scale(
                        "notification-radius",
                        "Radius",
                        radius,
                        [2., 4., 8., 16., 32.],
                        &theme,
                    ),
                    controls::text_field("Title", title, &theme),
                    controls::text_field("Children", children, &theme),
                ],
            ))
            .show_in(scope)
    })
}
