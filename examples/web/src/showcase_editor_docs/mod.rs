mod editor_bubble;
mod editor_floating;
mod editor_icons;
mod editor_insert_star;
mod editor_invisibles;
mod editor_placeholder;
mod editor_source;
mod editor_sticky;
mod editor_subtle;
mod editor_typography;
use super::showcase_examples::DetailSection;
mod controlled;
mod links;
mod states;
mod toolbar;
pub fn sections(theme: sapid::Theme) -> Vec<DetailSection> {
    vec![
        DetailSection {
            id: "editor-invisibles",
            title: "Invisible characters",
            note: "Try the component in the preview.",
            example: Box::new(editor_invisibles::demo(theme.clone())),
            source: include_str!("editor_invisibles.rs"),
        },
        DetailSection {
            id: "editor-source",
            title: "Source code mode",
            note: "Try the component in the preview.",
            example: Box::new(editor_source::demo(theme.clone())),
            source: include_str!("editor_source.rs"),
        },
        DetailSection {
            id: "editor-typography",
            title: "Typography styles",
            note: "Try the component in the preview.",
            example: Box::new(editor_typography::demo(theme.clone())),
            source: include_str!("editor_typography.rs"),
        },
        DetailSection {
            id: "editor-icons",
            title: "Change icons",
            note: "Try the component in the preview.",
            example: Box::new(editor_icons::demo(theme.clone())),
            source: include_str!("editor_icons.rs"),
        },
        DetailSection {
            id: "editor-insert-star",
            title: "Custom controls",
            note: "Try the component in the preview.",
            example: Box::new(editor_insert_star::demo(theme.clone())),
            source: include_str!("editor_insert_star.rs"),
        },
        DetailSection {
            id: "editor-bubble",
            title: "Bubble menu",
            note: "Try the component in the preview.",
            example: Box::new(editor_bubble::demo(theme.clone())),
            source: include_str!("editor_bubble.rs"),
        },
        DetailSection {
            id: "editor-floating",
            title: "Floating menu",
            note: "Try the component in the preview.",
            example: Box::new(editor_floating::demo(theme.clone())),
            source: include_str!("editor_floating.rs"),
        },
        DetailSection {
            id: "editor-subtle",
            title: "Subtle variant",
            note: "Try the component in the preview.",
            example: Box::new(editor_subtle::demo(theme.clone())),
            source: include_str!("editor_subtle.rs"),
        },
        DetailSection {
            id: "editor-placeholder",
            title: "Placeholder",
            note: "Try the component in the preview.",
            example: Box::new(editor_placeholder::demo(theme.clone())),
            source: include_str!("editor_placeholder.rs"),
        },
        DetailSection {
            id: "editor-sticky",
            title: "Sticky toolbar",
            note: "Try the component in the preview.",
            example: Box::new(editor_sticky::demo(theme.clone())),
            source: include_str!("editor_sticky.rs"),
        },
        DetailSection {
            id: "editor-toolbar",
            title: "Toolbar configuration and selection state",
            note: "toolbar_actions configures up to 16 unique actions: Bold, Italic, Code, Undo, Redo, Paragraph, Heading(1–6) and Quote. Formatting controls expose pressed state from the accepted selection and document. Inline marks require a nonempty selection; a collapsed selection can change block kind. Arrow/Home/End move through the toolbar. Size and theme apply to its real buttons.",
            example: Box::new(toolbar::demo(theme.clone())),
            source: include_str!("toolbar.rs"),
        },
        DetailSection {
            id: "editor-controlled",
            title: "Controlled documents and bounded history",
            note: "document accepts Cage/Bond/Vary<Rc<RichDocument>>. on_document_change proposes the full finite value; this example accepts it. Refusing a proposal restores the accepted value without a history entry. Undo/redo records accepted edits only, with 32 snapshots and a shared 2MiB payload limit. External program changes are silent and clear history; the editor node remains stable.",
            example: Box::new(controlled::demo(theme.clone())),
            source: include_str!("controlled.rs"),
        },
        DetailSection {
            id: "editor-links",
            title: "Safe links and the editor controller",
            note: "controller_ref exposes the Owner-bound editor controller. Select text and apply or remove a link. format queries the current physical UTF-16 selection; format_at accepts an explicit validated range. Only http, https and mailto links are accepted. The document is structured data and does not accept raw HTML or arbitrary Tiptap extensions. Clipboard paste is plain text; images, tables and persistent typing-format marks are outside this finite model.",
            example: Box::new(links::demo(theme.clone())),
            source: include_str!("links.rs"),
        },
        DetailSection {
            id: "editor-states",
            title: "Placeholder, subtle and read only content",
            note: "placeholder is passive guidance while empty; subtle changes presentation. toolbar(false) removes the toolbar. readonly preserves selection while rejecting edits, and enabled(false) disables both the editor and its controls. Web and SSR support the finite document; native rich editing is explicitly unsupported. These examples do not establish OS IME or screen-reader acceptance.",
            example: Box::new(states::demo(theme)),
            source: include_str!("states.rs"),
        },
    ]
}
pub fn navigation() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Invisible characters", "#editor-invisibles"),
        ("Source code mode", "#editor-source"),
        ("Typography styles", "#editor-typography"),
        ("Change icons", "#editor-icons"),
        ("Custom controls", "#editor-insert-star"),
        ("Bubble menu", "#editor-bubble"),
        ("Floating menu", "#editor-floating"),
        ("Subtle variant", "#editor-subtle"),
        ("Placeholder", "#editor-placeholder"),
        ("Sticky toolbar", "#editor-sticky"),
        ("Toolbar and selection", "#editor-toolbar"),
        ("Controlled document", "#editor-controlled"),
        ("Links and controller", "#editor-links"),
        ("Editor states", "#editor-states"),
    ]
}
