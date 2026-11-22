use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let size = scope.cage(1.);
        let value = scope.cage("clearable".to_owned());
        let palette = theme.clone();
        let preview = each(scope.bond(move || vec![size.get() as usize]))
            .key(Clone::clone)
            .children(move |item| {
                let index = item.value.get();
                let size = controls::step_size(index as f64);
                sapid::text_field()
                    .theme(palette.clone())
                    .size(size)
                    .name("Clearable input")
                    .value(value)
                    .on_value_change(move |next| value.revise(|v| *v = next))
                    .right_section(
                        sapid::close_button()
                            .theme(palette.clone())
                            .size(size)
                            .name("Clear input")
                            .enabled(value.map(|v| !v.is_empty()))
                            .on_press(move || value.revise(String::clear)),
                    )
                    .style(
                        Style::new()
                            .width(Length::Percent(100.))
                            .min_height(Length::Px([30., 36., 42., 50., 60.][index]))
                            .max_width(Limit::Px(340.)),
                    )
            });
        view()
            .class("site-configurator site-field-configurator")
            .fill(controls::playground(
                Box::new(
                    view()
                        .style(
                            Style::new()
                                .width(Length::Percent(100.))
                                .max_width(Limit::Px(340.)),
                        )
                        .fill(preview),
                ),
                vec![controls::field_scale(
                    "clear-input-size".into(),
                    "Size",
                    size,
                    &theme,
                )],
            ))
            .show_in(scope)
    })
}
