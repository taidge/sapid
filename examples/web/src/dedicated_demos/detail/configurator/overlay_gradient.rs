use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let visible = scope.cage(true);
        sapid::stack()
            .theme(theme.clone())
            .class("site-reference-overlay")
            .gap(24.)
            .fill((
                sapid::aspect_ratio(16. / 9.)
                    .theme(theme.clone())
                    .style(
                        Style::new()
                            .width(Length::Percent(100.))
                            .max_width(Limit::Px(400.)),
                    )
                    .fill(
                        sapid::overlay()
                            .theme(theme.clone())
                            .visible(visible)
                            .opacity(0.85)
                            .class("site-gradient-mask")
                            .fill(
                                sapid::image(format!(
                                    "{}/overlay-landscape.png",
                                    option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets")
                                ))
                                .alt("Landscape")
                                .width(Length::Percent(100.))
                                .height(Length::Percent(100.))
                                .fit(glory::style::ObjectFit::Cover),
                            ),
                    ),
                sapid::button()
                    .theme(theme.clone())
                    .label("Toggle overlay")
                    .style(
                        Style::new()
                            .width(Length::Percent(100.))
                            .max_width(Limit::Px(200.)),
                    )
                    .on_press(move || visible.revise(|v| *v = !*v)),
            ))
            .show_in(scope)
    })
}
