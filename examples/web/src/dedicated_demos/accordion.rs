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
