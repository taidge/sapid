use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let preview = sapid::progress(35.)
        .theme(theme.clone())
        .name("Document storage")
        .width(Length::Percent(100.))
        .thickness(16.)
        .section_label(0, "Documents")
        .sections([sapid::ProgressSection::new(35., theme.accent)]);
    crate::showcase_style_explorer::explorer(
        "progress-styles",
        Box::new(preview),
        &[
            ("root", "Root element", ".sapid-progress"),
            (
                "section",
                "`Progress.Section` root element",
                ".sapid-progress-section",
            ),
            (
                "label",
                "`Progress.Label` root element",
                ".sapid-progress-section-label",
            ),
        ],
    )
}
