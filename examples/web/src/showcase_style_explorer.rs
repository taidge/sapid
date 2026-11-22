use glory::prelude::*;
use glory::render::{Event, EventKind, PointerPhase};

/// The public selector name, its description and the matching Sapid CSS selector.
pub type StylePart = (&'static str, &'static str, &'static str);

fn selected(state: Cage<Option<usize>>, index: usize, active: bool) {
    state.revise(|current| {
        if active {
            *current = Some(index);
        } else if *current == Some(index) {
            *current = None;
        }
    });
}

pub fn explorer(
    id: &'static str,
    preview: Box<dyn Widget>,
    parts: &'static [StylePart],
) -> Box<dyn Widget> {
    let mut preview = Some(preview);
    Box::new(widget(move |scope| {
        #[cfg(target_arch = "wasm32")]
        {
            let mut rules = String::new();
            for (index, (_, _, selector)) in parts.iter().enumerate() {
                if selector.is_empty() {
                    continue;
                }
                use std::fmt::Write as _;
                let _ = writeln!(
                    rules,
                    "@scope (.site-style-{id}.site-style-active-{index} .site-styles-preview) {{ {selector} {{ outline: 2px solid #fe0d45 !important; outline-offset: -2px !important; }} }}"
                );
            }
            crate::theme_studio::host::style(&format!("showcase-style-rules-{id}"), &rules)
                .map_err(|_| {
                    glory::render::Error::InvalidProps("could not install showcase selector styles")
                })?;
        }
        let hover = scope.cage(None::<usize>);
        let focus = scope.cage(None::<usize>);
        let active = scope.bond(move || hover.get().or(focus.get()));
        let code = scope.bond(move || {
            active
                .get()
                .map(|index| {
                    if parts[index].2.is_empty() {
                        return format!("/* {} is part of the native control or has no separate CSS selector. */", parts[index].0);
                    }
                    format!(
                        "{} {{\n  outline: 2px solid #fe0d45;\n  outline-offset: -2px;\n}}",
                        parts[index].2
                    )
                })
                .unwrap_or_else(|| {
                    "/* Hover over or focus a selector to highlight the matching component part. */"
                        .to_owned()
                })
        });
        let rows = parts
            .iter()
            .enumerate()
            .map(|(index, &(name, description, _))| {
                sapid::button()
                    .unstyled()
                    .name(name)
                    .class("site-style-part")
                    .on(EventKind::PointerWithin, move |event| {
                        if let Event::PointerWithin { phase, .. } = event {
                            selected(hover, index, phase != PointerPhase::Leave);
                        }
                    })
                    .on(EventKind::FocusChanged, move |event| {
                        if let Event::FocusChanged(focused) = event {
                            selected(focus, index, focused);
                        }
                    })
                    .on_press(move || selected(focus, index, true))
                    .fill((
                        text().class("site-style-part-name").content(name),
                        text()
                            .class("site-style-part-description")
                            .content(description),
                    ))
            })
            .collect::<Vec<_>>();
        view()
            .class(scope.bond(move || {
                let base = format!("site-styles-explorer site-style-{id}");
                active.get().map_or(base.clone(), |index| {
                    format!("{base} site-style-active-{index}")
                })
            }))
            .fill((
                view().class("site-styles-columns").fill((
                    view()
                        .class("site-styles-preview")
                        .fill(preview.take().expect("mount once")),
                    view().class("site-styles-parts").fill(
                        view().class("site-style-part-intro").fill((
                            text().class("site-style-part-title").content("Component Styles API"),
                            text().class("site-style-part-description").content("Hover over or focus a selector to highlight its component part."),
                        ))
                    ).fill(rows),
                )),
                text().class("site-styles-css").content(code),
            ))
            .show_in(scope)
    }))
}
