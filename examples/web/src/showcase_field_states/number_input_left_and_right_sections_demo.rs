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
        .fill((
            sapid::field("number-input-inset-start")
                .theme(theme.clone())
                .label("Your number")
                .control(move |field| {
                    sapid::input_group()
                        .inset_start(
                            sapid::theme_icon(sapid::Icon::Search)
                                .icon_size(16.)
                                .variant(sapid::ThemeIconVariant::Transparent),
                        )
                        .input(
                            sapid::number_input()
                                .placeholder("Your number")
                                .field(field),
                        )
                }),
            sapid::field("number-input-inset-end")
                .theme(theme.clone())
                .label("Your number")
                .control(move |field| {
                    sapid::input_group()
                        .inset_end(
                            sapid::theme_icon(sapid::Icon::Search)
                                .icon_size(16.)
                                .variant(sapid::ThemeIconVariant::Transparent),
                        )
                        .input(
                            sapid::number_input()
                                .placeholder("Your number")
                                .field(field),
                        )
                }),
        ))
}
