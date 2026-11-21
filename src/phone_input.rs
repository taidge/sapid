//! International telephone drafts and a composite country/number editor.
use crate::composite::{self, Callback};
use crate::state::Value;
use glory::render::{FormControl, Purpose, Semantics};
use glory::ui::{Result, Widget};
use glory::{Scope, Vary};
use phonenumber::{Mode, PhoneNumber, country, metadata::DATABASE};
use std::collections::BTreeMap;
use std::rc::Rc;

/// The selected ISO region and the user's current text, including incomplete edits.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PhoneDraft {
    pub region: String,
    pub text: String,
}

impl Default for PhoneDraft {
    fn default() -> Self {
        Self::new("US", "")
    }
}

impl PhoneDraft {
    pub fn new(region: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            region: region.into(),
            text: text.into(),
        }
    }
    fn number(&self) -> Option<PhoneNumber> {
        if self.text.is_empty() || self.text.len() > 256 {
            return None;
        }
        phonenumber::parse(Some(self.region.parse().ok()?), &self.text)
            .ok()
            .filter(PhoneNumber::is_valid)
    }
    /// E.164 excludes any extension. Incomplete and invalid numbers return None.
    pub fn e164(&self) -> Option<String> {
        self.number()
            .map(|number| number.format().mode(Mode::E164).to_string())
    }
    /// Canonical form value including the extension, when present.
    pub fn rfc3966(&self) -> Option<String> {
        self.number()
            .map(|number| number.format().mode(Mode::Rfc3966).to_string())
    }
}

pub struct PhoneInput {
    key: String,
    value: Value<PhoneDraft>,
    regions: Option<Vec<(String, String)>>,
    label: Vary<String>,
    country_label: Vary<String>,
    theme: crate::Theme,
    size: crate::Size,
    enabled: Vary<bool>,
    readonly: Vary<bool>,
    required: Vary<bool>,
    placeholder: Vary<String>,
    form_name: Option<Vary<String>>,
    changed: Option<Box<dyn FnMut(PhoneDraft)>>,
    committed: Option<Box<dyn FnMut(PhoneDraft)>>,
}

pub fn phone_input(key: impl Into<String>) -> PhoneInput {
    PhoneInput {
        key: key.into(),
        value: Value::default(),
        regions: None,
        label: "Phone number".into(),
        country_label: "Country or region".into(),
        theme: crate::Theme::default(),
        size: crate::Size::default(),
        enabled: true.into(),
        readonly: false.into(),
        required: false.into(),
        placeholder: String::new().into(),
        form_name: None,
        changed: None,
        committed: None,
    }
}

impl PhoneInput {
    pub fn theme(mut self, theme: crate::Theme) -> Self {
        self.theme = theme;
        self
    }
    pub fn size(mut self, size: crate::Size) -> Self {
        self.size = size;
        self
    }
    pub fn label(mut self, label: impl Into<Vary<String>>) -> Self {
        self.label = label.into();
        self
    }
    pub fn country_label(mut self, label: impl Into<Vary<String>>) -> Self {
        self.country_label = label.into();
        self
    }
    pub fn placeholder(mut self, text: impl Into<Vary<String>>) -> Self {
        self.placeholder = text.into();
        self
    }
    pub fn enabled(mut self, enabled: impl Into<Vary<bool>>) -> Self {
        self.enabled = enabled.into();
        self
    }
    pub fn readonly(mut self, readonly: impl Into<Vary<bool>>) -> Self {
        self.readonly = readonly.into();
        self
    }
    pub fn required(mut self, required: impl Into<Vary<bool>>) -> Self {
        self.required = required.into();
        self
    }
    /// Submit the accepted, valid number as an RFC 3966 tel URI.
    pub fn form_name(mut self, name: impl Into<Vary<String>>) -> Self {
        self.form_name = Some(name.into());
        self
    }
    pub fn value(mut self, value: impl Into<Vary<PhoneDraft>>) -> Self {
        self.value.source = Some(value.into());
        self
    }
    pub fn default_value(mut self, value: PhoneDraft) -> Self {
        self.value.default = value;
        self
    }
    /// Limit the selector to the supplied ISO codes and localized labels.
    /// Without a list, all geographic regions in the numbering database are available.
    pub fn regions(
        mut self,
        regions: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
    ) -> Self {
        self.regions = Some(
            regions
                .into_iter()
                .take(257)
                .map(|(id, label)| (id.into(), label.into()))
                .collect(),
        );
        self
    }
    pub fn on_value_change(mut self, callback: impl FnMut(PhoneDraft) + 'static) -> Self {
        self.changed = Some(Box::new(callback));
        self
    }
    pub fn on_commit(mut self, callback: impl FnMut(PhoneDraft) + 'static) -> Self {
        self.committed = Some(Box::new(callback));
        self
    }
}

struct PhoneEditor {
    control: crate::state::Control<PhoneDraft>,
    changed: Callback<PhoneDraft>,
    committed: Callback<PhoneDraft>,
    regions: BTreeMap<String, String>,
    owner: glory::core::InstanceId,
}
impl PhoneEditor {
    fn update(&self, mut draft: PhoneDraft, commit: bool) -> Result {
        if draft.text.len() > 256 || draft.text.contains(['\r', '\n', '\0']) {
            return Ok(());
        }
        if (commit || draft.text.trim_start().starts_with('+'))
            && let Some(number) = draft.number()
        {
            if draft.text.trim_start().starts_with('+')
                && let Some(id) = number.country().id()
                && self.regions.contains_key(id.as_ref())
            {
                draft.region = id.as_ref().to_owned();
            }
            if commit {
                let local = number
                    .country()
                    .id()
                    .is_some_and(|id| id.as_ref() == draft.region);
                draft.text = number
                    .format()
                    .mode(if local {
                        Mode::National
                    } else {
                        Mode::International
                    })
                    .to_string();
            }
        }
        if self.control.source.try_with(|old| old != &draft)? {
            self.control.accept(&draft)?;
            self.changed.emit(self.owner, draft.clone());
        }
        if commit {
            self.committed.emit(self.owner, draft);
        }
        Ok(())
    }
}

impl Widget for PhoneInput {
    fn build(&mut self, scope: &Scope) -> Result {
        let invalid =
            || glory::render::Error::InvalidProps("invalid phone input region, label or draft");
        let labels = self.regions.take().unwrap_or_else(|| {
            DATABASE
                .iter()
                .filter(|meta| meta.id().parse::<country::Id>().is_ok())
                .map(|meta| (meta.id().to_owned(), meta.id().to_owned()))
                .collect()
        });
        if labels.is_empty() || labels.len() > 256 {
            return Err(invalid().into());
        }
        let mut regions = BTreeMap::new();
        for (id, label) in labels {
            let meta = DATABASE.by_id(id.as_str()).ok_or_else(invalid)?;
            if id.parse::<country::Id>().is_err()
                || label.trim().is_empty()
                || label.len() > 220
                || label.contains(['\0', '\r', '\n'])
                || regions
                    .insert(id, format!("{label} (+{})", meta.country_code()))
                    .is_some()
            {
                return Err(invalid().into());
            }
        }
        let editor = Rc::new(PhoneEditor {
            control: std::mem::take(&mut self.value).mount(scope),
            changed: Callback::new(self.changed.take()),
            committed: Callback::new(self.committed.take()),
            regions,
            owner: scope.id(),
        });
        let model = editor.clone();
        let draft = composite::validated(scope, &editor.control.source, move |draft| {
            if !model.regions.contains_key(&draft.region)
                || draft.text.len() > 256
                || draft.text.contains(['\0', '\r', '\n'])
            {
                Err(invalid().into())
            } else {
                Ok(())
            }
        })?;
        let enabled = self.enabled.clone();
        let readonly = self.readonly.clone();
        let choose = editor.clone();
        let weak = scope.downgrade();
        let select = crate::native_select(&self.key)
            .theme(self.theme.clone())
            .size(self.size)
            .label(self.country_label.clone())
            .enabled(scope.bond(move || enabled.get() && !readonly.get()))
            .options(
                editor
                    .regions
                    .iter()
                    .map(|(id, label)| crate::select_option(id, label)),
            )
            .value(scope.bond(move || Some(draft.with(|d| d.region.clone()))))
            .on_value_change(move |id| {
                if let Some(id) = id
                    && let Some(scope) = weak.upgrade()
                {
                    let mut next = choose.control.source.get();
                    if next.region == id {
                        return;
                    }
                    if next.text.trim_start().starts_with('+')
                        && let Some(number) = next.number()
                    {
                        next.text = number.format().mode(Mode::National).to_string();
                    }
                    next.region = id;
                    if let Err(err) = choose.update(next, false) {
                        scope.report_error(err);
                    }
                }
            });
        let edit = |commit: bool| {
            let handler = editor.clone();
            let weak = scope.downgrade();
            move |text: String| {
                if let Some(scope) = weak.upgrade() {
                    let region = handler.control.source.with(|d| d.region.clone());
                    if let Err(err) = handler.update(PhoneDraft::new(region, text), commit) {
                        scope.report_error(err);
                    }
                }
            }
        };
        let input = crate::text_field()
            .theme(self.theme.clone())
            .size(self.size)
            .purpose(Purpose::Tel)
            .placeholder(self.placeholder.clone())
            .enabled(self.enabled.clone())
            .readonly(self.readonly.clone())
            .required(self.required.clone())
            .value(scope.bond(move || draft.with(|d| d.text.clone())))
            .on_value_change(edit(false))
            .on_commit(edit(true));
        let mut input = Some(input);
        let field = crate::field(&self.key)
            .theme(self.theme.clone())
            .size(self.size)
            .label(self.label.clone())
            .control(move |field| input.take().expect("mount once").field(field));
        crate::group()
            .theme(self.theme.clone())
            .fill((select, field))
            .show_in(scope)?;
        if let Some(name) = composite::form_name(scope, &self.form_name)? {
            let value = scope.bond(move || draft.with(PhoneDraft::rfc3966));
            let enabled = self.enabled.clone();
            glory::ui::view()
                .semantics(Some(Box::new(Semantics {
                    hidden: true,
                    ..Default::default()
                })))
                .fill(
                    glory::ui::input()
                        .value(scope.bond(move || value.get().unwrap_or_default()))
                        .enabled(scope.bond(move || enabled.get() && value.with(Option::is_some)))
                        .semantics(scope.bond(move || {
                            Some(Box::new(Semantics {
                                form_control: Some(FormControl {
                                    name: name.get(),
                                    value: None,
                                }),
                                ..Default::default()
                            }))
                        })),
                )
                .show_in(scope)?;
        }
        Ok(())
    }
}
