use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let variant = scope.cage("light".to_owned());
        let color = scope.cage("Blue".to_owned());
        let palette = theme.clone();
        let preview = each(scope.bond(move || vec![(variant.get(), color.get())]))
            .key(Clone::clone)
            .children(move |entry| {
                let (v, c) = entry.value.get();
                let mut theme = controls::accent(palette.clone(), &c);
                let ink = theme.accent;
                theme.accent_soft = glory::style::Color(ink.0, ink.1, ink.2, 30);
                sapid::nav_link("styled-active-link")
                    .theme(theme)
                    .label("Active link")
                    .href("#navigation")
                    .highlighted(true)
                    .variant(match v.as_str() {
                        "filled" => sapid::NavLinkVariant::Filled,
                        "subtle" => sapid::NavLinkVariant::Subtle,
                        _ => sapid::NavLinkVariant::Light,
                    })
            });
        let panel = view()
            .class("site-configurator site-action-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::color_choices(color, &theme),
                    controls::segmented_choice(
                        "active-link-variant",
                        "Variant",
                        variant,
                        &[
                            ("light", "Light"),
                            ("filled", "Filled"),
                            ("subtle", "Subtle"),
                        ],
                        &theme,
                    ),
                ],
            ));
        panel.show_in(scope)
    })
}
