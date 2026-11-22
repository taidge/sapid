use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::rich_text_editor()
        .theme(theme)
        .name("Typography editor")
        .toolbar(false)
        .class("showcase-type-editor")
        .default_document(std::rc::Rc::new(
            sapid::RichDocument::new(vec![
                sapid::RichBlock {
                    kind: sapid::RichBlockKind::Heading(2),
                    runs: vec![sapid::RichRun {
                        text: "Heading 2".into(),
                        marks: Default::default(),
                    }],
                },
                sapid::RichBlock {
                    kind: sapid::RichBlockKind::Paragraph,
                    runs: vec![
                        sapid::RichRun {
                            text: "Paragraph with ".into(),
                            marks: Default::default(),
                        },
                        sapid::RichRun {
                            text: "link".into(),
                            marks: sapid::RichMarks {
                                link: Some("https://mantine.dev".into()),
                                ..Default::default()
                            },
                        },
                    ],
                },
            ])
            .expect("valid typography"),
        ))
}
