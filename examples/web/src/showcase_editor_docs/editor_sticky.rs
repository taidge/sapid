use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::rich_text_editor()
        .theme(theme)
        .name("Scrolling editor")
        .class("showcase-sticky-editor").default_document(std::rc::Rc::new(sapid::RichDocument::new((1..=18).map(|i|sapid::RichBlock {kind:sapid::RichBlockKind::Paragraph,runs:vec![sapid::RichRun {text:format!("Paragraph {i}. Scroll the document while using its formatting toolbar."),marks:Default::default()}]}).collect()).expect("valid editor document")))
        .placeholder("Write or paste a long note to scroll the toolbar.")
}
