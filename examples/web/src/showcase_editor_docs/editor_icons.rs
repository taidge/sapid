use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::rich_text_editor()
        .theme(theme)
        .name("Editor with custom icons")
        .toolbar_actions([
            sapid::EditorToolbarAction::Bold,
            sapid::EditorToolbarAction::Italic,
        ])
        .toolbar_glyph(sapid::EditorToolbarAction::Bold, "𝐁")
        .toolbar_glyph(sapid::EditorToolbarAction::Italic, "𝑰")
        .default_document(std::rc::Rc::new(
            sapid::RichDocument::new(vec![sapid::RichBlock {
                kind: sapid::RichBlockKind::Paragraph,
                runs: vec![sapid::RichRun {
                    text: "Customize the formatting icons.".into(),
                    marks: Default::default(),
                }],
            }])
            .expect("valid document"),
        ))
}
