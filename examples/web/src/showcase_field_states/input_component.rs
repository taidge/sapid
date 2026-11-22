use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .gap(16.)
        .class("site-polymorphic-inputs")
        .fill((
            sapid::button()
                .theme(theme.clone())
                .class("site-input-button")
                .label("Button input"),
            sapid::native_select("input-element-select")
                .theme(theme.clone())
                .right_section(
                    glory::ui::graphic()
                        .drawing(sapid::Icon::ChevronDown.drawing(theme.muted))
                        .style(Style::new().width(Length::Px(14.)).height(Length::Px(14.))),
                )
                .name("Select a number")
                .options(["1", "2"].map(|s| sapid::select_option(s, s)))
                .default_value(Some("1".into())),
        ))
}
