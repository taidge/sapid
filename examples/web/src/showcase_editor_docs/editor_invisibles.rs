use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let shown = scope.cage(false);
        let marks = scope.cage(Vec::<(i32, i32, String)>::new());
        let initial = std::rc::Rc::new(
            sapid::RichDocument::new(
                [
                    "Toggle the control to reveal spaces and paragraph breaks.",
                    "Each space becomes a dot and each paragraph ends with a pilcrow.",
                ]
                .into_iter()
                .map(|line| sapid::RichBlock {
                    kind: sapid::RichBlockKind::Paragraph,
                    runs: vec![sapid::RichRun {
                        text: line.into(),
                        marks: Default::default(),
                    }],
                })
                .collect(),
            )
            .expect("valid text"),
        );
        let document = scope.cage(initial);
        #[cfg(target_arch = "wasm32")]
        observe_marks(scope, shown, document, marks)?;
        let symbols =
            each(marks.map(|items| items.iter().cloned().enumerate().collect::<Vec<_>>()))
                .key(Clone::clone)
                .seed(|(_, (x, y, value))| {
                    text().content(value).style(Style {
                        position: Some(glory::style::Position::Absolute),
                        left: Some(Length::Px(f64::from(x))),
                        top: Some(Length::Px(f64::from(y))),
                        ..Style::new()
                    })
                });
        view()
            .class("showcase-whitespace-editor")
            .fill((
                sapid::button()
                    .theme(theme.clone())
                    .class("showcase-whitespace-toggle")
                    .compact(true)
                    .label(shown.map(|v| {
                        if *v {
                            "Hide invisible characters".into()
                        } else {
                            "Show invisible characters".to_owned()
                        }
                    }))
                    .on_press(move || shown.revise(|v| *v = !*v)),
                sapid::rich_text_editor()
                    .theme(theme.clone())
                    .name("Invisible characters editor")
                    .toolbar(false)
                    .document(document)
                    .on_document_change(move |v| document.revise(|old| *old = v)),
                view()
                    .class("showcase-whitespace-overlay")
                    .semantics(Some(Box::new(glory::render::Semantics {
                        inert: true,
                        ..Default::default()
                    })))
                    .fill(symbols),
            ))
            .show_in(scope)
    })
}

#[cfg(target_arch = "wasm32")]
fn observe_marks(
    scope: &Scope,
    shown: Cage<bool>,
    document: Cage<std::rc::Rc<sapid::RichDocument>>,
    marks: Cage<Vec<(i32, i32, String)>>,
) -> glory::ui::Result {
    use std::{cell::Cell, rc::Rc};
    use wasm_bindgen::{JsCast, closure::Closure};
    let Some(window) = web_sys::window() else {
        return Ok(());
    };
    let pending = Rc::new(Cell::new(None::<i32>));
    let finished = pending.clone();
    let frame = Rc::new(Closure::<dyn FnMut(f64)>::new(move |_| {
        finished.set(None);
        if !shown.get() {
            marks.revise(|v| v.clear());
            return;
        }
        let Some(doc) = web_sys::window().and_then(|w| w.document()) else {
            return;
        };
        let Ok(Some(root)) = doc.query_selector(".showcase-whitespace-editor") else {
            return;
        };
        let Ok(Some(editor)) = root.query_selector("[contenteditable=true]") else {
            return;
        };
        let origin = root.get_bounding_client_rect();
        let mut result = Vec::new();
        for index in 0..editor.child_element_count() {
            let Some(block) = editor.children().item(index) else {
                continue;
            };
            let mut end_node = None;
            let mut nodes = vec![web_sys::Node::from(block.clone())];
            while let Some(node) = nodes.pop() {
                if node.node_type() == web_sys::Node::TEXT_NODE {
                    let text = node.node_value().unwrap_or_default();
                    let mut offset = 0u32;
                    for ch in text.chars() {
                        let next = offset + ch.len_utf16() as u32;
                        if ch == ' '
                            && let Ok(range) = doc.create_range()
                            && range.set_start(&node, offset).is_ok()
                            && range.set_end(&node, next).is_ok()
                        {
                            let rect = range.get_bounding_client_rect();
                            result.push((
                                (rect.x() - origin.x()).round() as i32,
                                (rect.y() - origin.y()).round() as i32,
                                "·".into(),
                            ));
                        }
                        offset = next;
                    }
                    if offset > 0 {
                        end_node = Some((node, offset));
                    }
                } else {
                    let children = node.child_nodes();
                    for i in (0..children.length()).rev() {
                        if let Some(child) = children.item(i) {
                            nodes.push(child)
                        }
                    }
                }
            }
            let bounds = block.get_bounding_client_rect();
            let caret = end_node
                .and_then(|(node, offset)| {
                    let range = doc.create_range().ok()?;
                    range.set_start(&node, offset).ok()?;
                    range.collapse_with_to_start(true);
                    Some(range.get_bounding_client_rect())
                })
                .filter(|rect| rect.height() > 0.);
            let (x, y) = caret.map_or((bounds.x(), bounds.y()), |rect| (rect.x(), rect.y()));
            result.push((
                (x - origin.x()).round() as i32,
                (y - origin.y()).round() as i32,
                "¶".into(),
            ));
        }
        marks.revise(|current| *current = result);
    }));
    let schedule = Rc::new({
        let window = window.clone();
        let pending = pending.clone();
        move || {
            if pending.get().is_none() {
                pending.set(
                    window
                        .request_animation_frame(frame.as_ref().as_ref().unchecked_ref())
                        .ok(),
                );
            }
        }
    });
    let paint = schedule.clone();
    scope.structural(move || {
        shown.get();
        document.with(|_| ());
        paint();
    });
    let resize_handler = Closure::<dyn FnMut(web_sys::Event)>::new(move |_| schedule());
    let _ =
        window.add_event_listener_with_callback("resize", resize_handler.as_ref().unchecked_ref());
    scope.on_cleanup(move || {
        if let Some(id) = pending.take() {
            let _ = window.cancel_animation_frame(id);
        }
        let _ = window
            .remove_event_listener_with_callback("resize", resize_handler.as_ref().unchecked_ref());
    })?;
    Ok(())
}
