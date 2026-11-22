use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group()
        .theme(theme.clone())
        .class("site-reference-actions")
        .fill(Vec::from(["always", "hover", "never", "not-hover"].map(
            |mode| {
                sapid::anchor()
                    .theme(theme.clone())
                    .href("?widget=anchor#usage")
                    .open_in(glory::render::LinkOpen::NewContext)
                    .label(format!("Underline {mode}"))
                    .class(format!("site-link-underline-{mode}"))
            },
        )))
}
