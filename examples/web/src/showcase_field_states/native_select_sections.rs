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
        .fill((
            sapid::field("select-left-section")
                .theme(theme.clone())
                .label("Left section")
                .control(|field| {
                    sapid::native_select("left-section-select")
                        .field(field)
                        .options(["React", "Angular"].map(|name| sapid::select_option(name, name)))
                        .default_value(Some("React".into()))
                        .left_section(glory::ui::text().content("#"))
                }),
            sapid::field("select-right-section")
                .theme(theme.clone())
                .label("Right section")
                .control(move |field| {
                    sapid::native_select("right-section-select")
                        .field(field)
                        .options(["React", "Angular"].map(|name| sapid::select_option(name, name)))
                        .default_value(Some("React".into()))
                        .right_section(
                            glory::ui::graphic()
                                .drawing(sapid::Icon::ChevronDown.drawing(theme.muted))
                                .style(Style::new().width(Length::Px(16.)).height(Length::Px(16.))),
                        )
                }),
        ))
}
