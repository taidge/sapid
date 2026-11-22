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
            sapid::popover("tags-input-nested-outer")
                .theme(theme.clone())
                .label("Open popover")
                .title("Choose a value")
                .show_close(false)
                .width(sapid::PopoverWidth::Px(300.))
                .content(move || {
                    let theme = theme.clone();
                    sapid::field("tags-input-nested-field-field")
                        .theme(theme.clone())
                        .label("Pick a value")
                        .control(move |field| {
                            sapid::tags_input("tags-input-nested")
                                .placeholder("Enter tag")
                                .field(field)
                        })
                }),
        )
}
