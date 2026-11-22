use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::button()
        .theme(theme)
        .class("site-field-state-example site-input-button site-placeholder-button")
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .label("Placeholder content")
}
