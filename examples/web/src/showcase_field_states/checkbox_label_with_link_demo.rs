use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .theme(theme.clone())
        .class("site-field-state-example choice-demos-left")
        .gap(16.)
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .fill(
            sapid::group().gap(16.).fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("I accept")
                    .class("site-choice-input choice-example "),
                sapid::anchor()
                    .theme(theme.clone())
                    .href("https://mantine.dev")
                    .label("terms and conditions"),
            )),
        )
}
