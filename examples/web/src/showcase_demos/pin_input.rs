use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let size = scope.cage(1.);
        let length = scope.cage(4.);
        let kind = scope.cage(Some("alphanumeric".to_owned()));
        let mask = scope.cage(false);
        let disabled = scope.cage(false);
        let error = scope.cage(false);
        let placeholder = scope.cage("○".to_owned());
        let code = scope.cage(String::new());
        let status = scope.cage(String::new());
        let palette = theme.clone();
        let preview = each(
            scope.bond(move || vec![(size.get() as usize, length.get() as usize, kind.get())]),
        )
        .key(Clone::clone)
        .children(move |row| {
            let (index, count, kind) = row.value.get();
            let digits = kind.as_deref() == Some("number");
            let accepted = code.map(move |text| {
                text.chars()
                    .filter(|c| !digits || c.is_ascii_digit())
                    .take(count)
                    .collect::<String>()
            });
            sapid::stack().theme(palette.clone()).fill((
                sapid::input_otp()
                    .theme(palette.clone())
                    .size(controls::step_size(index as f64))
                    .name("Verification code")
                    .length(count)
                    .alphabet(if digits {
                        sapid::PinAlphabet::Digits
                    } else {
                        sapid::PinAlphabet::AsciiAlphanumeric
                    })
                    .masked(mask)
                    .invalid(error)
                    .placeholder(placeholder)
                    .enabled(disabled.map(|v| !*v))
                    .value(accepted)
                    .on_value_change(move |next| {
                        code.revise(|v| *v = next);
                        status.revise(String::clear);
                    })
                    .on_complete(move |_| status.revise(|v| *v = "Code ready to verify".to_owned()))
                    .on_reject(move |_| {
                        status.revise(|v| {
                            *v = format!(
                                "Enter {count} {}",
                                if digits {
                                    "digits"
                                } else {
                                    "letters or digits"
                                }
                            )
                        })
                    }),
                sapid::visually_hidden().content(status),
            ))
        });
        view()
            .class("site-configurator site-field-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::field_scale("pin-size".into(), "Size", size, &theme),
                    controls::range("pin-length", "Length", length, 1., 5., 1., &theme),
                    controls::switch("Mask", mask, &theme),
                    controls::field_setting(
                        "pin-placeholder".into(),
                        "Placeholder",
                        placeholder,
                        &theme,
                    ),
                    controls::switch("Disabled", disabled, &theme),
                    controls::switch("Error", error, &theme),
                    controls::select(
                        "pin-kind",
                        "Type",
                        kind,
                        &[("alphanumeric", "Alphanumeric"), ("number", "Number")],
                        &theme,
                    ),
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

    pub fn select(
        key: &str,
        label: &str,
        value: Cage<Option<String>>,
        options: &'static [(&'static str, &'static str)],
        theme: &Theme,
    ) -> Box<dyn Widget> {
        let field_key = format!("control-{key}");
        let select_key = format!("control-select-{key}");
        Box::new(
            sapid::field(field_key)
                .theme(theme.clone())
                .label(label.to_owned())
                .control(move |field| {
                    let mut select = sapid::native_select(select_key.clone()).field(field);
                    for (value, label) in options {
                        select = select.option(sapid::select_option(*value, *label));
                    }
                    select.value(value).on_value_change(move |next| {
                        if next.is_some() {
                            value.revise(|current| *current = next)
                        }
                    })
                }),
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

    pub fn range(
        key: &str,
        label: &str,
        value: Cage<f64>,
        min: f64,
        max: f64,
        step: f64,
        theme: &Theme,
    ) -> Box<dyn Widget> {
        let label = label.to_owned();
        Box::new(
            sapid::stack().theme(theme.clone()).gap(8.).fill((
                sapid::text()
                    .theme(theme.clone())
                    .class("site-control-label")
                    .content(value.map({
                        let label = label.clone();
                        move |v| {
                            let decimals = if step.fract() == 0. {
                                0
                            } else if (step * 10.).fract() == 0. {
                                1
                            } else {
                                2
                            };
                            format!("{label} · {v:.decimals$}")
                        }
                    })),
                sapid::slider(key.to_owned())
                    .theme(theme.clone())
                    .name(label)
                    .min(min)
                    .max(max)
                    .step(step)
                    .value(value)
                    .on_value_change(move |next| value.revise(|v| *v = next)),
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
