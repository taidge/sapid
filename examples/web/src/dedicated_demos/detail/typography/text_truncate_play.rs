use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let direction = scope.cage(Some("end".to_owned()));
        let preview = {
            let theme = theme.clone();

            sapid::text().theme(theme.clone()).truncate(true).class(direction.map(|v|if v.as_deref()==Some("start"){"sapid-truncate-start".to_owned()}else{"".to_owned()})).style(Style::new().width(Length::Percent(100.)).max_width(Limit::Px(210.))).content("A thoughtful review connects decisions to the people using the project. Give the team time to explore alternatives, test assumptions and clarify the details together.")
        };
        view()
            .class(
                "site-configurator site-display-configurator site-text-truncate-play-configurator",
            )
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::option_segments(
                    "text_truncate_play-direction",
                    "Truncate",
                    direction,
                    &[("start", "Start"), ("end", "End")],
                    &theme,
                )],
            ))
            .show_in(scope)
    })
}
