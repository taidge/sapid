use glory::prelude::*;

const LOCATIONS: &[([&str; 3], [&str; 3])] = &[
    (["asia", "jp", "tokyo"], ["Asia", "Japan", "Tokyo"]),
    (["asia", "jp", "osaka"], ["Asia", "Japan", "Osaka"]),
    (["asia", "jp", "kyoto"], ["Asia", "Japan", "Kyoto"]),
    (["asia", "kr", "seoul"], ["Asia", "South Korea", "Seoul"]),
    (["asia", "kr", "busan"], ["Asia", "South Korea", "Busan"]),
    (["europe", "fr", "paris"], ["Europe", "France", "Paris"]),
    (["europe", "fr", "lyon"], ["Europe", "France", "Lyon"]),
    (["europe", "de", "berlin"], ["Europe", "Germany", "Berlin"]),
    (["europe", "de", "munich"], ["Europe", "Germany", "Munich"]),
    (
        ["north-america", "us", "new-york"],
        ["North America", "United States", "New York"],
    ),
    (
        ["north-america", "us", "san-francisco"],
        ["North America", "United States", "San Francisco"],
    ),
    (
        ["north-america", "ca", "toronto"],
        ["North America", "Canada", "Toronto"],
    ),
    (
        ["north-america", "ca", "vancouver"],
        ["North America", "Canada", "Vancouver"],
    ),
];

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let selected = scope.cage(Some("tokyo".to_owned()));
        let columns = scope.cage(true);
        #[cfg(target_arch = "wasm32")]
        if let Some(window) = web_sys::window() {
            use wasm_bindgen::{JsCast, closure::Closure};
            let owner = window.clone();
            let refresh = move || {
                let Some(width) = owner.inner_width().ok().and_then(|value| value.as_f64()) else {
                    return;
                };
                let wide = width >= 768.;
                if columns.get_untracked() != wide {
                    columns.revise(|value| *value = wide)
                }
            };
            refresh();
            let callback = Closure::<dyn Fn()>::new(refresh);
            if window
                .add_event_listener_with_callback("resize", callback.as_ref().unchecked_ref())
                .is_ok()
            {
                scope.on_cleanup(move || {
                    let _ = window.remove_event_listener_with_callback(
                        "resize",
                        callback.as_ref().unchecked_ref(),
                    );
                })?;
            }
        }
        let compact_theme = theme.clone();
        let wide_theme = theme.clone();
        when(columns, move || field(wide_theme.clone(), selected, true))
            .otherwise(move || field(compact_theme.clone(), selected, false))
            .show_in(scope)
    })
}

fn field(theme: sapid::Theme, selected: Cage<Option<String>>, columns: bool) -> impl Widget {
    sapid::field("cascader-clear-field")
        .theme(theme)
        .class("site-field-state-example")
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .label("Location")
        .control(move |field| {
            let value = selected.map(move |city| {
                let Some((keys, _)) = LOCATIONS
                    .iter()
                    .find(|(keys, _)| Some(keys[2]) == city.as_deref())
                else {
                    return sapid::TreeChoices::default();
                };
                let path = if columns { &keys[..] } else { &keys[2..] };
                let path = sapid::TreePath::new(path.iter().map(|key| (*key).to_owned())).unwrap();
                sapid::TreeChoices::new([path]).unwrap()
            });
            sapid::cascader("cascader-clear")
                .field(field)
                .placeholder("Pick location")
                .path_separator(" / ")
                .with_columns(columns)
                .nodes(locations(columns))
                .value(value)
                .on_value_change(move |paths| {
                    selected
                        .revise(|city| *city = paths.paths().first().map(|p| p.leaf().to_owned()))
                })
                .clearable(true)
        })
}

fn locations(columns: bool) -> Vec<sapid::TreeNode> {
    let mut result = Vec::new();
    let mut inserted = std::collections::BTreeSet::new();
    for (keys, labels) in LOCATIONS {
        if !columns {
            result.push(sapid::TreeNode::new(keys[2], labels.join(" / ")));
            continue;
        }
        for i in 0..keys.len() {
            if !inserted.insert(keys[i]) {
                continue;
            }
            let mut node = sapid::TreeNode::new(keys[i], labels[i]);
            if i > 0 {
                node = node.parent(keys[i - 1]);
            }
            if i + 1 < keys.len() {
                node = node.kind(sapid::TreeKind::Branch);
            }
            result.push(node);
        }
    }
    result
}
