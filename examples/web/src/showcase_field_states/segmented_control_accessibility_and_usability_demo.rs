use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .theme(theme.clone())
        .class("site-field-state-example choice-demos-centered")
        .gap(16.)
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .fill(
            sapid::segmented_control("segment-choice-segment-icons")
                .theme(theme.clone())
                .class("choice-segment-icons")
                .default_value("preview")
                .segments([
                    sapid::segment("preview")
                        .label("Preview")
                        .name("Preview")
                        .class("choice-segment-preview"),
                    sapid::segment("code")
                        .label("Code")
                        .name("Code")
                        .class("choice-segment-code"),
                    sapid::segment("export")
                        .label("Export")
                        .name("Export")
                        .class("choice-segment-export"),
                ]),
        )
}
