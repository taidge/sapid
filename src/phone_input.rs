//! International telephone drafts and a composite country/number editor.
use crate::composite::{self, Callback};
use crate::state::Value;
use glory::render::{FormControl, Purpose, Semantics};
use glory::style::{Align, Display, FlexWrap, Length, PointerEvents, Position, Style};
use glory::ui::{Result, Widget};
use glory::{Scope, Vary};
use phonenumber::{Mode, PhoneNumber, country, metadata::DATABASE};
use std::collections::BTreeSet;
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
    searchable_region: bool,
    region_open: bool,
    label: Vary<String>,
    country_label: Vary<String>,
    theme: crate::Theme,
    size: crate::Size,
    editor_style: Vary<glory::style::Style>,
    enabled: Vary<bool>,
    readonly: Vary<bool>,
    required: Vary<bool>,
    placeholder: Vary<String>,
    field: Option<crate::FieldControl>,
    loading: Vary<bool>,
    editor_ref: Option<glory::Cage<Option<glory::render::NodeId>>>,
    form_name: Option<Vary<String>>,
    changed: Option<Box<dyn FnMut(PhoneDraft)>>,
    committed: Option<Box<dyn FnMut(PhoneDraft)>>,
}

pub fn phone_input(key: impl Into<String>) -> PhoneInput {
    PhoneInput {
        key: key.into(),
        value: Value::default(),
        regions: None,
        searchable_region: false,
        region_open: false,
        label: "Phone number".into(),
        country_label: "Country or region".into(),
        theme: crate::Theme::default(),
        size: crate::Size::default(),
        editor_style: glory::style::Style::new().into(),
        enabled: true.into(),
        readonly: false.into(),
        required: false.into(),
        placeholder: String::new().into(),
        field: None,
        loading: false.into(),
        editor_ref: None,
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
    /// Apply appearance overrides to the telephone editor.
    pub fn editor_style(mut self, style: impl Into<Vary<glory::style::Style>>) -> Self {
        self.editor_style = style.into();
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
    /// Use the enclosing Field's label, errors, availability, and submission name.
    pub fn field(mut self, field: crate::FieldControl) -> Self {
        self.theme = field.theme.clone();
        self.size = field.size;
        self.field = Some(field);
        self
    }
    pub fn loading(mut self, loading: impl Into<Vary<bool>>) -> Self {
        self.loading = loading.into();
        self
    }
    /// Refers to the editable telephone input.
    pub fn node_ref(mut self, reference: glory::Cage<Option<glory::render::NodeId>>) -> Self {
        self.editor_ref = Some(reference);
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
    /// Offer a searchable country list in place of the platform-native menu.
    pub fn searchable_region(mut self, value: bool) -> Self {
        self.searchable_region = value;
        self
    }

    /// Open the searchable country menu on mount.
    pub fn default_region_open(mut self, value: bool) -> Self {
        self.region_open = value;
        self
    }

    /// Display the supplied ISO codes and localized labels in their declared order.
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
    regions: BTreeSet<String>,
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
                && self.regions.contains(id.as_ref())
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
            let mut labels = DATABASE
                .iter()
                .filter(|meta| meta.id().parse::<country::Id>().is_ok())
                .map(|meta| (meta.id().to_owned(), meta.id().to_owned()))
                .collect::<Vec<_>>();
            labels.sort_unstable();
            labels
        });
        if labels.is_empty() || labels.len() > 256 {
            return Err(invalid().into());
        }
        let mut regions = BTreeSet::new();
        let mut options = Vec::with_capacity(labels.len());
        let mut searchable_options = Vec::new();
        for (id, label) in labels {
            let meta = DATABASE.by_id(id.as_str()).ok_or_else(invalid)?;
            if id.parse::<country::Id>().is_err()
                || label.trim().is_empty()
                || label.len() > 220
                || label.contains(['\0', '\r', '\n'])
                || !regions.insert(id.clone())
            {
                return Err(invalid().into());
            }
            if self.searchable_region {
                searchable_options.push((
                    id.clone(),
                    label.clone(),
                    format!("+{}", meta.country_code()),
                ));
            }
            options.push(crate::select_option(
                id,
                format!("{label} (+{})", meta.country_code()),
            ));
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
            if !model.regions.contains(&draft.region)
                || draft.text.len() > 256
                || draft.text.contains(['\0', '\r', '\n'])
            {
                Err(invalid().into())
            } else {
                Ok(())
            }
        })?;
        let mut field = self.field.take();
        let field_name = field.as_mut().and_then(crate::FieldControl::take_form_name);
        let form_name = composite::form_name(scope, &self.form_name.take().or(field_name))?;
        let enabled = field
            .as_ref()
            .map(|f| f.enabled(scope, self.enabled.clone()))
            .unwrap_or_else(|| self.enabled.clone());
        let select_enabled = enabled.clone();
        let readonly = field
            .as_ref()
            .map(|f| f.readonly_state(scope, self.readonly.clone()))
            .unwrap_or_else(|| self.readonly.clone());
        let plain = field.as_ref().is_some_and(|f| f.unstyled);
        let choose = editor.clone();
        let weak = scope.downgrade();
        let region_enabled = scope.bond(move || select_enabled.get() && !readonly.get());
        let region_value = scope.bond(move || Some(draft.with(|d| d.region.clone())));
        let choose_region = move |id| {
            if let Some(id) = id
                && let Some(scope) = weak.upgrade()
            {
                let mut next = choose.control.source.get();
                if next.region == id {
                    return;
                }
                if let (Some(old), Some(new)) = (DATABASE.by_id(&next.region), DATABASE.by_id(&id))
                    && next.text.trim() == format!("+{}", old.country_code())
                {
                    next.text = format!("+{}", new.country_code());
                } else if next.text.trim_start().starts_with('+')
                    && let Some(number) = next.number()
                {
                    next.text = number.format().mode(Mode::National).to_string();
                }
                next.region = id;
                if let Err(err) = choose.update(next, false) {
                    scope.report_error(err);
                }
            }
        };
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
        let mut input = crate::text_field()
            .theme(self.theme.clone())
            .size(self.size)
            .style(self.editor_style.clone())
            .purpose(Purpose::Tel)
            .placeholder(self.placeholder.clone())
            .enabled(self.enabled.clone())
            .loading(self.loading.clone())
            .readonly(self.readonly.clone())
            .required(self.required.clone())
            .value(scope.bond(move || draft.with(|d| d.text.clone())))
            .on_value_change(edit(false))
            .on_commit(edit(true));
        if let Some(reference) = self.editor_ref {
            input = input.node_ref(reference);
        }
        let editor: Box<dyn Widget> = if let Some(field) = field {
            Box::new(input.field(field))
        } else {
            let mut input = Some(input);
            Box::new(
                crate::field(&self.key)
                    .theme(self.theme.clone())
                    .size(self.size)
                    .label(self.label.clone())
                    .required(self.required.clone())
                    .control(move |field| input.take().expect("mount once").field(field)),
            )
        };
        // Keep the dialing region and telephone editor on one line. The full
        // country names and dialing prefixes remain in the native option list.
        let flag = scope.bond(move || {
            draft.with(|draft| {
                draft
                    .region
                    .bytes()
                    .filter(u8::is_ascii_alphabetic)
                    .map(|ascii| {
                        char::from_u32(0x1F1E6 + u32::from(ascii.to_ascii_uppercase() - b'A'))
                            .unwrap_or_default()
                    })
                    .collect::<String>()
            })
        });
        let appearance = self.editor_style.clone();
        let region_style = scope.bond(move || {
            let field = appearance.get();
            Style {
                min_height: field.min_height,
                radius: field.radius,
                background: field.background,
                border_color: field.border_color,
                ..Style::new().width(Length::Percent(100.))
            }
        });
        let country: Box<dyn Widget> = if self.searchable_region {
            let open = scope.cage(self.region_open);
            let search = scope.cage(String::new());
            let mut chooser = crate::combobox(format!("{}-regions", self.key))
                .theme(self.theme.clone())
                .name("Search countries")
                .placeholder("Search countries")
                .inline_list(true)
                .search(search)
                .on_search_change(move |next| search.revise(|value| *value = next))
                .open(true)
                .value(region_value)
                .on_value_change({
                    move |next| {
                        if next.is_some() {
                            choose_region(next);
                            open.revise(|value| *value = false);
                        }
                    }
                });
            for (id, label, code) in searchable_options {
                chooser = chooser
                    .option(crate::select_option(&id, label))
                    .option_content(id, crate::SelectOptionContent::default().trailing(code));
            }
            let mut chooser = Some(chooser);
            // Keep the search/listbox mounted when the anchor closes, retaining the
            // selected country and allowing the menu to reopen with keyboard focus.
            Box::new(
                glory::ui::view()
                    .class("sapid-phone-region sapid-phone-searchable")
                    .fill(
                        crate::popover(format!("{}-region-picker", self.key))
                            .theme(self.theme.clone())
                            .label(self.country_label.clone())
                            .enabled(region_enabled)
                            .open(open)
                            .on_open_change(move |next| {
                                search.revise(|value| value.clear());
                                open.revise(|value| *value = next);
                            })
                            .trigger_content(move || {
                                glory::ui::text()
                                    .class("sapid-phone-country-flag")
                                    .content(flag)
                            })
                            .trigger_style(region_style)
                            .width(crate::PopoverWidth::Px(260.))
                            .show_close(false)
                            .content_policy(crate::ContentPolicy::KeepMounted)
                            .content(move || chooser.take().expect("country search mounts once")),
                    ),
            )
        } else {
            let mut select = crate::native_select(&self.key)
                .theme(self.theme.clone())
                .size(self.size)
                .label(self.country_label.clone())
                .enabled(region_enabled)
                .options(options)
                .value(region_value)
                .on_value_change(choose_region);
            if plain {
                select = select.unstyled();
            }
            Box::new(
                glory::ui::view()
                    .class("sapid-phone-region")
                    .style(Style {
                        shrink: Some(0.),
                        ..Style::new()
                            .position(Position::Relative)
                            .width(Length::Px(54.))
                    })
                    .fill((
                        select.style(region_style),
                        glory::ui::view()
                            .class("sapid-phone-flag")
                            .semantics(Some(Box::new(Semantics {
                                inert: true,
                                ..Default::default()
                            })))
                            .style(Style {
                                align: Some(Align::Center),
                                ..Style::new()
                                    .position(Position::Absolute)
                                    .left(Length::Px(10.))
                                    .top(Length::Px(0.))
                                    .height(Length::Percent(100.))
                                    .display(Display::None)
                                    .pointer_events(PointerEvents::None)
                            })
                            .fill(glory::ui::text().content(flag)),
                    )),
            )
        };
        crate::group()
            .theme(self.theme.clone())
            .class("sapid-phone-input")
            .gap(0.)
            .style(Style {
                align: Some(Align::End),
                ..Style::new()
                    .width(Length::Percent(100.))
                    .flex_wrap(FlexWrap::NoWrap)
            })
            .fill((
                country,
                glory::ui::view()
                    .class("sapid-phone-editor")
                    .style(Style {
                        grow: Some(1.),
                        shrink: Some(1.),
                        ..Style::new().min_width(Length::Px(0.))
                    })
                    .fill(editor),
            ))
            .show_in(scope)?;
        if let Some(name) = form_name {
            let value = scope.bond(move || draft.with(PhoneDraft::rfc3966));
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
