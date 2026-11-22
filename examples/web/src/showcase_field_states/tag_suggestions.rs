use glory::prelude::*;
use glory::render::{Event, EventKind, OptionSemantics, Role, SemanticId, Semantics};

#[derive(Clone)]
pub(super) struct Suggestion {
    pub label: String,
    pub group: &'static str,
    pub description: &'static str,
    pub enabled: bool,
}
impl Suggestion {
    pub(super) fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            group: "",
            description: "",
            enabled: true,
        }
    }
}

pub(super) struct SuggestionMenu {
    pub limit: usize,
    pub class: &'static str,
    pub externally_controlled: bool,
}

pub(super) fn suggestions(
    theme: sapid::Theme,
    key: &'static str,
    label: &'static str,
    placeholder: &'static str,
    options: Vec<Suggestion>,
    menu: SuggestionMenu,
) -> impl Widget {
    let SuggestionMenu {
        limit: maximum,
        class,
        externally_controlled: external,
    } = menu;
    let options = std::rc::Rc::new(options);
    widget(move |scope| {
        let state = scope.cage(sapid::TagsInputState::default());
        let open = scope.cage(false);
        let owner = SemanticId::new(format!("{key}-options"))?;
        let list_id = owner.clone();
        let options = options.clone();
        let filter_data = options.clone();
        let matches = scope.bond(move || {
            if !open.get() {
                return Vec::new();
            }
            state.with(|current| {
                let query = current.search.to_lowercase();
                let terms = query.split_whitespace().collect::<Vec<_>>();
                filter_data
                    .iter()
                    .enumerate()
                    .filter(|(_, item)| {
                        !current.values.contains(&item.label) && {
                            let name = item.label.to_lowercase();
                            terms.iter().all(|term| name.contains(*term))
                        }
                    })
                    .take(maximum)
                    .map(|(index, _)| index)
                    .collect::<Vec<_>>()
            })
        });
        let choices = each(matches).key(|index| *index).children(move |row| {
            let options = options.clone();
            let list_id = list_id.clone();
            widget(move |scope| {
                let index = row.value.try_get()?;
                let choice = &options[index];
                let value = choice.label.clone();
                let mut parts: Vec<Box<dyn Widget>> = Vec::new();
                if !choice.group.is_empty()
                    && (index == 0 || options[index - 1].group != choice.group)
                {
                    parts.push(Box::new(
                        text().class("site-tags-menu-group").content(choice.group),
                    ));
                }
                let mut option = glory::ui::button()
                    .class("site-tags-menu-option")
                    .focus(glory::render::FocusMode::Programmatic)
                    .name(value.clone())
                    .enabled(choice.enabled)
                    .semantics(Some(Box::new(Semantics {
                        id: Some(SemanticId::new(format!("{key}-choice-{index}"))?),
                        option: Some(OptionSemantics {
                            owner: list_id.clone(),
                        }),
                        selected: Some(false),
                        ..Default::default()
                    })))
                    .on(EventKind::Activate, move |_| {
                        state.revise(|current| {
                            if !current.values.contains(&value) {
                                current.values.push(value.clone());
                            }
                            current.search.clear();
                        });
                        open.revise(|v| *v = false);
                    });
                option = if choice.description.is_empty() {
                    option.label(choice.label.clone())
                } else {
                    option.fill((
                        text().content(choice.label.clone()),
                        text()
                            .class("site-tags-menu-description")
                            .content(choice.description),
                    ))
                };
                parts.push(Box::new(option));
                view().fill(parts).show_in(scope)
            })
        });
        let list = view()
            .class("site-tags-menu-list")
            .role(Role::Listbox)
            .name("Tag suggestions")
            .semantics(scope.bond(move || {
                Some(Box::new(Semantics {
                    id: Some(owner.clone()),
                    navigation: Some(glory::render::Orientation::Vertical),
                    hidden: !open.get(),
                    ..Default::default()
                }))
            }))
            .fill(choices);
        let mut list = Some(list);
        let target = sapid::field(format!("{key}-field"))
            .theme(theme.clone())
            .label(label)
            .control(move |field| {
                let input = sapid::tags_input(key)
                    .state(state)
                    .placeholder(placeholder)
                    .accept_value_on_blur(false)
                    .field(field)
                    .on_change(move |next| {
                        state.revise(|value| *value = next);
                        if !external {
                            open.revise(|v| *v = true);
                        }
                    });
                view()
                    .class(format!("site-tags-menu-anchor {class}"))
                    .on(EventKind::FocusWithin, move |event| {
                        if let Event::FocusWithin { focused, .. } = event
                            && !external
                        {
                            open.revise(|v| *v = focused);
                        }
                    })
                    .on(EventKind::Dismiss, move |_| open.revise(|v| *v = false))
                    .fill((input, list.take().expect("field control is mounted once")))
            });
        if external {
            sapid::stack()
                .gap(12.)
                .fill((
                    sapid::button()
                        .theme(theme.clone())
                        .label("Toggle dropdown")
                        .on_press(move || open.revise(|v| *v = !*v)),
                    target,
                ))
                .show_in(scope)
        } else {
            target.show_in(scope)
        }
    })
}
