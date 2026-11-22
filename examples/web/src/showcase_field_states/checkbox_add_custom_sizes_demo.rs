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
            sapid::stack().gap(7.).fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Extra small checkbox")
                    .class("site-choice-input choice-example choice-xxs"),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Extra large checkbox")
                    .class("site-choice-input choice-example choice-xxl"),
            )),
        )
}
