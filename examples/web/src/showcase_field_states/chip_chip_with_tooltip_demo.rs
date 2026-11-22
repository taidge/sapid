use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .theme(theme.clone())
        .class("site-field-state-example choice-demos-centered")
        .gap(16.)
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .fill({
            let theme = theme.clone();
            sapid::tooltip("chip-help")
                .theme(theme.clone())
                .text("Chip tooltip")
                .target(move |target| {
                    target.view(
                        view().class("choice-tooltip-target ").fill(
                            sapid::chip()
                                .theme(theme.clone())
                                .label("Chip with tooltip")
                                .default_checked(true),
                        ),
                    )
                })
        })
}
