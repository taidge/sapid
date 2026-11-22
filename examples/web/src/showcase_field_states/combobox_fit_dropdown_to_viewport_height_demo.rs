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
                sapid::field("combo-viewport-field")
                    .theme(theme.clone())
                    .label("Pick value")
                    .control(move |field| {
                        sapid::combobox("combo-viewport")
                            .placeholder("Pick value")
                            .options((0..100).map(|i| {
                                sapid::select_option(format!("value-{i}"), format!("Option {i}"))
                            }))
                            .field(field)
                    }),
            ),
        )
}
