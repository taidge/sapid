use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let variant = scope.cage("default".to_owned());
        let radius = scope.cage(1.);
        let palette = theme.clone();
        let appearance = scope.bond(move || {
            let mut style = Style::new().radius(controls::step_radius(radius.get()));
            if variant.get() == "filled" {
                style.background = Some(palette.disabled);
                style.border_color = Some(palette.disabled);
            } else if variant.get() == "unstyled" {
                style.background = Some(glory::style::Color(0, 0, 0, 0));
                style.border_color = Some(glory::style::Color(0, 0, 0, 0));
            }
            style
        });
        let preview_theme = theme.clone();
        let preview = sapid::fieldset("profile")
            .theme(preview_theme.clone())
            .legend("Personal information")
            .style(scope.bond(move || {
                appearance
                    .get()
                    .width(Length::Percent(100.))
                    .max_width(Limit::Px(420.))
            }))
            .fill((
                sapid::field("profile-name")
                    .theme(preview_theme.clone())
                    .label("Your name")
                    .control(|field| sapid::text_field().field(field).placeholder("Your name")),
                sapid::field("profile-email")
                    .theme(preview_theme.clone())
                    .label("Email address")
                    .control(|field| {
                        sapid::text_field()
                            .field(field)
                            .placeholder("you@example.com")
                    }),
            ));
        view()
            .class("site-configurator site-field-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::segment_setting(
                        "fieldset-variant",
                        "Variant",
                        variant,
                        &[
                            ("default", "Default"),
                            ("filled", "Filled"),
                            ("unstyled", "Unstyled"),
                        ],
                        &theme,
                    ),
                    controls::field_scale("fieldset-radius".into(), "Radius", radius, &theme),
                ],
            ))
            .show_in(scope)
    })
}
