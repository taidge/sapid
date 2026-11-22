use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let size = scope.cage(1.);
        let radius = scope.cage(2.);
        let kind = scope.cage("default".to_owned());
        let disabled = scope.cage(false);
        let error = scope.cage(false);
        let palette = theme.clone();
        let style = scope.bond(move || {
            let index = size.get() as usize;
            let mut style = Style::new()
                .radius(controls::step_radius(radius.get()))
                .min_height(Length::Px([30., 36., 42., 50., 60.][index]));
            style.font_size = Some([12., 14., 16., 18., 20.][index]);
            let transparent = glory::style::Color(0, 0, 0, 0);
            match kind.get().as_str() {
                "filled" => {
                    style.background = Some(if palette.disabled == palette.input {
                        palette.border
                    } else {
                        palette.disabled
                    });
                    style.border_color = Some(transparent);
                }
                "unstyled" => {
                    style.background = Some(transparent);
                    style.border_color = Some(transparent);
                }
                _ => {}
            }
            style
        });
        let preview = sapid::field("input-email")
            .theme(theme.clone())
            .label("Email address")
            .description("We will only use this for important account updates.")
            .required(true)
            .enabled(disabled.map(|v| !v))
            .invalid(error)
            .style(
                Style::new()
                    .width(Length::Percent(100.))
                    .max_width(Limit::Px(340.)),
            )
            .control(move |field| {
                sapid::text_field()
                    .field(field)
                    .style(style)
                    .placeholder("you@example.com")
            });
        view()
            .class("site-configurator site-field-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::segment_setting(
                        "input-variant",
                        "Variant",
                        kind,
                        &[
                            ("default", "Default"),
                            ("filled", "Filled"),
                            ("unstyled", "Unstyled"),
                        ],
                        &theme,
                    ),
                    controls::field_scale("input-size".into(), "Size", size, &theme),
                    controls::field_scale("input-radius".into(), "Radius", radius, &theme),
                    controls::switch("Disabled", disabled, &theme),
                    controls::switch("Error", error, &theme),
                ],
            ))
            .show_in(scope)
    })
}
