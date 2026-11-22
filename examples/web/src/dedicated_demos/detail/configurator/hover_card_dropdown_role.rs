use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    view().class("site-ref-tour-centered").fill(
        sapid::hover_card("reference-role-card")
            .theme(theme)
            .tooltip_role(true)
            .content(|| {
                text().content("Attaching interaction handlers to HTML produced by a server.")
            })
            .target(|t| {
                t.button(
                    glory::ui::button()
                        .label("Hydration")
                        .class("site-ref-hydration"),
                )
            }),
    )
}
