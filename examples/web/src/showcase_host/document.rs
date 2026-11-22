use crate::showcase_navigation::{Page, Route, State};
use crate::theme_studio::{
    host,
    model::{ThemeRecord, ThemeStore},
};

pub(super) fn resolve(mut route: Route) -> Result<(Page, ThemeStore), String> {
    let (store, mut notice) = match host::load() {
        Ok(store) => (store, String::new()),
        Err(e) => (
            ThemeStore::default(),
            format!("Local themes could not be loaded: {e}. Stored data was kept."),
        ),
    };
    let selected = store.themes.iter().find(|r| r.id == route.palette);
    let theme = if let Some(record) = selected {
        let mode = if record.supports(route.mode()) {
            route.mode()
        } else {
            notice = format!(
                "{} does not support {}. Showing its default {} mode.",
                record.title,
                route.scheme,
                record.default_scheme.name()
            );
            record.default_scheme
        };
        record.theme(mode)?
    } else {
        if !matches!(route.palette.as_str(), "blue" | "green") {
            notice="That local preview is unavailable in this browser. Choose a saved palette or create one in Theme studio.".into();
        }
        crate::themes::select(&route.palette, route.mode())
    };
    let mut initial = selected.cloned().unwrap_or_else(|| {
        ThemeRecord::from_definition(if theme.name == "green" {
            &sapid::GREEN
        } else {
            &sapid::BLUE
        })
    });
    if let Some(edit) = &route.edit {
        if let Some(record) = store.themes.iter().find(|r| &r.id == edit) {
            initial = record.clone()
        } else {
            notice =
                "The saved theme to edit is unavailable. Your current workspace was kept.".into();
            route.edit = None;
        }
    }
    route.scheme = theme.scheme.name().into();
    Ok((
        Page {
            route,
            theme,
            initial,
            notice,
            revision: 0,
            shell_revision: 0,
        },
        store,
    ))
}
pub(super) fn publish(state: State, mut page: Page, store: ThemeStore) -> Result<(), String> {
    page.shell_revision = state
        .page
        .with(|old| {
            let document = |route: &Route| crate::showcase_catalog::find(&route.widget).is_some();
            if old.theme == page.theme
                && ((document(&old.route) && document(&page.route))
                    || (old.route.same_view(&page.route) && old.initial == page.initial))
            {
                Some(old.shell_revision)
            } else {
                old.shell_revision.checked_add(1)
            }
        })
        .ok_or("Navigation shell revision exhausted")?;
    page.revision = state
        .page
        .with(|old| {
            if old.route.same_view(&page.route)
                && old.theme == page.theme
                && old.initial == page.initial
            {
                Some(old.revision)
            } else {
                old.revision.checked_add(1)
            }
        })
        .ok_or("Navigation revision exhausted")?;
    host::install_catalog(&store)?;
    state.catalog.revise(|v| *v = store.themes);
    state.notice.revise(|v| *v = page.notice.clone());
    state.page.revise(|v| *v = page);
    Ok(())
}
