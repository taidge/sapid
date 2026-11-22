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

#[allow(unused_imports)]
mod controls {
    use glory::prelude::*;
    use glory::style::{Color, FlexWrap};
    use sapid::{Size, Theme};

    pub fn playground(preview: Box<dyn Widget>, controls: Vec<Box<dyn Widget>>) -> Box<dyn Widget> {
        Box::new(
            view()
                .class("site-button-playground")
                .style(
                    Style::new()
                        .display(Display::Flex)
                        .flex_wrap(FlexWrap::Wrap)
                        .gap(24.0),
                )
                .fill((
                    col()
                        .class("site-playground-preview")
                        .style(Style {
                            grow: Some(1.0),
                            align: Some(Align::Center),
                            justify: Some(Justify::Center),
                            ..Style::new()
                                .display(Display::Flex)
                                .flex_direction(FlexDirection::Column)
                                .gap(16.0)
                                .min_width(Length::Px(180.0))
                        })
                        .fill(preview),
                    col()
                        .class("site-playground-controls")
                        .style(
                            Style::new()
                                .display(Display::Flex)
                                .flex_direction(FlexDirection::Column)
                                .gap(16.0)
                                .width(Length::Px(240.0))
                                .max_width(Limit::Percent(100.0)),
                        )
                        .fill(controls),
                )),
        )
    }

    pub fn switch(label: &str, value: Cage<bool>, theme: &Theme) -> Box<dyn Widget> {
        Box::new(
            sapid::switch()
                .theme(theme.clone())
                .size(Size::Small)
                .with_thumb_indicator(true)
                .label(label.to_owned())
                .checked(value)
                .on_checked_change(move |next| value.revise(|current| *current = next)),
        )
    }

    pub fn field_scale(
        key: String,
        label: &'static str,
        step: Cage<f64>,
        theme: &Theme,
    ) -> Box<dyn Widget> {
        const NAMES: [&str; 5] = ["xs", "sm", "md", "lg", "xl"];
        Box::new(
            sapid::stack().theme(theme.clone()).gap(4.).fill((
                sapid::text()
                    .theme(theme.clone())
                    .class("site-control-label")
                    .content(label),
                sapid::slider(key)
                    .theme(theme.clone())
                    .name(label)
                    .min(0.)
                    .max(4.)
                    .step(1.)
                    .marks((0..5).map(|i| sapid::SliderMark::unlabeled(f64::from(i))))
                    .value_text(step.map(|i| Some(NAMES[*i as usize].to_owned())))
                    .value(step)
                    .on_value_change(move |next| step.revise(|value| *value = next)),
            )),
        )
    }

    pub fn field_setting(
        key: String,
        label: &'static str,
        value: Cage<String>,
        theme: &Theme,
    ) -> Box<dyn Widget> {
        Box::new(
            sapid::field(key)
                .theme(theme.clone())
                .label(label)
                .control(move |field| {
                    sapid::text_field()
                        .field(field)
                        .value(value)
                        .placeholder("Enter prop value")
                        .on_value_change(move |next| value.revise(|current| *current = next))
                }),
        )
    }

    pub fn step_size(index: f64) -> Size {
        [
            Size::ExtraSmall,
            Size::Small,
            Size::Medium,
            Size::Large,
            Size::ExtraLarge,
        ][index as usize]
    }
}
