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
        .fill(
            sapid::switch()
                .theme(theme.clone())
                .name("Switch with thumb icon")
                .size(sapid::Size::Medium)
                .label("Switch with thumb icon")
                .with_thumb_indicator(false)
                .class("choice-switch-thumb"),
        )
}
