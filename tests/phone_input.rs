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
