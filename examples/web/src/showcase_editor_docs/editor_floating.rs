use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let controller = scope.cage(None::<sapid::EditorController>);
        let visible = scope.bond(move || {
            controller.get().is_some_and(|c| {
                let sel = c.selection().get();
                sel.anchor == sel.focus
                    && c.document()
                        .get()
                        .blocks
                        .get(sel.focus.block)
                        .is_some_and(|b| b.runs.iter().all(|r| r.text.is_empty()))
            })
        });
        let palette = theme.clone();
        let popup = when(visible, move || {
            let palette = palette.clone();
            widget(move |scope| {
                let style = scope.bond(move || {
                    let _ = controller.get().map(|c| c.selection().get());
                    #[cfg(target_arch = "wasm32")]
                    {
                        if let Some(rect) = web_sys::window()
                            .and_then(|w| w.get_selection().ok().flatten())
                            .and_then(|s| s.get_range_at(0).ok())
                            .map(|r| r.get_bounding_client_rect())
                        {
                            return Style {
                                position: Some(glory::style::Position::Fixed),
                                left: Some(Length::Px(rect.left().max(8.))),
                                top: Some(Length::Px((rect.top() - 36.).max(4.))),
                                ..Style::new()
                            };
                        }
                    }
                    Style::new()
                });
                sapid::group()
                    .theme(palette.clone())
                    .class("showcase-context-editor-menu")
                    .style(style)
                    .fill(
                        sapid::button()
                            .theme(palette.clone())
                            .compact(true)
                            .label("Insert heading")
                            .on_press(move || {
                                if let Some(c) = controller.get() {
                                    let selection = c.selection().get();
                                    let _ = c.format_at(
                                        selection,
                                        sapid::RichFormat::Block(sapid::RichBlockKind::Heading(2)),
                                    );
                                }
                            }),
                    )
                    .show_in(scope)
            })
        });
        view()
            .class("showcase-context-editor")
            .fill((
                sapid::rich_text_editor()
                    .theme(theme.clone())
                    .name("Floating menu editor")
                    .toolbar(false)
                    .controller_ref(controller)
                    .default_document(std::rc::Rc::new(
                        sapid::RichDocument::new(vec![sapid::RichBlock {
                            kind: sapid::RichBlockKind::Paragraph,
                            runs: vec![sapid::RichRun {
                                text: "Enter a new line to see the floating menu.".into(),
                                marks: Default::default(),
                            }],
                        }])
                        .expect("valid document"),
                    )),
                popup,
            ))
            .show_in(scope)
    })
}
