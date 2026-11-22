use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let preview = sapid::segmented_control("styles-framework")
        .theme(theme)
        .name("Framework")
        .default_value("React")
        .segments(["React", "Angular", "Vue"].map(|v| sapid::segment(v).label(v)));
    crate::showcase_style_explorer::explorer(
        "segmented-control-styles",
        Box::new(
            sapid::stack()
                .class("site-field-state-example")
                .style(
                    Style::new()
                        .width(Length::Percent(100.))
                        .max_width(Limit::Px(340.)),
                )
                .fill(preview),
        ),
        &[
            ("root", "Root element", ".sapid-segmented-control"),
            (
                "control",
                "Wrapper element for input and label",
                ".sapid-segment",
            ),
            (
                "input",
                "Input element (`input[type=\"radio\"]`), hidden by default",
                "input",
            ),
            ("label", "Label element associated with input", "label"),
            (
                "indicator",
                "Floating indicator that moves between items",
                ".sapid-segment:has(:checked)",
            ),
            (
                "innerLabel",
                "Wrapper of label element children",
                ".sapid-segment",
            ),
        ],
    )
}
