use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::picker_field_playground(
        theme,
        "year-picker-input-properties",
        "Input label",
        "Input description",
        Some("Pick year"),
        true,
        move |field, style, placeholder| {
            let provider = sapid::DatesProvider::new(sapid::CivilDate::new(2026, 11, 4).unwrap());
            sapid::year_picker_input("example-year-picker-input", provider)
                .field(field)
                .style(style)
                .placeholder(placeholder)
        },
    )
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

    pub fn picker_field_playground<W: Widget>(
        theme: Theme,
        key: &'static str,
        default_label: &'static str,
        default_description: &'static str,
        default_placeholder: Option<&'static str>,
        default_asterisk: bool,
        build: impl Fn(sapid::FieldControl, Bond<Style>, Cage<String>) -> W + Clone + 'static,
    ) -> impl Widget {
        widget(move |scope| {
            let variant = scope.cage("default".to_owned());
            let size = scope.cage(1.);
            let radius = scope.cage(2.);
            let label = scope.cage(default_label.to_owned());
            let description = scope.cage(default_description.to_owned());
            let error = scope.cage(String::new());
            let placeholder = scope.cage(default_placeholder.unwrap_or_default().to_owned());
            let disabled = scope.cage(false);
            let asterisk = scope.cage(default_asterisk);
            let palette = theme.clone();
            let style = scope.bond(move || {
                let index = size.get() as usize;
                let font_size = [12., 14., 16., 18., 20.][index];
                let transparent = Color(0, 0, 0, 0);
                let mut appearance = Style {
                    font_size: Some(font_size),
                    line_height: Some(font_size * 1.4),
                    ..Style::new()
                        .width(Length::Percent(100.))
                        .min_width(Length::Px(0.))
                        .min_height(Length::Px([30., 36., 42., 50., 60.][index]))
                        .radius([2., 4., 8., 16., 32.][radius.get() as usize])
                };
                match variant.get().as_str() {
                    "filled" => {
                        appearance.background = Some(if palette.disabled == palette.input {
                            palette.border
                        } else {
                            palette.disabled
                        });
                        appearance.border_color = Some(transparent);
                    }
                    "unstyled" => {
                        appearance.background = Some(transparent);
                        appearance.border_color = Some(transparent);
                    }
                    _ => {}
                }
                if !error.get().is_empty() {
                    appearance.border_color = Some(palette.danger);
                }
                appearance
            });
            let build = build.clone();
            let preview = sapid::field(key)
                .theme(theme.clone())
                .enabled(disabled.map(|flag| !*flag))
                .label(label)
                .description(description)
                .error(error)
                .with_asterisk(asterisk)
                .style(
                    Style::new()
                        .width(Length::Percent(100.))
                        .max_width(Limit::Px(340.))
                        .gap(2.),
                )
                .control(move |field| build(field, style, placeholder));
            let variant_control = sapid::segmented_control(format!("{key}-variant"))
                .theme(theme.clone())
                .label("Variant")
                .class("site-variant-control")
                .size(Size::Small)
                .full_width(true)
                .value(variant)
                .segments([
                    sapid::segment("default").label("Default"),
                    sapid::segment("filled").label("Filled"),
                    sapid::segment("unstyled").label("Unstyled"),
                ])
                .on_value_change(move |next| variant.revise(|current| *current = next));
            let mut settings: Vec<Box<dyn Widget>> = vec![
                Box::new(variant_control),
                field_scale(format!("{key}-size"), "Size", size, &theme),
                field_scale(format!("{key}-radius"), "Radius", radius, &theme),
                field_setting(format!("{key}-label"), "Label", label, &theme),
                switch("With asterisk", asterisk, &theme),
                field_setting(
                    format!("{key}-description"),
                    "Description",
                    description,
                    &theme,
                ),
                field_setting(format!("{key}-error"), "Error", error, &theme),
            ];
            if default_placeholder.is_some() {
                settings.insert(
                    0,
                    field_setting(
                        format!("{key}-placeholder"),
                        "Placeholder",
                        placeholder,
                        &theme,
                    ),
                );
                settings.push(switch("Disabled", disabled, &theme));
            }
            view()
                .class("site-configurator site-field-configurator")
                .fill(playground(Box::new(preview), settings))
                .show_in(scope)
        })
    }
}
