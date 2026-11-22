//! Bounded, live-target guided steps using genuine anchored Dialog layers.
use crate::{
    appearance::Common,
    composite::{self, Callback},
    state::{Control, Value},
};
use glory::render::{
    Action, AnchorPlacement, DismissReason, Event, EventKind, FocusMode, Layer, LayerPlacement,
    NodeId, Query, QueryToken, QueryValue, Role, SemanticId, Semantics, TextKind,
};
use glory::style::{Edges, Length, Style};
use glory::ui::{Result, WeakScope, Widget};
use glory::{Cage, Scope, Vary};
use std::{
    cell::{Cell, RefCell},
    collections::{BTreeMap, BTreeSet},
    rc::{Rc, Weak},
};

pub const MAX_TOUR_STEPS: usize = 64;
fn invalid(message: &'static str) -> glory::ui::Error {
    glory::render::Error::InvalidProps(message).into()
}
fn label(value: &str) -> Result {
    if value.len() > 1024 || value.trim().is_empty() || value.contains(['\0', '\r']) {
        return Err(invalid("Tour text requires 1..=1024 canonical bytes"));
    }
    Ok(())
}
#[derive(Clone)]
struct Entry {
    node: Option<NodeId>,
    owner: glory::core::InstanceId,
}
struct Registry {
    key: String,
    spotlight: bool,
    owner: glory::core::InstanceId,
    pulse: Cage<()>,
    entries: RefCell<BTreeMap<String, Entry>>,
    attached: Cell<bool>,
}
/// One application-owned namespace containing only its currently mounted targets.
#[derive(Clone)]
pub struct TourContext(Rc<Registry>);
pub fn tour_context(scope: &Scope, key: impl Into<String>) -> Result<TourContext> {
    TourContext::create(scope, key.into(), false)
}
impl TourContext {
    /// Create a tour that highlights each anchored step in a viewport backdrop.
    /// Hosts without CSS anchor positioning retain the ordinary anchored dialog.
    pub fn with_spotlight(scope: &Scope, key: impl Into<String>) -> Result<Self> {
        Self::create(scope, key.into(), true)
    }

    fn target_id(&self, key: &str) -> Result<SemanticId> {
        let id = composite::id("tour", &self.0.key, "target", key)?;
        if self.0.spotlight {
            Ok(SemanticId::new(format!("--{}", id.as_str()))?)
        } else {
            Ok(id)
        }
    }

    fn create(scope: &Scope, key: String, spotlight: bool) -> Result<Self> {
        composite::id("tour", &key, "context", "root")?;
        let state = Rc::new(Registry {
            key,
            spotlight,
            owner: scope.id(),
            pulse: scope.cage(()),
            entries: RefCell::new(BTreeMap::new()),
            attached: Cell::new(false),
        });
        let weak = Rc::downgrade(&state);
        scope.on_cleanup(move || {
            if let Some(state) = weak.upgrade() {
                let entries = std::mem::take(&mut *state.entries.borrow_mut());
                drop(entries);
            }
        })?;
        Ok(TourContext(state))
    }

    pub fn len(&self) -> usize {
        self.0.entries.borrow().len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn ensure(&self, scope: &Scope) -> Result {
        if !self.0.owner.is_alive() {
            return Err(glory::ui::Error::Gone);
        }
        if self.0.owner.runtime() != scope.id().runtime() {
            return Err(glory::render::Error::ForeignRuntime.into());
        }
        Ok(())
    }
    fn changed(&self) -> Result {
        if self.0.owner.is_alive() {
            self.0.pulse.try_revise(|_| ())?;
        }
        Ok(())
    }
}
pub struct TourTarget {
    context: TourContext,
    key: String,
    child: Option<Box<dyn Widget>>,
}
pub fn tour_target(context: &TourContext, key: impl Into<String>) -> TourTarget {
    TourTarget {
        context: context.clone(),
        key: key.into(),
        child: None,
    }
}
impl TourTarget {
    pub fn fill(mut self, child: impl Widget) -> Self {
        self.child = Some(Box::new(child));
        self
    }
}
impl Widget for TourTarget {
    fn build(&mut self, scope: &Scope) -> Result {
        self.context.ensure(scope)?;
        let id = self.context.target_id(&self.key)?;
        {
            let mut entries = self.context.0.entries.borrow_mut();
            if entries.contains_key(&self.key) {
                return Err(invalid("duplicate live Tour target"));
            }
            if entries.len() >= MAX_TOUR_STEPS {
                return Err(invalid("Tour live target budget exceeded"));
            }
            entries.insert(
                self.key.clone(),
                Entry {
                    node: None,
                    owner: scope.id(),
                },
            );
        }
        let ctx = self.context.clone();
        let key = self.key.clone();
        let owner = scope.id();
        scope.on_cleanup(move || {
            let removed = {
                let mut entries = ctx.0.entries.borrow_mut();
                if entries.get(&key).is_some_and(|e| e.owner == owner) {
                    entries.remove(&key)
                } else {
                    None
                }
            };
            if removed.is_some() {
                let _ = ctx.changed();
            }
        })?;
        let reference = scope.cage(None);
        let mut target = glory::ui::view()
            .class("sapid-tour-target")
            .focus(FocusMode::Programmatic)
            .node_ref(reference)
            .semantics(composite::metadata(Semantics {
                id: Some(id.clone()),
                ..Default::default()
            }));
        let ctx = self.context.clone();
        let weak = scope.downgrade();
        target = target.on(EventKind::Resize, move |_| {
            if let Err(error) = ctx.changed()
                && let Some(scope) = weak.upgrade()
            {
                scope.report_error(error);
            }
        });
        if let Some(child) = self.child.take() {
            target = target.fill(child);
        }
        target.show_in(scope)?;
        let node = reference
            .try_get()?
            .ok_or_else(|| invalid("Tour target failed to mount"))?;
        self.context
            .0
            .entries
            .borrow_mut()
            .get_mut(&self.key)
            .expect("reserved Tour target")
            .node = Some(node);
        self.context.changed()?;
        Ok(())
    }
}
/// A static declaration references one current target key, never an unmounted row.
#[derive(Clone, Debug)]
pub struct TourStep {
    pub key: String,
    pub title: String,
    pub description: String,
}
pub fn tour_step(
    key: impl Into<String>,
    title: impl Into<String>,
    description: impl Into<String>,
) -> TourStep {
    TourStep {
        key: key.into(),
        title: title.into(),
        description: description.into(),
    }
}
struct State {
    owner: glory::core::InstanceId,
    weak: WeakScope,
    context: TourContext,
    steps: Vec<TourStep>,
    targets: BTreeMap<String, Option<String>>,
    current: Control<usize>,
    input_current: Vary<usize>,
    open: Control<bool>,
    visible: Cage<bool>,
    changed: Callback<usize>,
    opened: Callback<bool>,
    cancelled: Callback<DismissReason>,
    lost: Callback<usize>,
    complete: Callback<()>,
    task: RefCell<Option<glory::core::TaskToken>>,
    pending: Cell<Option<QueryToken>>,
    ticket: Cell<u64>,
    loss: Cell<bool>,
    shown: Cell<Option<(usize, NodeId)>>,
}
impl State {
    fn index(&self) -> Result<usize> {
        let index = self.input_current.try_get()?;
        if index >= self.steps.len() {
            return Err(invalid("Tour current step is outside its declaration"));
        }
        Ok(index)
    }
    fn target_key(&self, index: usize) -> Option<&str> {
        let key = &self.steps[index].key;
        match self.targets.get(key) {
            Some(value) => value.as_deref(),
            None => Some(key),
        }
    }
    fn entry(&self) -> Result<Option<Entry>> {
        let Some(key) = self.target_key(self.index()?) else {
            return Ok(None);
        };
        Ok(self
            .context
            .0
            .entries
            .borrow()
            .get(key)
            .filter(|entry| entry.node.is_some())
            .cloned())
    }
    fn ensure(&self) -> Result {
        if self.owner.is_alive() {
            Ok(())
        } else {
            Err(glory::ui::Error::Gone)
        }
    }
    fn report(&self, result: Result) {
        if let Err(error) = result
            && let Some(scope) = self.weak.upgrade()
        {
            scope.report_error(error);
        }
    }
    fn cancel_work(&self) {
        let task = self.task.borrow_mut().take();
        if let Some(task) = task {
            task.cancel();
        }
        if let Some(token) = self.pending.take()
            && let Some(scope) = self.weak.upgrade()
        {
            let _ = scope.cancel_request(token);
        }
    }
    fn hide(&self) -> Result {
        self.shown.set(None);
        if self.visible.try_get()? {
            self.visible.try_revise(|visible| *visible = false)?;
        }
        Ok(())
    }
    fn missing(&self) -> Result {
        self.hide()?;
        if !self.loss.replace(true) {
            self.lost.emit(self.owner, self.index()?);
        }
        Ok(())
    }
    fn refresh(self: &Rc<Self>) -> Result {
        self.ensure()?;
        self.cancel_work();
        let current = self.index()?;
        if self.target_key(current).is_none() {
            self.shown.set(None);
            self.loss.set(false);
            let visible = self.open.source.try_get()?;
            if self.visible.try_get()? != visible {
                self.visible.try_revise(|v| *v = visible)?;
            }
            return Ok(());
        }
        if self.shown.get()
            != self
                .entry()?
                .and_then(|entry| entry.node.map(|node| (current, node)))
        {
            self.hide()?;
        }
        let ticket = self
            .ticket
            .get()
            .checked_add(1)
            .ok_or_else(|| invalid("Tour request identity exhausted"))?;
        self.ticket.set(ticket);
        if !self.open.source.try_get()? {
            self.hide()?;
            self.loss.set(false);
            return Ok(());
        }
        let Some(scope) = self.weak.upgrade() else {
            return Ok(());
        };
        let caps = scope.capabilities()?;
        // Deterministic construction/adoption: first observation always runs in a ready host task.
        if !caps.measure || !caps.scroll {
            return Ok(());
        }
        let weak = Rc::downgrade(self);
        let task = scope.spawn(async move {
            if let Some(state) = weak.upgrade() {
                let _finished = state.task.borrow_mut().take();
                state.report(state.measure(ticket));
            }
        })?;
        *self.task.borrow_mut() = Some(task);
        Ok(())
    }
    fn measure(self: &Rc<Self>, ticket: u64) -> Result {
        if self.ticket.get() != ticket || !self.open.source.try_get()? {
            return Ok(());
        }
        let Some(entry) = self.entry()? else {
            return self.missing();
        };
        let node = entry.node.expect("mounted Tour target");
        let Some(scope) = self.weak.upgrade() else {
            return Ok(());
        };
        if !entry.owner.is_alive() || !scope.is_active(node)? || scope.is_inert(node)? {
            return self.missing();
        }
        scope.action(Action::ScrollIntoView(node))?;
        let weak = Rc::downgrade(self);
        let index = self.index()?;
        let token = scope.request(Query::Bounds(node), move |reply| {
            let Some(state) = weak.upgrade() else { return; };
            if !state.owner.is_alive() || state.ticket.get() != ticket { return; }
            state.pending.set(None);
            state.report((|| {
                if !state.open.source.try_get()? || state.index()? != index || state.entry()?.is_none_or(|e| e.node != entry.node) { return Ok(()); }
                let Some(scope) = state.weak.upgrade() else { return Ok(()); };
                let visible = matches!(reply, Ok(QueryValue::Bounds(rect)) if rect.width > 0. && rect.height > 0.)
                    && scope.is_active(node)? && !scope.is_inert(node)?;
                if visible {
                    state.loss.set(false);
                    state.shown.set(Some((index, node)));
                    if !state.visible.try_get()? { state.visible.try_revise(|value| *value = true)?; }
                    Ok(())
                }
                else { state.missing() }
            })());
        })?;
        self.pending.set(Some(token));
        Ok(())
    }
    fn select(self: &Rc<Self>, index: usize) -> Result {
        self.ensure()?;
        if index >= self.steps.len() {
            return Err(invalid("Tour step is outside its declaration"));
        }
        if !self.open.source.try_get()? {
            return Ok(());
        }
        if self.index()? != index {
            self.current.accept(&index)?;
            self.changed.emit(self.owner, index);
        }
        Ok(())
    }
    fn close(&self, reason: Option<DismissReason>, finished: bool) -> Result {
        self.ensure()?;
        if !self.open.source.try_get()? {
            return Ok(());
        }
        self.open.accept(&false)?;
        if let Some(reason) = reason {
            self.cancelled.emit(self.owner, reason);
        }
        if finished {
            self.complete.emit(self.owner, ());
        }
        self.opened.emit(self.owner, false);
        Ok(())
    }
}
#[derive(Clone)]
pub struct TourHandle(Weak<State>);
impl TourHandle {
    fn state(&self) -> Result<Rc<State>> {
        self.0
            .upgrade()
            .filter(|state| state.owner.is_alive())
            .ok_or(glory::ui::Error::Gone)
    }
    pub fn refresh(&self) -> Result {
        self.state()?.refresh()
    }
    pub fn go_to(&self, index: usize) -> Result {
        self.state()?.select(index)
    }
    pub fn next(&self) -> Result {
        let state = self.state()?;
        let index = state.index()?;
        if index + 1 == state.steps.len() {
            state.close(None, true)
        } else {
            state.select(index + 1)
        }
    }
    pub fn previous(&self) -> Result {
        let state = self.state()?;
        state.select(state.index()?.saturating_sub(1))
    }
    pub fn cancel(&self) -> Result {
        self.state()?.close(Some(DismissReason::Gesture), false)
    }
}
pub struct Tour {
    key: String,
    context: TourContext,
    common: Common,
    steps: Vec<TourStep>,
    targets: BTreeMap<String, Option<String>>,
    placements: BTreeMap<String, AnchorPlacement>,
    over_target: BTreeSet<String>,
    previous_label: String,
    next_label: String,
    finish_label: String,
    cancel_label: String,
    counter: Rc<dyn Fn(usize, usize) -> String>,
    overflow: bool,
    current: Value<usize>,
    open: Value<bool>,
    placement: AnchorPlacement,
    escape: Vary<bool>,
    outside: Vary<bool>,
    show_cancel: Vary<bool>,
    compact_navigation: bool,
    changed: Option<Box<dyn FnMut(usize)>>,
    opened: Option<Box<dyn FnMut(bool)>>,
    cancelled: Option<Box<dyn FnMut(DismissReason)>>,
    lost: Option<Box<dyn FnMut(usize)>>,
    complete: Option<Box<dyn FnMut(())>>,
    ready: Option<Box<dyn FnMut(TourHandle)>>,
}
pub fn tour(context: &TourContext, key: impl Into<String>) -> Tour {
    Tour {
        key: key.into(),
        context: context.clone(),
        common: Common::default(),
        steps: Vec::new(),
        targets: BTreeMap::new(),
        placements: BTreeMap::new(),
        over_target: BTreeSet::new(),
        previous_label: "Previous step".into(),
        next_label: "Next step".into(),
        finish_label: "Finish".into(),
        cancel_label: "Cancel tour".into(),
        counter: Rc::new(|current, total| format!("Step {current} of {total}")),
        overflow: false,
        current: Value::default(),
        open: Value::default(),
        placement: AnchorPlacement::default(),
        escape: true.into(),
        outside: false.into(),
        show_cancel: true.into(),
        compact_navigation: false,
        changed: None,
        opened: None,
        cancelled: None,
        lost: None,
        complete: None,
        ready: None,
    }
}
impl Tour {
    /// Place the progress count and compact navigation controls in one footer.
    pub fn compact_navigation(mut self, value: bool) -> Self {
        self.compact_navigation = value;
        self
    }
    pub fn close_on_escape(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.escape = value.into();
        self
    }
    pub fn close_on_outside_press(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.outside = value.into();
        self
    }
    pub fn with_close_button(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.show_cancel = value.into();
        self
    }
    pub fn step(mut self, step: TourStep) -> Self {
        if self.steps.len() < MAX_TOUR_STEPS
            && step.key.len() <= 64
            && SemanticId::new(&step.key).is_ok()
            && label(&step.title).is_ok()
            && label(&step.description).is_ok()
        {
            self.steps.push(step);
        } else {
            self.overflow = true;
        }
        self
    }
    /// Display this step in the viewport center without registering a target.
    pub fn centered_step(
        mut self,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        let key = format!("centered-{}", self.steps.len());
        self.targets.insert(key.clone(), None);
        self.step(tour_step(key, title, description))
    }

    /// Give a step its own identity and position while reusing a registered target.
    pub fn anchored_step(
        mut self,
        step: TourStep,
        target: impl Into<String>,
        placement: AnchorPlacement,
    ) -> Self {
        let target = target.into();
        if target.len() > 64 || SemanticId::new(&target).is_err() || placement.validate().is_err() {
            self.overflow = true;
            return self;
        }
        self.targets.insert(step.key.clone(), Some(target));
        self.placements.insert(step.key.clone(), placement);
        self.step(step)
    }

    /// Center this step over its target, keeping the panel inside the viewport.
    pub fn over_target_step(mut self, step: TourStep, target: impl Into<String>) -> Self {
        self.over_target.insert(step.key.clone());
        self.anchored_step(
            step,
            target,
            AnchorPlacement::new(
                glory::render::AnchorSide::Bottom,
                glory::render::AnchorAlign::Center,
            ),
        )
    }

    pub fn navigation_labels(
        mut self,
        previous: impl Into<String>,
        next: impl Into<String>,
        finish: impl Into<String>,
        cancel: impl Into<String>,
    ) -> Self {
        self.previous_label = previous.into();
        self.next_label = next.into();
        self.finish_label = finish.into();
        self.cancel_label = cancel.into();
        self
    }

    pub fn step_counter(mut self, label: impl Fn(usize, usize) -> String + 'static) -> Self {
        self.counter = Rc::new(label);
        self
    }

    pub fn steps(mut self, steps: impl IntoIterator<Item = TourStep>) -> Self {
        for step in steps {
            if self.steps.len() == MAX_TOUR_STEPS {
                self.overflow = true;
                break;
            }
            self = self.step(step);
            if self.overflow {
                break;
            }
        }
        self
    }
    pub fn current(mut self, value: impl Into<Vary<usize>>) -> Self {
        self.current.source = Some(value.into());
        self
    }
    pub fn default_current(mut self, value: usize) -> Self {
        self.current.default = value;
        self
    }
    pub fn open(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.open.source = Some(value.into());
        self
    }
    pub fn default_open(mut self, value: bool) -> Self {
        self.open.default = value;
        self
    }
    pub fn placement(mut self, value: AnchorPlacement) -> Self {
        self.placement = value;
        self
    }
    pub fn on_current_change(mut self, value: impl FnMut(usize) + 'static) -> Self {
        self.changed = Some(Box::new(value));
        self
    }
    pub fn on_open_change(mut self, value: impl FnMut(bool) + 'static) -> Self {
        self.opened = Some(Box::new(value));
        self
    }
    pub fn on_cancel(mut self, value: impl FnMut(DismissReason) + 'static) -> Self {
        self.cancelled = Some(Box::new(value));
        self
    }
    pub fn on_target_lost(mut self, value: impl FnMut(usize) + 'static) -> Self {
        self.lost = Some(Box::new(value));
        self
    }
    pub fn on_complete(mut self, mut value: impl FnMut() + 'static) -> Self {
        self.complete = Some(Box::new(move |_| value()));
        self
    }
    pub fn on_ready(mut self, value: impl FnMut(TourHandle) + 'static) -> Self {
        self.ready = Some(Box::new(value));
        self
    }
    pub fn theme(mut self, value: crate::Theme) -> Self {
        self.common.theme = value;
        self
    }
    pub fn unstyled(mut self) -> Self {
        self.common.unstyled = true;
        self
    }
}
impl Widget for Tour {
    fn build(&mut self, scope: &Scope) -> Result {
        self.context.ensure(scope)?;
        if self.overflow || self.steps.is_empty() {
            return Err(invalid("Tour requires 1..=64 steps"));
        }
        composite::validate_keys(&self.steps.iter().map(|s| s.key.clone()).collect::<Vec<_>>())?;
        for step in &self.steps {
            label(&step.title)?;
            label(&step.description)?;
        }
        self.placement.validate()?;
        let id = composite::id("tour", &self.key, "panel", "root")?;
        let title_id = composite::id("tour", &self.key, "title", "root")?;
        let desc_id = composite::id("tour", &self.key, "description", "root")?;
        let count = self.steps.len();
        let mut current = std::mem::take(&mut self.current).mount(scope);
        let input_current = current.source.clone();
        current.source = composite::validated(scope, &current.source, move |index| {
            if *index >= count {
                Err(invalid("Tour current step is outside its declaration"))
            } else {
                Ok(())
            }
        })?
        .into();
        let open = std::mem::take(&mut self.open).mount(scope);
        if self.context.0.attached.replace(true) {
            return Err(invalid("Tour context already has a live controller"));
        }
        let state = Rc::new(State {
            owner: scope.id(),
            weak: scope.downgrade(),
            context: self.context.clone(),
            steps: std::mem::take(&mut self.steps),
            targets: std::mem::take(&mut self.targets),
            current,
            input_current,
            open,
            visible: scope.cage(false),
            changed: Callback::new(self.changed.take()),
            opened: Callback::new(self.opened.take()),
            cancelled: Callback::new(self.cancelled.take()),
            lost: Callback::new(self.lost.take()),
            complete: Callback::new(self.complete.take()),
            task: RefCell::new(None),
            pending: Cell::new(None),
            ticket: Cell::new(0),
            loss: Cell::new(false),
            shown: Cell::new(None),
        });
        let weak = Rc::downgrade(&state);
        let ctx = self.context.clone();
        scope.on_cleanup(move || {
            ctx.0.attached.set(false);
            if let Some(state) = weak.upgrade() {
                state.cancel_work();
            }
        })?;
        let s = state.clone();
        let placement = self.placement;
        let placements = std::mem::take(&mut self.placements);
        let outside = self.outside.clone();
        let escape = self.escape.clone();
        let labelled = title_id.clone();
        let described = desc_id.clone();
        let over_target = std::mem::take(&mut self.over_target);
        let styled = state.clone();
        let spotlight = self.context.0.spotlight;
        let compact = self.compact_navigation;
        let classes = scope.bond(move || {
            let mut classes = "sapid-tour sapid-layer".to_owned();
            if compact {
                classes.push_str(" sapid-tour-compact");
            }
            if spotlight {
                classes.push_str(" sapid-tour-spotlight");
            }
            if over_target.contains(&styled.steps[styled.current.source.get()].key) {
                classes.push_str(" sapid-tour-over-target");
            }
            classes
        });
        let mut panel = glory::ui::view()
            .class(classes)
            .role(Role::Dialog)
            .focus(FocusMode::Programmatic)
            .semantics(scope.bond(move || {
                s.context.0.pulse.get();
                let index = s.current.source.get();
                let target = s.target_key(index);
                let anchor =
                    target.map(|key| s.context.target_id(key).expect("validated Tour target key"));
                let detached = target.is_some_and(|key| {
                    s.context
                        .0
                        .entries
                        .borrow()
                        .get(key)
                        .is_none_or(|e| e.node.is_none())
                });
                let position = if target.is_some() {
                    LayerPlacement::Anchor(
                        placements
                            .get(&s.steps[index].key)
                            .copied()
                            .unwrap_or(placement),
                    )
                } else {
                    LayerPlacement::Center
                };
                composite::metadata(Semantics {
                    id: Some(id.clone()),
                    labelled_by: Some(labelled.clone()),
                    described_by: Some(described.clone()),
                    hidden: !s.open.source.get() || !s.visible.get() || detached,
                    layer: Some(Layer {
                        modal: false,
                        outside: outside.get(),
                        escape: escape.get(),
                        placement: position,
                        anchor: anchor.clone(),
                        return_focus: anchor,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
            }));
        let s = state.clone();
        panel = panel.on(EventKind::Dismiss, move |event| {
            if let Event::Dismiss(reason) = event {
                if reason == DismissReason::AnchorLost {
                    s.report(s.missing());
                } else {
                    s.report(s.close(Some(reason), false));
                }
            }
        });
        let mut style = Style::new()
            .width(Length::Px(320.))
            .padding(Edges::all(16.))
            .display(glory::style::Display::Flex)
            .flex_direction(glory::style::FlexDirection::Column)
            .gap(12.);
        if !self.common.unstyled {
            style = crate::appearance::merge(
                self.common
                    .theme
                    .description(self.common.size)
                    .font_weight(400.),
                style,
            );
            style.radius = Some(self.common.theme.radius);
            style.background = Some(self.common.theme.surface);
            style.color = Some(self.common.theme.foreground);
        }
        panel = panel.style(style);
        let s = state.clone();
        let title = scope.bond(move || s.steps[s.current.source.get()].title.clone());
        let s = state.clone();
        let description = scope.bond(move || s.steps[s.current.source.get()].description.clone());
        let s = state.clone();
        let counter = self.counter.clone();
        let progress = scope.bond(move || counter(s.current.source.get() + 1, s.steps.len()));
        let h = TourHandle(Rc::downgrade(&state));
        let previous = h.clone();
        let next = h.clone();
        let cancel = h.clone();
        let s = state.clone();
        let previous_enabled = scope.bond(move || s.current.source.get() > 0);
        let s = state.clone();
        let next_word = self.next_label.clone();
        let finish_word = self.finish_label.clone();
        let next_label = scope.bond(move || {
            if s.current.source.get() + 1 == s.steps.len() {
                finish_word.clone()
            } else {
                next_word.clone()
            }
        });
        let weak = scope.downgrade();
        let previous_scope = weak.clone();
        let next_scope = weak.clone();
        let mut previous_button = crate::button()
            .theme(self.common.theme.clone())
            .size(crate::Size::Small)
            .variant(crate::ButtonVariant::Outline)
            .label(self.previous_label.clone())
            .enabled(previous_enabled)
            .on_press(move || {
                if let Err(error) = previous.previous()
                    && let Some(scope) = previous_scope.upgrade()
                {
                    scope.report_error(error);
                }
            });
        let mut next_button = crate::button()
            .theme(self.common.theme.clone())
            .size(crate::Size::Small)
            .label(next_label)
            .on_press(move || {
                if let Err(error) = next.next()
                    && let Some(scope) = next_scope.upgrade()
                {
                    scope.report_error(error);
                }
            });
        let mut cancel_button = crate::button()
            .theme(self.common.theme.clone())
            .style({
                let visible = self.show_cancel.clone();
                scope.bond(move || {
                    if visible.get() {
                        Style::new()
                    } else {
                        Style::new().display(glory::style::Display::None)
                    }
                })
            })
            .size(crate::Size::Small)
            .variant(crate::ButtonVariant::Subtle)
            .label(self.cancel_label.clone())
            .on_press(move || {
                if let Err(error) = cancel.cancel()
                    && let Some(scope) = weak.upgrade()
                {
                    scope.report_error(error);
                }
            });
        let mut title_style = Style::new().margin(Edges::all(0.));
        let mut description_style = Style::new();
        let mut progress_style = Style::new();
        if self.common.unstyled {
            previous_button = previous_button.unstyled();
            next_button = next_button.unstyled();
            cancel_button = cancel_button.unstyled();
        } else {
            title_style = crate::appearance::merge(
                self.common.theme.label(self.common.size).font_weight(600.),
                title_style,
            );
            description_style = self.common.theme.description(self.common.size);
            progress_style = self
                .common
                .theme
                .description(crate::Size::Small)
                .font_weight(400.);
        }
        if compact {
            let close = h.clone();
            let scope_ref = scope.downgrade();
            let close_button = crate::close_button()
                .theme(self.common.theme.clone())
                .name("Close tour")
                .on_press(move || {
                    if let Err(error) = close.cancel()
                        && let Some(scope) = scope_ref.upgrade()
                    {
                        scope.report_error(error);
                    }
                });
            panel = panel.fill(close_button);
        }
        panel
            .fill((
                glory::ui::text()
                    .class("sapid-tour-title")
                    .kind(TextKind::Heading(2))
                    .content(title)
                    .style(title_style)
                    .semantics(composite::metadata(Semantics {
                        id: Some(title_id),
                        ..Default::default()
                    })),
                glory::ui::text()
                    .class("sapid-tour-description")
                    .content(description)
                    .style(description_style)
                    .semantics(composite::metadata(Semantics {
                        id: Some(desc_id),
                        ..Default::default()
                    })),
                glory::ui::text()
                    .class("sapid-tour-progress")
                    .content(progress)
                    .style(progress_style),
                glory::ui::view()
                    .class("sapid-tour-navigation")
                    .style(Style {
                        justify: Some(glory::style::Justify::End),
                        align: Some(glory::style::Align::Center),
                        ..Style::new()
                            .display(glory::style::Display::Flex)
                            .flex_wrap(glory::style::FlexWrap::Wrap)
                            .gap(8.)
                            .min_width(Length::Px(0.))
                    })
                    .fill((previous_button, next_button, cancel_button)),
            ))
            .show_in(scope)?;
        let s = state.clone();
        scope.effect(move || {
            s.context.0.pulse.get();
            s.current.source.get();
            s.open.source.get();
            glory::core::untracked(|| s.report(s.refresh()));
        });
        if let Some(mut ready) = self.ready.take() {
            ready(h);
        }
        scope.retain(state)?;
        Ok(())
    }
}
