use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .class("site-field-state-example")
        .gap(16.)
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .fill(
            sapid::json_input("json-loading")
                .theme(theme)
                .name("Your JSON")
                .placeholder("Your JSON")
                .rows(4)
                .autosize(true)
                .show_formatter(false)
                .loading(true),
        )
}
