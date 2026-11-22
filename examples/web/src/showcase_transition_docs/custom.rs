use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(false);
        let palette = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .align(Align::Center)
            .gap(8.)
            .fill((
                sapid::button()
                    .theme(theme.clone())
                    .label(open.map(|on| {
                        if *on {
                            "Close dropdown".to_owned()
                        } else {
                            "Open dropdown".to_owned()
                        }
                    }))
                    .on_press(move || open.revise(|v| *v = !*v)),
                sapid::transition()
                    .theme(theme.clone())
                    .mounted(open)
                    .duration(220)
                    .exit_duration(180)
                    .effect(sapid::TransitionEffect::Custom(sapid::TransitionStyles {
                        opacity: 0,
                        scale_x: 1000,
                        scale_y: 0,
                        translate_x: 0,
                        translate_y: -8,
                        origin_y: 0,
                        ..Default::default()
                    }))
                    .content(move || {
                        sapid::paper()
                            .theme(palette.clone())
                            .class("sapid-demo-transition-content")
                            .fill(
                                sapid::text()
                                    .theme(palette.clone())
                                    .content("This dropdown is animated."),
                            )
                    }),
            ))
            .show_in(scope)
    })
}
