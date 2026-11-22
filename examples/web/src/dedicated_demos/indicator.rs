use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let color = scope.cage("Blue".to_owned());
        let position = scope.cage(Some("top-right".to_owned()));
        let radius = scope.cage(32_f64);
        let size = scope.cage(10_f64);
        let disabled = scope.cage(false);
        let border = scope.cage(false);
        let processing = scope.cage(false);
        let show_marker = disabled.map(|v| !*v);
        let palette = theme.clone();
        let preview = each(scope.bond(move || {
            vec![(
                color.get(),
                position.get(),
                radius.get().to_bits(),
                size.get().to_bits(),
                border.get(),
            )]
        }))
        .key(Clone::clone)
        .children(move |_| {
            let theme = palette.clone();
            let indicator_position = match position.get().as_deref() {
                Some("top-left") => sapid::IndicatorPosition::TopLeft,
                Some("top-center") => sapid::IndicatorPosition::TopCenter,
                Some("middle-left") => sapid::IndicatorPosition::MiddleLeft,
                Some("middle-center") => sapid::IndicatorPosition::MiddleCenter,
                Some("middle-right") => sapid::IndicatorPosition::MiddleRight,
                Some("bottom-left") => sapid::IndicatorPosition::BottomLeft,
                Some("bottom-center") => sapid::IndicatorPosition::BottomCenter,
                Some("bottom-right") => sapid::IndicatorPosition::BottomRight,
                _ => sapid::IndicatorPosition::TopRight,
            };
            sapid::indicator()
                .theme(theme.clone())
                .diameter(size.get())
                .radius(radius.get())
                .position(indicator_position)
                .color(controls::accent(theme.clone(), &color.get()).accent)
                .with_border(border.get())
                .processing(processing)
                .visible(show_marker)
                .fill(
                    display_sample::portrait(theme, false, false),
                )
        });
        view()
            .class("site-configurator site-display-configurator site-indicator-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::color_choices(color, &theme),
                    controls::select(
                        "indicator-position",
                        "Position",
                        position,
                        &[
                            ("top-left", "top-start"),
                            ("top-center", "top-center"),
                            ("top-right", "top-end"),
                            ("middle-left", "middle-start"),
                            ("middle-center", "middle-center"),
                            ("middle-right", "middle-end"),
                            ("bottom-left", "bottom-start"),
                            ("bottom-center", "bottom-center"),
                            ("bottom-right", "bottom-end"),
                        ],
                        &theme,
                    ),
                    controls::pixel_scale(
                        "indicator-radius",
                        "Radius",
                        radius,
                        [2., 4., 8., 16., 32.],
                        &theme,
                    ),
                    controls::range("indicator-size", "Size", size, 6., 30., 1., &theme),
                    controls::switch("Disabled", disabled, &theme),
                    controls::switch("With border", border, &theme),
                    controls::switch("Processing", processing, &theme),
                ],
            ))
            .show_in(scope)
    })
}
