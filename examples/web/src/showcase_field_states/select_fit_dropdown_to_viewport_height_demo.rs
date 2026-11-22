use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .theme(theme.clone())
        .class("site-field-state-example")
        .gap(16.)
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .fill(
            view().class("site-dd-viewport").fill(
                sapid::field("select-viewport-field")
                    .theme(theme.clone())
                    .label("Your favorite library")
                    .control(move |field| {
                        sapid::select("select-viewport")
                            .placeholder("Pick value")
                            .options((1..=48).map(|index| {
                                sapid::select_option(
                                    format!("option-{index}"),
                                    format!("Option {index}"),
                                )
                            }))
                            .field(field)
                    }),
            ),
        )
}
