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
        .fill((
            sapid::checkbox()
                .theme(theme.clone())
                .label("Default cursor")
                .class("site-choice-input choice-example choice-default-cursor"),
            sapid::checkbox()
                .theme(theme)
                .label("Pointer cursor")
                .class("site-choice-input choice-example choice-pointer"),
        ))
}
