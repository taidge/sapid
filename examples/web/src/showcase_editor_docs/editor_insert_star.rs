use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let control = scope.cage(None::<sapid::EditorController>);
        let weak = scope.downgrade();
        view()
            .class("showcase-custom-editor")
            .fill((
                sapid::button()
                    .theme(theme.clone())
                    .compact(true)
                    .name("Insert star")
                    .label("☆")
                    .on_press(move || {
                        if let Some(c) = control.get()
                            && let Err(e) =
                                c.insert_at(c.selection().get(), "⭐", Default::default())
                            && let Some(owner) = weak.upgrade()
                        {
                            owner.report_error(e)
                        }
                    }),
                sapid::rich_text_editor()
                    .theme(theme.clone())
                    .toolbar(false)
                    .name("Star editor")
                    .controller_ref(control)
                    .default_document(std::rc::Rc::new(
                        sapid::RichDocument::new(vec![sapid::RichBlock {
                            kind: sapid::RichBlockKind::Paragraph,
                            runs: vec![sapid::RichRun {
                                text: "Click the control to insert a star emoji.".into(),
                                marks: Default::default(),
                            }],
                        }])
                        .expect("valid document"),
                    )),
            ))
            .show_in(scope)
    })
}
