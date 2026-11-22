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
            sapid::popover("phone-popover")
                .theme(theme.clone())
                .label("Open phone input")
                .width(sapid::PopoverWidth::Px(320.))
                .title("Phone number")
                .show_close(false)
                .content(move || {
                    sapid::phone_input("in-popover-phone")
                        .theme(theme.clone())
                        .label("Phone number")
                        .searchable_region(true)
                        .regions([
                            ("US", "United States"),
                            ("GB", "United Kingdom"),
                            ("DE", "Germany"),
                        ])
                }),
        )
}
