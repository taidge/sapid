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
            sapid::input_group()
                .inset_end(sapid::loader().size(sapid::Size::Small))
                .input(
                    sapid::number_input()
                        .theme(theme.clone())
                        .name("Age")
                        .placeholder("Age")
                        .controls(false),
                ),
        )
}
