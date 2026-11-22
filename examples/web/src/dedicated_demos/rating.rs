use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(2.0_f64);
        let color = scope.cage("Yellow".to_owned());
        let size = scope.cage(2.);
        let count = scope.cage(5.);
        let only = scope.cage(false);
        let palette = theme.clone();
        let rating =
            each(scope.bond(move || vec![(color.get(), size.get() as usize, count.get() as u8)]))
                .key(Clone::clone)
                .children(move |item| {
                    let (color, index, count) = item.value.get();
                    sapid::rating("product-rating")
                        .theme(controls::accent(palette.clone(), &color))
                        .size(controls::step_size(index as f64))
                        .name("Product rating")
                        .class("site-rating-example")
                        .count(count)
                        .highlight_selected_only(only)
                        .precision(sapid::RatingPrecision::Half)
                        .value(value.map(move |v| v.min(f64::from(count))))
                        .on_value_change(move |v| value.revise(|current| *current = v))
                });
        let label = scope.bond(move || {
            format!(
                "Rating: {} of {}",
                value.get().min(count.get()),
                count.get()
            )
        });
        let preview = sapid::stack()
            .theme(theme.clone())
            .gap(8.)
            .fill((rating, sapid::visually_hidden().content(label)));
        view()
            .class("site-configurator site-field-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::color_choices(color, &theme),
                    controls::field_scale("rating-size".into(), "Size", size, &theme),
                    controls::range("rating-count", "Count", count, 1., 10., 1., &theme),
                    controls::switch("Highlight selected only", only, &theme),
                ],
            ))
            .show_in(scope)
    })
}
