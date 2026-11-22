use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let preview = widget(move |scope| {
        let state = scope.cage(sapid::TagsInputState {
            values: vec!["First".into(), "Second".into()],
            search: String::new(),
        });
        sapid::field("tags-input-styles-field")
            .theme(theme.clone())
            .label("TagsInput")
            .description("Description")
            .required(true)
            .control(move |field| {
                sapid::stack().gap(4.).fill((
                    sapid::input_group().inset_start(text().content("@")).input(
                        sapid::tags_input("styles-tags")
                            .state(state)
                            .placeholder("TagsInput")
                            .accept_value_on_blur(false)
                            .on_change(move |next| state.revise(|value| *value = next))
                            .field(field),
                    ),
                    suggestions(state),
                ))
            })
            .show_in(scope)
    });
    crate::showcase_style_explorer::explorer(
        "tags-input-styles",
        Box::new(
            sapid::stack()
                .class("site-field-state-example")
                .style(
                    Style::new()
                        .width(Length::Percent(100.))
                        .max_width(Limit::Px(340.)),
                )
                .fill(preview),
        ),
        &[
            (
                "wrapper",
                "Root element of the Input",
                ".sapid-tags-input-shell",
            ),
            ("input", "Input element", "input.sapid-tags-input"),
            (
                "section",
                "Left and right sections",
                ".sapid-input-inset-start",
            ),
            (
                "bottomSection",
                "Bottom section element, rendered inside the input border at the bottom",
                "",
            ),
            ("root", "Root element", ".sapid-field"),
            ("label", "Label element", ".sapid-field-label"),
            (
                "required",
                "Required asterisk element, rendered inside label",
                ".sapid-field-label::after",
            ),
            (
                "description",
                "Description element",
                ".sapid-field-description",
            ),
            ("error", "Error element", ".sapid-field-error"),
            (
                "success",
                "Success element",
                ".site-valid-field .sapid-field-description",
            ),
            ("dropdown", "Dropdown root element", ".site-tag-suggestions"),
            ("options", "Options wrapper", "[role=listbox]"),
            ("option", "Option", "[role=option]"),
            ("empty", "Nothing found message", ""),
            ("group", "Options group wrapper", "[role=group]"),
            (
                "groupLabel",
                "Options group label",
                ".site-tag-suggestions-label",
            ),
            ("pill", "Value pill", ".sapid-tags-input-tag"),
            ("inputField", "Input field", "input.sapid-tags-input"),
            (
                "pillsList",
                "List of pills, also contains input field",
                ".sapid-tags-input-values",
            ),
        ],
    )
}

fn suggestions(state: Cage<sapid::TagsInputState>) -> impl Widget {
    use glory::render::{FocusMode, OptionSemantics, Orientation, Role, SemanticId, Semantics};

    widget(move |scope| {
        let list = SemanticId::new("styles-tag-suggestions")?;
        let mut groups: Vec<Box<dyn Widget>> = Vec::new();
        for (label, values) in [
            ("Frontend", ["React", "Angular"]),
            ("Backend", ["Node", "Django"]),
        ] {
            let matching = move |value: &str, current: &sapid::TagsInputState| {
                !current.values.iter().any(|item| item == value)
                    && value
                        .to_lowercase()
                        .contains(&current.search.trim().to_lowercase())
            };
            let mut rows: Vec<Box<dyn Widget>> = vec![Box::new(
                text().class("site-tag-suggestions-label").content(label),
            )];
            for value in values {
                let owner = list.clone();
                let id = SemanticId::new(format!("styles-tag-suggestion-{value}"))?;
                rows.push(Box::new(
                    glory::ui::button()
                        .class("site-tag-suggestion")
                        .focus(FocusMode::Programmatic)
                        .label(value)
                        .semantics(scope.bond(move || {
                            Some(Box::new(Semantics {
                                id: Some(id.clone()),
                                option: Some(OptionSemantics {
                                    owner: owner.clone(),
                                }),
                                selected: Some(false),
                                hidden: !state.with(|s| matching(value, s)),
                                ..Default::default()
                            }))
                        }))
                        .on(glory::render::EventKind::Activate, move |_| {
                            state.revise(|s| {
                                if !s.values.iter().any(|item| item == value) {
                                    s.values.push(value.to_owned());
                                }
                                s.search.clear();
                            });
                        }),
                ));
            }
            groups.push(Box::new(
                view()
                    .role(Role::Group)
                    .name(label)
                    .semantics(scope.bond(move || {
                        Some(Box::new(Semantics {
                            hidden: !state.with(|s| values.iter().any(|v| matching(v, s))),
                            ..Default::default()
                        }))
                    }))
                    .fill(rows),
            ));
        }
        view()
            .class("site-tag-suggestions")
            .role(Role::Listbox)
            .name("Tag suggestions")
            .semantics(Some(Box::new(Semantics {
                id: Some(list),
                navigation: Some(Orientation::Vertical),
                ..Default::default()
            })))
            .fill(groups)
            .show_in(scope)
    })
}
