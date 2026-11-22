use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let initial=parse("<p>Source code control example</p><p>New line with <strong>bold text</strong></p><p>New line with <em>italic text</em></p>").expect("sample source");
        let document = scope.cage(std::rc::Rc::new(initial));
        let source_mode = scope.cage(false);
        let source = scope.cage(String::new());
        let error = scope.cage(String::new());
        let preview_theme = theme.clone();
        let source_theme = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .gap(8.)
            .fill((
                sapid::button()
                    .theme(theme.clone())
                    .compact(true)
                    .label(source_mode.map(|editing| {
                        if *editing {
                            "Apply source".to_owned()
                        } else {
                            "Edit source".to_owned()
                        }
                    }))
                    .on_press(move || {
                        if source_mode.get() {
                            match parse(&source.get()) {
                                Ok(doc) => {
                                    document.revise(|v| *v = std::rc::Rc::new(doc));
                                    error.revise(|v| v.clear());
                                    source_mode.revise(|v| *v = false)
                                }
                                Err(e) => error.revise(|v| *v = e),
                            }
                        } else {
                            source.revise(|v| *v = serialize(&document.get()));
                            source_mode.revise(|v| *v = true);
                        }
                    }),
                when(source_mode, move || {
                    sapid::textarea()
                        .theme(source_theme.clone())
                        .name("Document source")
                        .value(source)
                        .on_value_change(move |v| source.revise(|s| *s = v))
                }),
                when(source_mode.map(|v| !*v), move || {
                    sapid::rich_text_editor()
                        .theme(preview_theme.clone())
                        .name("Source mode editor")
                        .document(document)
                        .on_document_change(move |new| document.revise(|v| *v = new))
                }),
                text().content(error),
            ))
            .show_in(scope)
    })
}

fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
fn serialize(doc: &sapid::RichDocument) -> String {
    let mut parts = Vec::new();
    for block in &doc.blocks {
        let tag = match block.kind {
            sapid::RichBlockKind::Paragraph => "p".into(),
            sapid::RichBlockKind::Quote => "blockquote".into(),
            sapid::RichBlockKind::Heading(n) => format!("h{n}"),
        };
        let mut content = String::new();
        for run in &block.runs {
            let mut value = escape(&run.text);
            for (active, name) in [
                (run.marks.bold, "strong"),
                (run.marks.italic, "em"),
                (run.marks.code, "code"),
            ] {
                if active {
                    value = format!("<{name}>{value}</{name}>");
                }
            }
            if let Some(url) = &run.marks.link {
                value = format!("<a href=\"{}\">{value}</a>", escape(url));
            }
            content.push_str(&value);
        }
        parts.push(format!("<{tag}>{content}</{tag}>"));
    }
    parts.join("\n")
}
fn parse(source: &str) -> std::result::Result<sapid::RichDocument, String> {
    if source.len() > 131072 {
        return Err("Source exceeds the editor document limit".into());
    }
    let fragment = format!("<document>{source}</document>");
    let tree = roxmltree::Document::parse(&fragment).map_err(|e| e.to_string())?;
    let mut blocks = Vec::new();
    for node in tree.root_element().children() {
        if node.is_text() && node.text().unwrap_or_default().trim().is_empty() {
            continue;
        }
        let tag = node.tag_name().name();
        let kind = match tag {
            "p" => sapid::RichBlockKind::Paragraph,
            "blockquote" => sapid::RichBlockKind::Quote,
            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                sapid::RichBlockKind::Heading(tag.as_bytes()[1] - b'0')
            }
            _ => return Err(format!("Unsupported block <{tag}>")),
        };
        let mut runs = Vec::new();
        for leaf in node.descendants().filter(|n| n.is_text()) {
            let mut marks = sapid::RichMarks::default();
            for parent in leaf.ancestors().skip(1).take_while(|p| *p != node) {
                match parent.tag_name().name() {
                    "strong" | "b" => marks.bold = true,
                    "em" | "i" => marks.italic = true,
                    "code" => marks.code = true,
                    "a" => {
                        marks.link = Some(
                            parent
                                .attribute("href")
                                .ok_or("A link requires href")?
                                .into(),
                        )
                    }
                    tag => return Err(format!("Unsupported inline element <{tag}>")),
                }
            }
            runs.push(sapid::RichRun {
                text: leaf.text().unwrap_or_default().into(),
                marks,
            });
        }
        for child in node.descendants().filter(|n| n.is_element() && *n != node) {
            if !matches!(
                child.tag_name().name(),
                "strong" | "b" | "em" | "i" | "code" | "a"
            ) {
                return Err(format!(
                    "Unsupported inline element <{}>",
                    child.tag_name().name()
                ));
            }
        }
        blocks.push(sapid::RichBlock { kind, runs });
    }
    sapid::RichDocument::new(blocks).map_err(|e| e.to_string())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn source_round_trip_preserves_escaped_text_and_nested_marks() {
        let source = "<h2>A &amp; B</h2><p><strong><em>both</em></strong> <a href=\"https://example.org/a?b=2&amp;c=3\">link</a></p>";
        let document = parse(source).unwrap();
        assert_eq!(document.plain_text(), "A & B\nboth link");
        assert_eq!(parse(&serialize(&document)).unwrap(), document);
    }
    #[test]
    fn unsupported_elements_and_invalid_rich_links_leave_the_editor_in_source_mode() {
        assert!(parse("<table/>").is_err());
        assert!(parse("<p><unknown/></p>").is_err());
        assert!(parse("<p><a href=\"unknown:example\">link</a></p>").is_err());
    }
}
