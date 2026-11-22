use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(vec!["shipping".to_owned()]);
        let variant = scope.cage(Some("default".to_owned()));
        let radius = scope.cage(6.);
        let position = scope.cage("right".to_owned());
        let icon_size = scope.cage(16.);
        let fixed = scope.cage(false);
        let palette = theme.clone();
        let preview = each(scope.bond(move || {
            vec![(
                variant.get(),
                radius.get() as u8,
                position.get(),
                icon_size.get() as u8,
                fixed.get(),
            )]
        }))
        .key(Clone::clone)
        .children(move |_| {
            let mut accordion = sapid::accordion("showcase-faq")
                .theme(palette.clone())
                .open(open)
                .on_open_change(move |next| open.revise(|v| *v = next))
                .item_radius(radius.get())
                .chevron_at_start(position.get() == "left")
                .chevron_size(icon_size.get())
                .rotate_chevron(!fixed.get());
            accordion = match variant.get().as_deref() {
                Some("contained") => accordion.variant(sapid::AccordionVariant::Contained),
                Some("filled") => accordion.variant(sapid::AccordionVariant::Filled),
                Some("separated") => accordion.variant(sapid::AccordionVariant::Separated),
                Some("unstyled") => accordion.unstyled(),
                _ => accordion,
            };
            for (key, question, answer) in [
                (
                    "shipping",
                    "How long does shipping take?",
                    "Orders usually arrive within 3–5 business days.",
                ),
                (
                    "returns",
                    "What is your return policy?",
                    "You can request a return within 30 days.",
                ),
                (
                    "support",
                    "How can I contact support?",
                    "Send a message to the support team from your account.",
                ),
            ] {
                let theme = palette.clone();
                accordion = accordion.item(
                    sapid::accordion_item(key)
                        .label(question)
                        .content(move || sapid::text().theme(theme.clone()).content(answer)),
                );
            }
            accordion
        });
        let side = sapid::stack().theme(theme.clone()).gap(8.).fill((
            sapid::text()
                .theme(theme.clone())
                .class("site-control-label")
                .content("Chevron position"),
            sapid::segmented_control("chevron-position")
                .theme(theme.clone())
                .name("Chevron position")
                .value(position)
                .segments([
                    sapid::segment("left").label("Left"),
                    sapid::segment("right").label("Right"),
                ])
                .on_value_change(move |next| position.revise(|v| *v = next)),
        ));
        view()
            .class("site-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::select(
                        "accordion-variant",
                        "Variant",
                        variant,
                        &[
                            ("default", "Default"),
                            ("contained", "Contained"),
                            ("filled", "Filled"),
                            ("separated", "Separated"),
                            ("unstyled", "Unstyled"),
                        ],
                        &theme,
                    ),
                    controls::radius_slider(radius, &theme),
                    Box::new(side),
                    controls::range(
                        "chevron-size",
                        "Chevron icon size",
                        icon_size,
                        12.,
                        25.,
                        1.,
                        &theme,
                    ),
                    controls::switch("Disable chevron rotation", fixed, &theme),
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
                        move |v| format!("{label} · {v:.0}")
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
    pub fn radius_slider(value: Cage<f64>, theme: &Theme) -> Box<dyn Widget> {
        range("control-radius", "Radius", value, 0., 12., 1., theme)
    }
}
