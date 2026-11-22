use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let transitions = [
        "fade",
        "fade-down",
        "fade-up",
        "fade-left",
        "fade-right",
        "scale",
        "scale-y",
        "scale-x",
        "skew-up",
        "skew-down",
        "rotate-left",
        "rotate-right",
        "slide-down",
        "slide-up",
        "slide-left",
        "slide-right",
        "pop",
        "pop-bottom-left",
        "pop-bottom-right",
        "pop-top-left",
        "pop-top-right",
    ];
    let tips = transitions.map(|name| {
        sapid::tooltip(format!("transition-{name}"))
            .theme(theme.clone())
            .class(format!("site-ref-tip-transition site-ref-tip-{name}"))
            .text(name)
            .target(move |target| {
                target.button(
                    glory::ui::button()
                        .label(name)
                        .class("site-ref-transition-badge"),
                )
            })
    });
    sapid::group()
        .theme(theme)
        .class("site-reference-actions site-ref-transition-badges")
        .fill(Vec::from(tips))
}
