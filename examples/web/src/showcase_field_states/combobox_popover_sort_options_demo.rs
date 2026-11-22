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
        .fill({
            let mut choices = ["4 – React", "1 – Angular", "3 – Vue", "2 – Svelte"];
            choices.sort_unstable();
            sapid::field("combobox-popover-sorted-field")
                .theme(theme.clone())
                .label("Your favorite library")
                .control(move |field| {
                    sapid::select("combobox-popover-sorted")
                        .placeholder("Pick value")
                        .options(choices.into_iter().enumerate().map(|(index, label)| {
                            sapid::select_option(format!("library-{index}"), label)
                        }))
                        .field(field)
                })
        })
}
