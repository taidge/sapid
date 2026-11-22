//! Bounded JSON disclosure lists. Tree widget keyboard/selection roles are not
//! inferred from object shape; these are genuine document lists and buttons.
use crate::appearance::{Common, common_methods, configure, observe};
use crate::composite::{self, Callback, Observers};
use crate::json_tree_model::*;
use crate::state::{Control, Value};
use glory::render::{DocumentPart, EventKind, TextKind};
use glory::style::{Display, Edges, Family, Style};
use glory::ui::{Result, Widget};
use glory::{Bond, Cage, Scope, Vary};
use std::{cell::Cell, collections::HashSet, rc::Rc};
mod tree_mode;

pub struct JsonViewer {
    common: Common,
    key: String,
    source: Vary<String>,
    root_name: String,
    sort_keys: bool,
    line_numbers: bool,
    types: bool,
    string_limit: Option<usize>,
    marks: Vec<(String, glory::style::Color)>,
    initial_depth: Option<usize>,
    expanded: Value<Vec<String>>,
    search: Vary<String>,
    limits: JsonViewLimits,
    change: Option<Box<dyn FnMut(Vec<String>)>>,
    copy: Option<crate::CopyButton>,
    tree_navigation: bool,
}
pub fn json_viewer(key: impl Into<String>) -> JsonViewer {
    JsonViewer {
        common: Common::default(),
        key: key.into(),
        source: "null".into(),
        root_name: "$".into(),
        sort_keys: false,
        line_numbers: false,
        types: false,
        string_limit: None,
        marks: Vec::new(),
        initial_depth: None,
        expanded: Value {
            source: None,
            default: vec![String::new()],
        },
        search: "".into(),
        limits: JsonViewLimits::default(),
        change: None,
        copy: None,
        tree_navigation: false,
    }
}
impl JsonViewer {
    common_methods!();
    /// Add a configured CopyButton for the whole accepted JSON source.
    pub fn copy_button(mut self, value: crate::CopyButton) -> Self {
        self.copy = Some(value);
        self
    }
    /// Reuse the shared Tree keyboard, focus and accessibility controller.
    pub fn tree_navigation(mut self, value: bool) -> Self {
        self.tree_navigation = value;
        self
    }
    pub fn source(mut self, value: impl Into<Vary<String>>) -> Self {
        self.source = value.into();
        self
    }
    /// JSON pointers. Empty string names the root; unknown paths preserve intent.
    pub fn expanded(mut self, value: impl Into<Vary<Vec<String>>>) -> Self {
        self.expanded.source = Some(value.into());
        self
    }
    /// Choose the label for the root value. JSON pointer paths remain stable.
    pub fn root_name(mut self, name: impl Into<String>) -> Self {
        self.root_name = name.into();
        self
    }
    /// Present object children alphabetically while preserving numeric array order and source text.
    pub fn sort_keys(mut self, value: bool) -> Self {
        self.sort_keys = value;
        self
    }
    /// Show a gutter with consecutive numbers for visible rows.
    pub fn with_line_numbers(mut self, value: bool) -> Self {
        self.line_numbers = value;
        self
    }

    /// Add a type badge after primitive values.
    pub fn with_types(mut self, value: bool) -> Self {
        self.types = value;
        self
    }
    /// Shorten long strings to a Unicode character count until the user expands the row.
    pub fn collapse_strings_after(mut self, characters: usize) -> Self {
        self.string_limit = Some(characters);
        self
    }
    /// Highlight a JSON pointer using the supplied row background.
    pub fn highlight_path(mut self, path: impl Into<String>, color: glory::style::Color) -> Self {
        if self.marks.len() <= MAX_JSON_VIEW_NODES {
            self.marks.push((path.into(), color));
        }
        self
    }
    /// Open object and array rows through this many levels on initial mount.
    pub fn default_expand_depth(mut self, depth: usize) -> Self {
        self.initial_depth = Some(depth);
        self
    }
    pub fn default_expanded(mut self, value: impl IntoIterator<Item = String>) -> Self {
        self.initial_depth = None;
        self.expanded.default.clear();
        crate::document::extend_bounded(&mut self.expanded.default, value, MAX_JSON_VIEW_NODES);
        self
    }
    pub fn search(mut self, value: impl Into<Vary<String>>) -> Self {
        self.search = value.into();
        self
    }
    pub fn limits(mut self, value: JsonViewLimits) -> Self {
        self.limits = value;
        self
    }
    pub fn on_expanded_change(mut self, value: impl FnMut(Vec<String>) + 'static) -> Self {
        self.change = Some(Box::new(value));
        self
    }
}
fn value_ink(theme: &crate::Theme, kind: crate::JsonKind) -> glory::style::Color {
    match kind {
        crate::JsonKind::String => theme.syntax_string,
        crate::JsonKind::Number => theme.accent_text,
        crate::JsonKind::Boolean => theme.warning,
        crate::JsonKind::Null => theme.danger,
        _ => theme.foreground,
    }
}
fn error(value: JsonViewError) -> glory::ui::Error {
    glory::render::Error::InvalidProps(match value {
        JsonViewError::Configuration => "invalid JSON viewer limits",
        JsonViewError::Syntax(_) => "JSON viewer source is invalid or exceeds byte/depth budget",
        JsonViewError::Empty => "JSON viewer requires a value",
        JsonViewError::Nodes => "JSON viewer exceeds 2048 current nodes",
        JsonViewError::Depth => "JSON viewer exceeds its nesting budget",
        JsonViewError::Key => "JSON viewer key exceeds 256 bytes",
        JsonViewError::Path => "JSON viewer path exceeds 2048 bytes",
        JsonViewError::DuplicateKey => "JSON viewer rejects duplicate object keys",
        JsonViewError::Search => "JSON viewer search exceeds 256 bytes",
    })
    .into()
}
fn check_paths(paths: &[String]) -> Result {
    if paths.len() > MAX_JSON_VIEW_NODES
        || paths
            .iter()
            .try_fold(0usize, |n, path| n.checked_add(path.len()))
            .is_none_or(|bytes| bytes > crate::MAX_JSON_BYTES)
    {
        return Err(glory::render::Error::InvalidProps(
            "JSON expansion exceeds 2048 paths or 65536 total bytes",
        )
        .into());
    }
    let mut seen = std::collections::HashSet::new();
    for path in paths {
        if path.len() > MAX_JSON_VIEW_PATH_BYTES
            || !path.is_empty() && !path.starts_with('/')
            || !seen.insert(path)
        {
            return Err(glory::render::Error::InvalidProps(
                "JSON expansion paths must be unique finite JSON pointers",
            )
            .into());
        }
        let mut chars = path.chars();
        while let Some(c) = chars.next() {
            if c == '~' && !matches!(chars.next(), Some('0' | '1')) {
                return Err(
                    glory::render::Error::InvalidProps("invalid JSON pointer escape").into(),
                );
            }
        }
    }
    Ok(())
}
struct State {
    types: bool,
    string_limit: Option<usize>,
    marks: std::collections::HashMap<String, glory::style::Color>,
    expanded: Control<Vec<String>>,
    accepted_paths: Cage<Vec<String>>,
    query: Cage<String>,
    search: Bond<Rc<Search>>,
    changed: Callback<Vec<String>>,
    activated: Observers,
    namespace: String,
    next: Cell<u64>,
    theme: crate::Theme,
    size: crate::Size,
    unstyled: bool,
    enabled: Vary<bool>,
}
struct Search {
    matches: HashSet<String>,
    open: HashSet<String>,
}
impl State {
    fn open(&self, path: &str) -> bool {
        if self
            .accepted_paths
            .with(|paths| paths.iter().any(|p| p == path))
        {
            return true;
        }
        self.search.with(|search| search.open.contains(path))
    }
    fn matching(&self, path: &str) -> bool {
        self.search.with(|search| search.matches.contains(path))
    }
}
type Item = (Rc<JsonTree>, usize);
fn show_row(
    state: Rc<State>,
    row: Bond<Option<Item>>,
    class: &'static str,
    scope: &Scope,
) -> Result {
    let expanded = scope.cage(false);
    let limit = state.string_limit;
    let display = scope.bond(move || {
        row.with(|item| {
            let Some((tree, index)) = item else {
                return (String::new(), false, "", false);
            };
            let node = &tree.nodes()[*index];
            let kind = tree_mode::kind(node.kind);
            let primitive = !matches!(node.kind, crate::JsonKind::Object | crate::JsonKind::Array);
            if !primitive {
                return (
                    format!("{}: {} {}", node.label, kind, node.children.len()),
                    false,
                    kind,
                    false,
                );
            }
            let raw = tree.value(*index).unwrap_or("");
            if node.kind == crate::JsonKind::String
                && let Some(max) = limit
                && let Ok(decoded) = serde_json::from_str::<String>(raw)
                && decoded.chars().count() > max
            {
                let content = if expanded.get() {
                    raw.to_string()
                } else {
                    format!(
                        "{}…",
                        serde_json::to_string(&decoded.chars().take(max).collect::<String>())
                            .expect("a string is valid JSON")
                    )
                };
                return (format!("{}: {}", node.label, content), true, kind, true);
            }
            (format!("{}: {}", node.label, raw), false, kind, true)
        })
    });
    let paint = state.clone();
    let text = glory::ui::text()
        .kind(TextKind::Code)
        .class(class)
        .content(scope.bond(move || display.with(|value| value.0.clone())))
        .selectable(true)
        .style(scope.bond(move || {
            let mut style = Style::new();
            if !paint.unstyled {
                row.with(|item| {
                    if let Some((tree, index)) = item {
                        let node = &tree.nodes()[*index];
                        style.color = Some(value_ink(&paint.theme, node.kind));
                        style.background = paint.marks.get(&node.path).copied().or_else(|| {
                            paint
                                .matching(&node.path)
                                .then_some(paint.theme.accent_soft)
                        });
                    }
                });
            }
            style
        }));
    if !state.types && limit.is_none() {
        return text.show_in(scope);
    }
    let type_label = scope.bond(move || display.with(|value| value.2.to_lowercase()));
    let show_types = state.types;
    let palette = state.theme.clone();
    glory::ui::view()
        .class("sapid-json-formatted")
        .style(Style::new().display(Display::Flex).gap(5.))
        .fill((
            text,
            glory::ui::when(
                scope.bond(move || display.with(|value| show_types && value.3)),
                move || {
                    glory::ui::text()
                        .class("sapid-json-type")
                        .content(type_label)
                },
            ),
            glory::ui::when(
                scope.bond(move || display.with(|value| value.1)),
                move || {
                    crate::button()
                        .theme(palette.clone())
                        .compact(true)
                        .class("sapid-json-string-toggle")
                        .label(expanded.map(|open| {
                            if *open {
                                "Show less".to_owned()
                            } else {
                                "Show more".to_owned()
                            }
                        }))
                        .on_press(move || expanded.revise(|old| *old = !*old))
                },
            ),
        ))
        .show_in(scope)
}
fn branch(state: Rc<State>, item: Cage<Item>) -> Box<dyn Widget> {
    Box::new(glory::ui::widget(move |scope| {
        let path = item.with(|(tree, index)| tree.nodes()[*index].path.clone());
        let slot = state
            .next
            .get()
            .checked_add(1)
            .ok_or(glory::render::Error::InvalidProps(
                "JSON disclosure identities exhausted",
            ))?;
        state.next.set(slot);
        let row = scope.cage(None);
        let mut style = Style::new().display(Display::Block);
        if !state.unstyled {
            style.padding = Some(Edges {
                left: 12.,
                ..Edges::all(0.)
            });
        }
        glory::ui::view()
            .document_part(DocumentPart::ListItem)
            .class("sapid-json-node")
            .node_ref(row)
            .style(style)
            .show_in(scope)?;
        let children = scope.child_in(row.get().ok_or(glory::ui::Error::Gone)?)?;
        let container = scope.bond(move || {
            item.with(|(tree, index)| {
                matches!(
                    tree.nodes()[*index].kind,
                    crate::JsonKind::Array | crate::JsonKind::Object
                )
            })
        });
        let branch_state = state.clone();
        let branch_path = path.clone();
        let leaf_state = state.clone();

        glory::ui::when(container, move || {
            let state = branch_state.clone();
            let path = branch_path.clone();
            glory::ui::widget(move |scope| {
                let state = state.clone();
                let path = path.clone();
                let title = scope.bond(move || {
                    item.with(|(tree, index)| {
                        let node = &tree.nodes()[*index];
                        format!(
                            "{}: {} {}",
                            node.label,
                            if node.kind == crate::JsonKind::Array {
                                "Array"
                            } else {
                                "Object"
                            },
                            node.children.len()
                        )
                    })
                });
                let open_state = state.clone();
                let open_path = path.clone();
                let open = scope.bond(move || open_state.open(&open_path));
                let enabled_state = state.clone();
                let enabled = scope.bond(move || {
                    enabled_state.enabled.get() && enabled_state.query.with(String::is_empty)
                });
                let changed = state.changed.clone();
                let control = state.expanded.clone();
                let owner = scope.id();
                let weak = scope.downgrade();
                let activated = state.activated.clone();
                let child_state = state.clone();
                let mut disclosure = crate::collapsible(format!(
                    "json-{}-{}-{slot}",
                    state.namespace.len(),
                    state.namespace
                ))
                .theme(state.theme.clone())
                .size(state.size)
                .label(title)
                .open(open)
                .enabled(enabled)
                .on_open_change(move |next| {
                    let result = (|| -> Result {
                        let mut proposal = control.source.try_with(|paths| {
                            check_paths(paths)?;
                            Ok::<_, glory::ui::Error>(paths.clone())
                        })??;
                        if next {
                            if !proposal.contains(&path) {
                                proposal.push(path.clone())
                            }
                        } else {
                            proposal.retain(|value| value != &path)
                        }
                        check_paths(&proposal)?;
                        control.accept(&proposal)?;
                        changed.emit(owner, proposal);
                        Ok(())
                    })();
                    if let Err(error) = result
                        && let Some(scope) = weak.upgrade()
                    {
                        scope.report_error(error)
                    }
                })
                .on(EventKind::Activate, move |event| {
                    activated.emit(owner, event)
                })
                .content(move || {
                    let state = child_state.clone();
                    glory::ui::widget(move |scope| {
                        let state = state.clone();
                        let items = scope.bond(move || {
                            item.with(|(tree, index)| {
                                tree.nodes()[*index]
                                    .children
                                    .iter()
                                    .map(|index| (tree.clone(), *index))
                                    .collect::<Vec<_>>()
                            })
                        });
                        glory::ui::view()
                            .document_part(DocumentPart::UnorderedList)
                            .class("sapid-json-children")
                            .style(Style::new().margin(Edges::all(0.)).padding(Edges::all(0.)))
                            .fill(
                                glory::ui::each(items)
                                    .key(|(tree, index)| tree.nodes()[*index].path.clone())
                                    .children(move |entry| {
                                        let item = entry.value;
                                        branch(state.clone(), item)
                                    }),
                            )
                            .show_in(scope)
                    })
                });
                if state.unstyled {
                    disclosure = disclosure.unstyled();
                }
                disclosure.show_in(scope)
            })
        })
        .otherwise(move || {
            let state = leaf_state.clone();

            glory::ui::widget(move |scope| {
                let state = state.clone();
                let row = scope.bond(move || Some(item.get()));
                show_row(state, row, "sapid-json-value", scope)
            })
        })
        .show_in(&children)?;
        scope.retain(children)?;
        Ok(())
    }))
}
impl Widget for JsonViewer {
    fn build(&mut self, scope: &Scope) -> Result {
        if self.key.len() > 24 {
            return Err(
                glory::render::Error::InvalidProps("JSON viewer key exceeds 24 bytes").into(),
            );
        }
        glory::render::SemanticId::new(&self.key)?;
        self.limits.validate().map_err(error)?;
        if self.root_name.trim().is_empty()
            || self.root_name.len() > 256
            || self.root_name.contains(['\0', '\r', '\n'])
        {
            return Err(glory::render::Error::InvalidProps(
                "JSON root label must be a nonempty single line within 256 bytes",
            )
            .into());
        }
        let mut initial = self
            .source
            .try_with(|source| parse_json_tree(source, self.limits).map_err(error))??;
        let sort_keys = self.sort_keys;
        if sort_keys {
            initial.sort_keys();
        }
        initial.label_root(&self.root_name);
        if let Some(depth) = self.initial_depth {
            self.expanded.default = initial.expanded_at_depth(depth);
        }
        let root_name = self.root_name.clone();
        let initial = Rc::new(initial);
        let projection = if self.tree_navigation {
            Some(scope.cage(tree_mode::Projection::prepare(initial.clone(), None)?))
        } else {
            None
        };
        let tree = scope.cage(initial);
        let source = self.source.clone();
        let limits = self.limits;
        let weak = scope.downgrade();
        scope.structural(move || {
            let Some(scope) = weak.upgrade() else { return };
            let result = (|| -> Result {
                let next = source.try_with(|source| {
                    if glory::core::untracked(|| tree.try_with(|old| old.source() == source))? {
                        return Ok::<_, glory::ui::Error>(None);
                    }
                    parse_json_tree(source, limits)
                        .map(|mut tree| {
                            if sort_keys {
                                tree.sort_keys();
                            }
                            tree.label_root(&root_name);
                            Some(tree)
                        })
                        .map_err(error)
                })??;
                if let Some(next) = next {
                    let next = Rc::new(next);
                    let prepared = if let Some(projection) = projection {
                        Some(glory::core::untracked(|| {
                            projection.try_with(|previous| {
                                tree_mode::Projection::prepare(next.clone(), Some(previous))
                            })
                        })??)
                    } else {
                        None
                    };
                    if let (Some(projection), Some(prepared)) = (projection, prepared) {
                        projection.try_revise(|value| *value = prepared)?;
                    }
                    tree.try_revise(|value| *value = next)?;
                }
                Ok(())
            })();
            if let Err(error) = result {
                scope.report_error(error)
            }
        });
        let expanded = std::mem::take(&mut self.expanded).mount(scope);
        let accepted_paths =
            composite::validated(scope, &expanded.source, |paths| check_paths(paths))?;
        let query = composite::validated(scope, &self.search, |query| {
            if query.len() <= MAX_JSON_SEARCH_BYTES {
                Ok(())
            } else {
                Err(error(JsonViewError::Search))
            }
        })?;
        let search = scope.bond(move || {
            let query = query.get();
            tree.with(|tree| {
                let mut result = Search {
                    matches: HashSet::new(),
                    open: HashSet::new(),
                };
                for index in tree.search(&query).expect("validated search") {
                    result.matches.insert(tree.nodes()[index].path.clone());
                    let mut ancestor = Some(index);
                    while let Some(index) = ancestor {
                        result.open.insert(tree.nodes()[index].path.clone());
                        ancestor = tree.nodes()[index].parent;
                    }
                }
                Rc::new(result)
            })
        });
        check_paths(
            &self
                .marks
                .iter()
                .map(|(p, _)| p.clone())
                .collect::<Vec<_>>(),
        )?;
        let state = Rc::new(State {
            types: self.types,
            string_limit: self.string_limit,
            marks: std::mem::take(&mut self.marks).into_iter().collect(),
            expanded,
            accepted_paths,
            query,
            search,
            changed: Callback::new(self.change.take()),
            activated: Observers::new(
                self.common
                    .handlers
                    .remove(&EventKind::Activate)
                    .unwrap_or_default(),
            ),
            namespace: self.key.clone(),
            next: Cell::new(0),
            theme: self.common.theme.clone(),
            size: self.common.size,
            unstyled: self.common.unstyled,
            enabled: self.common.enabled.clone(),
        });
        let reference = self.common.node_ref.unwrap_or_else(|| scope.cage(None));
        let mut style = self.common.theme.label(self.common.size);
        style.font_family = Some(vec![Family::Mono]);
        style.background = Some(self.common.theme.canvas);
        let mut root = configure!(
            self.common,
            glory::ui::view(),
            scope,
            if self.line_numbers {
                "sapid-json-viewer sapid-json-numbered"
            } else {
                "sapid-json-viewer"
            },
            style
        )
        .node_ref(reference);
        let owner = scope.id();
        for (kind, mut handlers) in std::mem::take(&mut self.common.handlers) {
            root = root.on(kind, move |event| observe(&mut handlers, owner, event))
        }
        root.show_in(scope)?;
        let area = scope.child_in(reference.get().ok_or(glory::ui::Error::Gone)?)?;
        if let Some(mut copy) = self.copy.take() {
            if self.common.unstyled {
                copy = copy.unstyled();
            }
            copy.source(scope.bond(move || tree.with(|value| value.source().to_string())))
                .show_in(&area)?;
        }
        if let Some(projection) = projection {
            tree_mode::show(state, projection, &area)?;
            scope.retain(area)?;
            return Ok(());
        }
        let items = scope.bond(move || vec![(tree.get(), 0usize)]);
        glory::ui::view()
            .document_part(DocumentPart::UnorderedList)
            .class("sapid-json-root")
            .style(Style::new().margin(Edges::all(0.)).padding(Edges::all(0.)))
            .fill(glory::ui::each(items).key(|_| ()).children(move |entry| {
                let item = entry.value;
                branch(state.clone(), item)
            }))
            .show_in(&area)?;
        scope.retain(area)?;
        Ok(())
    }
}
