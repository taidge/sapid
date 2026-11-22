use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group()
        .theme(theme.clone())
        .class("site-reference-actions")
        .fill(Vec::from([("12", true), ("3", false)].map(
            |(count, enabled)| {
                sapid::button()
                    .theme(theme.clone())
                    .label("Send files")
                    .enabled(enabled)
                    .class("site-reference-send-files")
                    .left_section(text().class("site-reference-send-count").content(count))
                    .right_section(
                        glory::ui::graphic()
                            .drawing(sapid::Icon::ArrowRight.drawing(if enabled {
                                theme.on_accent
                            } else {
                                theme.disabled_text
                            }))
                            .style(Style::new().width(Length::Px(17.)).height(Length::Px(17.))),
                    )
            },
        )))
}
