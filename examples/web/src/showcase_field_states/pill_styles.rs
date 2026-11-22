use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let preview = sapid::pill()
        .theme(theme)
        .content("Test pill")
        .removable("Remove Test pill");
    crate::showcase_style_explorer::explorer(
        "pill-styles",
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
            ("root", "Root element", ".sapid-pill,.sapid-tag"),
            ("label", "Pill label (children)", ".sapid-pill-label"),
            ("remove", "Remove button", ".sapid-pill-remove"),
        ],
    )
}
