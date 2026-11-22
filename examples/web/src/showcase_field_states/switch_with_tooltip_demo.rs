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
            sapid::tooltip("switch-help")
                .theme(theme.clone())
                .text("Switch tooltip")
                .target(move |target| {
                    target.view(
                        view().class("choice-tooltip-target ").fill(
                            sapid::switch()
                                .theme(theme.clone())
                                .name("Switch with tooltip")
                                .size(sapid::Size::Small)
                                .label("Switch with tooltip")
                                .with_thumb_indicator(false)
                                .class(""),
                        ),
                    )
                })
        })
}
