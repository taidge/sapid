use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let label = scope.cage("Input label".to_owned());
        let required = scope.cage(false);
        let description = scope.cage("Input description".to_owned());
        let error = scope.cage("Input error".to_owned());
        let size = scope.cage(1.);
        let text = scope.cage(String::new());
        let palette = theme.clone();
        let preview = each(scope.bond(move || vec![size.get() as usize]))
            .key(Clone::clone)
            .children(move |item| {
                let index = item.value.get();
                sapid::field("input-wrapper-example")
                    .theme(palette.clone())
                    .size(controls::step_size(item.value.get() as f64))
                    .label(label)
                    .description(description)
                    .error(error)
                    .with_asterisk(required)
                    .style(Style::new().width(Length::Percent(100.)))
                    .control(move |field| {
                        sapid::text_field()
                            .field(field)
                            .style(
                                Style::new()
                                    .min_height(Length::Px([30., 36., 42., 50., 60.][index])),
                            )
                            .placeholder("Input inside Input.Wrapper")
                            .value(text)
                            .on_value_change(move |next| text.revise(|v| *v = next))
                    })
            });
        view()
            .class("site-configurator site-field-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::field_setting("wrapper-demo-label".into(), "Label", label, &theme),
                    controls::switch("With asterisk", required, &theme),
                    controls::field_setting(
                        "wrapper-demo-description".into(),
                        "Description",
                        description,
                        &theme,
                    ),
                    controls::field_setting("wrapper-demo-error".into(), "Error", error, &theme),
                    controls::field_scale("wrapper-demo-size".into(), "Size", size, &theme),
                ],
            ))
            .show_in(scope)
    })
}
