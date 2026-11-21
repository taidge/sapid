#![cfg(feature = "phone")]
use glory::render::{Event, InputProps, NodeId, Props, Purpose};
use glory::ui::App;
use sapid::{PhoneDraft, phone_input};
use std::{cell::RefCell, rc::Rc};

fn editor(app: &App) -> (NodeId, InputProps) {
    let tree = app.tree();
    let mut pending = tree.roots().to_vec();
    while let Some(id) = pending.pop() {
        let node = tree.node(id).unwrap();
        if let Props::Input(value) = node.descriptor.view().props
            && value.purpose == Purpose::Tel
        {
            return (id, value.clone());
        }
        pending.extend(&node.children);
    }
    panic!("missing telephone editor");
}

#[test]
fn national_and_international_forms_preserve_extensions() {
    let draft = PhoneDraft::new("GB", "020 7946 0018 ext. 42");
    assert_eq!(draft.e164().as_deref(), Some("+442079460018"));
    assert_eq!(
        draft.rfc3966().as_deref(),
        Some("tel:+44-20-7946-0018;ext=42")
    );
    assert_eq!(
        PhoneDraft::new("US", "+44 20 7946 0018").e164(),
        draft.e164()
    );
    for text in ["", "+", "123"] {
        assert!(PhoneDraft::new("US", text).e164().is_none());
    }
}

#[test]
fn country_detection_and_controlled_rejection_keep_one_editor() {
    let app = App::memory();
    let source = app.scope().cage(PhoneDraft::new("US", ""));
    let proposals = Rc::new(RefCell::new(Vec::new()));
    let seen = proposals.clone();
    app.mount(
        phone_input("contact")
            .value(source)
            .regions([("US", "United States"), ("CA", "Canada")])
            .on_value_change(move |value| seen.borrow_mut().push(value)),
    )
    .unwrap();
    let (id, props) = editor(&app);
    app.dispatch(
        id,
        Event::ValueChanged {
            value: "+1 416 555 0123".into(),
            composing: false,
            revision: props.revision,
        },
    )
    .unwrap();
    assert_eq!(
        &*proposals.borrow(),
        &[PhoneDraft::new("CA", "+1 416 555 0123")]
    );
    assert_eq!(source.get(), PhoneDraft::new("US", ""));
    assert_eq!(editor(&app).1.value, "");
    source.revise(|v| *v = proposals.borrow()[0].clone());
    app.flush().unwrap();
    assert_eq!(editor(&app).0, id);
    assert_eq!(editor(&app).1.value, "+1 416 555 0123");
    app.clear().unwrap();
    assert_eq!(app.stats().nodes, 0);
}

#[test]
fn default_database_and_invalid_region_configuration() {
    let app = App::memory();
    app.mount(phone_input("all-regions")).unwrap();
    let (id, props) = editor(&app);
    app.dispatch(
        id,
        Event::Commit {
            value: "+44 20 7946 0018".into(),
            revision: props.revision,
        },
    )
    .unwrap();
    assert_eq!(editor(&app).1.value, "020 7946 0018");
    for regions in [
        vec![("XX", "Unknown")],
        vec![("GB", "UK")],
        vec![("US", "US"), ("US", "Duplicate")],
    ] {
        let bad = App::memory();
        assert!(bad.mount(phone_input("invalid").regions(regions)).is_err());
        assert!(bad.tree().is_empty());
    }
}

#[test]
fn field_controls_one_editor_one_successful_value_and_the_declared_region_order() {
    let app = App::memory();
    let enabled = app.scope().cage(true);
    let readonly = app.scope().cage(false);
    let reference = app.scope().cage(None);
    app.mount(
        sapid::field("support")
            .label("Support phone")
            .form_name("phone")
            .required(true)
            .enabled(enabled)
            .readonly(readonly)
            .control(move |field| {
                phone_input("region")
                    .field(field)
                    .node_ref(reference)
                    .regions([("GB", "UK"), ("US", "USA"), ("CA", "Canada")])
                    .default_value(PhoneDraft::new("GB", "020 7946 0018"))
            }),
    )
    .unwrap();
    let id = editor(&app).0;
    for (active, locked) in [(true, false), (true, true), (false, false)] {
        enabled.revise(|v| *v = active);
        readonly.revise(|v| *v = locked);
        app.flush().unwrap();
        assert_eq!(reference.get(), Some(id));
        let tree = app.tree();
        let mut pending = tree.roots().to_vec();
        let mut named = Vec::new();
        let mut regions = Vec::new();
        while let Some(next) = pending.pop() {
            let node = tree.node(next).unwrap();
            let view = node.descriptor.view();
            if next == id {
                let semantics = view.semantics.as_deref().unwrap();
                assert!(semantics.required);
                assert_eq!(semantics.readonly, locked);
                assert!(semantics.form_control.is_none());
            }
            if let Props::Input(input) = view.props {
                assert_eq!(input.enabled, active);
                if let Some(control) = view
                    .semantics
                    .as_deref()
                    .and_then(|s| s.form_control.as_ref())
                {
                    named.push((control.name.clone(), input.value.clone()));
                }
            }
            if let Props::Select(select) = view.props {
                assert_eq!(select.enabled, active && !locked);
                regions.extend(select.options().map(|(opt, _)| opt.key.as_str().to_owned()));
            }
            pending.extend(&node.children);
        }
        assert_eq!(regions, ["GB", "US", "CA"]);
        assert_eq!(
            named,
            [("phone".to_owned(), "tel:+44-20-7946-0018".to_owned())]
        );
    }
}
