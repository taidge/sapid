use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(50_f64);

        let color = scope.cage("Blue".to_owned());
        let radius = scope.cage(8_f64);
        let size = scope.cage(8_f64);
        let striped = scope.cage(false);
        let animated = scope.cage(false);
        let stripes = scope.bond(move || striped.get() || animated.get());
        let palette = theme.clone();
        let preview = each(scope.bond(move || vec![(color.get(), size.get().to_bits())]))
            .key(Clone::clone)
            .children(move |_| {
                let theme = palette.clone();

                sapid::progress(value)
                    .name("Project upload")
                    .theme(controls::accent(theme, &color.get()))
                    .thickness(size.get())
                    .style(radius.map(|r| {
                        Style::new()
                            .width(Length::Percent(100.))
                            .max_width(Limit::Px(400.))
                            .radius(*r)
                    }))
                    .striped(stripes)
                    .animated(animated)
                    .value_text(value.map(|v| format!("{v:.0} percent uploaded")))
            });
        view()
            .class("site-configurator site-display-configurator site-progress-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::color_choices(color, &theme),
                    controls::pixel_scale(
                        "progress-radius",
                        "Radius",
                        radius,
                        [2., 4., 8., 16., 32.],
                        &theme,
                    ),
                    controls::pixel_scale(
                        "progress-size",
                        "Size",
                        size,
                        [3., 5., 8., 12., 16.],
                        &theme,
                    ),
                    controls::range("progress-value", "Value", value, 0., 100., 10., &theme),
                    controls::switch("Striped", striped, &theme),
                    controls::switch("Animated", animated, &theme),
                ],
            ))
            .show_in(scope)
    })
}
