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
