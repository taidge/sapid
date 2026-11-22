use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let page = scope.cage(1_u64);
        let size = scope.cage(Some("md".to_owned()));
        let radius = scope.cage(8.);
        let color = scope.cage("Blue".to_owned());
        let with_controls = scope.cage(true);
        let with_edges = scope.cage(false);
        let disabled = scope.cage(false);
        let palette = theme.clone();
        let preview = each(scope.bond(move || {
            vec![(
                size.get(),
                radius.get() as u8,
                color.get(),
                with_controls.get(),
                with_edges.get(),
            )]
        }))
        .key(Clone::clone)
        .children(move |entry| {
            let (size, radius, color, with_controls, with_edges) = entry.value.get();
            let mut theme = controls::accent(palette.clone(), &color);
            theme.radius = f64::from(radius);
            sapid::pagination("focused-pages")
                .theme(theme)
                .name("Example pages")
                .page(page)
                .total(Some(10))
                .size(controls::size_value(size.as_deref().unwrap_or("md")))
                .with_controls(with_controls)
                .with_edges(with_edges)
                .enabled(disabled.map(|v| !*v))
                .on_page_change(move |next| page.revise(|current| *current = next))
        });
        view()
            .class("site-configurator site-action-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::color_choices(color, &theme),
                    controls::size_slider(size, &theme),
                    controls::pixel_scale(
                        "pagination-radius",
                        "Radius",
                        radius,
                        [2., 4., 8., 16., 32.],
                        &theme,
                    ),
                    controls::switch("With controls", with_controls, &theme),
                    controls::switch("With edges", with_edges, &theme),
                    controls::switch("Disabled", disabled, &theme),
                ],
            ))
            .show_in(scope)
    })
}
