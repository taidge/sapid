use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    crate::showcase_misc_docs::toolbar_format::format_toolbar(
        theme,
        "document-tools",
        crate::showcase_misc_docs::toolbar_format::ToolbarExample::Basic,
    )
}
