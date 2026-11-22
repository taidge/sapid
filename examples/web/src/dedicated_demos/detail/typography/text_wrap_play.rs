use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let wrap = scope.cage(Some("wrap".to_owned()));
        let preview = {
            let theme = theme.clone();

            sapid::text().theme(theme.clone()).class(wrap.map(|v| if v.as_deref()==Some("balance") {"sapid-text-balanced".to_owned()}else{"".to_owned()})).style(Style::new().max_width(Limit::Px(320.))).content("A thoughtful review connects decisions to the people using the project. Give the team time to explore alternatives, test assumptions and clarify the details together.")
        };
        view()
            .class("site-configurator site-display-configurator site-text-wrap-play-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::option_segments(
                    "text_wrap_play-wrap",
                    "Text wrap",
                    wrap,
                    &[("wrap", "Wrap"), ("balance", "Balance")],
                    &theme,
                )],
            ))
            .show_in(scope)
    })
}
