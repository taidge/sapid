//! In-process route and fragment navigation for the native showcase window.
use super::document::{publish, resolve};
use crate::showcase_navigation::{Route, State};
use glory::{
    prelude::*,
    render::{Action, SemanticId},
    routing::{Location, RouterHandle},
    wry::shell::WindowApp,
};
use std::cell::RefCell;

struct Running {
    state: State,
    router: RouterHandle,
    scroll_target: Option<String>,
}

thread_local! { static APP: RefCell<Option<Running>> = const { RefCell::new(None) }; }

pub fn site(theme: sapid::Theme, selection: String) -> impl Widget {
    let initial = Route::new(selection.clone(), &theme).location();
    crate::showcase::site_with_router(theme, selection, initial, |scope, state, router| {
        APP.with(|slot| {
            *slot.borrow_mut() = Some(Running {
                state,
                router,
                scroll_target: None,
            })
        });
        scope.on_cleanup(|| {
            APP.with(|slot| {
                slot.borrow_mut().take();
            })
        })?;
        Ok(())
    })
}

pub(super) fn report(message: String) {
    if let Some(state) = APP.with(|slot| slot.borrow().as_ref().map(|app| app.state)) {
        state.notice.revise(|v| *v = message);
    }
}

pub(super) fn navigate(href: &str) -> std::result::Result<(), String> {
    let (state, router) = APP
        .with(|slot| {
            slot.borrow()
                .as_ref()
                .map(|app| (app.state, app.router.clone()))
        })
        .ok_or("Showcase is not mounted")?;
    let previous = state.page.get();
    let requested = if let Some(fragment) = href.strip_prefix('#') {
        previous.route.location().with_fragment(fragment)
    } else if href.starts_with("showcase.html?") {
        Location::parse(&format!("/{href}")).map_err(|error| error.to_string())?
    } else {
        return Err("Only showcase routes use local navigation".into());
    };
    let route = Route::from_location(&requested);
    let (page, store) = resolve(route)?;
    router
        .push(page.route.canonical_location(&requested))
        .map_err(|error| error.to_string())?;
    let target = if !page.route.fragment.is_empty() {
        Some(page.route.fragment.clone())
    } else if previous.route.widget != page.route.widget {
        Some("site-main".to_owned())
    } else {
        None
    };
    APP.with(|slot| {
        if let Some(app) = slot.borrow_mut().as_mut() {
            app.scroll_target = target;
        }
    });
    publish(state, page, store)
}

/// Resolve against the settled Rust tree, after a route may have replaced the article.
pub fn finish_navigation(window: &WindowApp) -> glory::ui::Result {
    let target = APP.with(|slot| {
        slot.borrow_mut()
            .as_mut()
            .and_then(|app| app.scroll_target.take())
    });
    let id = target
        .and_then(|target| SemanticId::new(target).ok())
        .and_then(|name| window.app().tree().named(&name));
    if let Some(id) = id {
        window.app().scope().action(Action::ScrollIntoView(id))?;
    }
    Ok(())
}
