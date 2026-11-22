use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let preview = sapid::badge()
        .theme(theme)
        .left_section(sapid::Icon::Check)
        .content("Verified")
        .right_section("3");
    crate::showcase_style_explorer::explorer(
        "badge-styles",
        Box::new(preview),
        &[
            ("root", "Root element", ".sapid-badge"),
            (
                "section",
                "Left and right sections",
                ".sapid-badge-left, .sapid-badge-right",
            ),
            ("label", "Badge children", ".sapid-badge-content"),
        ],
    )
}
