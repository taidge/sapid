//! Inline messages use intrinsic live-region roles; severity is independent of
//! whether the application requests an announcement.
use crate::appearance::Common;
use crate::composite::{self, Callback};
use crate::state::Value;
use glory::render::{Role, Semantics, TextKind};
use glory::style::{Align, BorderStyle, Color, Display, Edges, FlexDirection, Length, Style};
use glory::ui::{Children, IntoChildren, Result, Widget};
use glory::{Scope, Vary};

pub const MAX_ALERT_TEXT_BYTES: usize = 64 * 1024;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Announcement {
    Off,
    #[default]
    Polite,
    Assertive,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AlertSeverity {
    #[default]
    Info,
    Success,
    Warning,
    Danger,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AlertVariant {
    #[default]
    Light,
    Filled,
    Outline,
    Transparent,
}
pub struct Alert {
    key: String,
    common: Common,
    content: Vary<String>,
    optional_content: bool,
    notification_surface: bool,
    title: Option<Vary<String>>,
    severity: AlertSeverity,
    variant: AlertVariant,
    color: Option<Color>,
    icon: Option<Children>,
    announcement: Announcement,
    visible: Value<bool>,
    closable: bool,
    close_label: Vary<String>,
    close: Option<Box<dyn FnMut(())>>,
    children: Option<Children>,
    busy: Option<Vary<bool>>,
    icon_visible: Vary<bool>,
}
pub fn alert(key: impl Into<String>) -> Alert {
    Alert {
        key: key.into(),
        common: Common::default(),
        content: "".into(),
        optional_content: false,
        notification_surface: false,
        title: None,
        severity: AlertSeverity::Info,
        variant: AlertVariant::Light,
        color: None,
        icon: None,
        announcement: Announcement::Polite,
        visible: Value {
            source: None,
            default: true,
        },
        closable: false,
        close_label: "Dismiss message".into(),
        close: None,
        children: None,
        busy: None,
        icon_visible: true.into(),
    }
}
impl Alert {
    pub(crate) fn notification_surface(mut self, value: bool) -> Self {
        self.notification_surface = value;
        self
    }
    pub(crate) fn icon_color(&self) -> Color {
        let theme = &self.common.theme;
        let color = self.color.unwrap_or(match self.severity {
            AlertSeverity::Info => theme.accent_text,
            AlertSeverity::Success => theme.success,
            AlertSeverity::Warning => theme.warning,
            AlertSeverity::Danger => theme.danger,
        });
        if self.common.unstyled {
            theme.foreground
        } else if self.variant == AlertVariant::Filled {
            theme.contrasting_foreground(color, theme.on_accent)
        } else {
            color
        }
    }
    pub(crate) fn busy(mut self, value: Vary<bool>) -> Self {
        self.busy = Some(value);
        self
    }
    pub(crate) fn icon_visible(mut self, value: Vary<bool>) -> Self {
        self.icon_visible = value;
        self
    }
    pub fn variant(mut self, value: AlertVariant) -> Self {
        self.variant = value;
        self
    }
    pub fn color(mut self, value: Color) -> Self {
        self.color = Some(value);
        self
    }
    pub fn icon(mut self, value: impl IntoChildren) -> Self {
        self.icon = Some(value.into_children());
        self
    }
    pub fn content(mut self, value: impl Into<Vary<String>>) -> Self {
        self.optional_content = false;
        self.content = value.into();
        self
    }
    /// An editable message may be cleared while its surrounding notice stays mounted.
    pub fn optional_content(mut self, value: impl Into<Vary<String>>) -> Self {
        self.content = value.into();
        self.optional_content = true;
        self
    }
    pub fn title(mut self, value: impl Into<Vary<String>>) -> Self {
        self.title = Some(value.into());
        self
    }
    pub fn severity(mut self, value: AlertSeverity) -> Self {
        self.severity = value;
        self
    }
    pub fn announcement(mut self, value: Announcement) -> Self {
        self.announcement = value;
        self
    }
    pub fn visible(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.visible.source = Some(value.into());
        self
    }
    pub fn closable(mut self, value: bool) -> Self {
        self.closable = value;
        self
    }
    pub fn close_label(mut self, value: impl Into<Vary<String>>) -> Self {
        self.close_label = value.into();
        self
    }
    pub fn on_close(mut self, mut handler: impl FnMut() + 'static) -> Self {
        self.close = Some(Box::new(move |()| handler()));
        self
    }
    pub fn fill(mut self, value: impl IntoChildren) -> Self {
        self.children = Some(value.into_children());
        self
    }
    pub fn theme(mut self, value: crate::Theme) -> Self {
        self.common.theme = value;
        self
    }
    pub fn size(mut self, value: crate::Size) -> Self {
        self.common.size = value;
        self
    }
    pub fn class(mut self, value: impl Into<Vary<String>>) -> Self {
        self.common.class = value.into();
        self
    }
    pub fn style(mut self, value: impl Into<Vary<Style>>) -> Self {
        self.common.style = value.into();
        self
    }
    pub fn name(mut self, value: impl Into<Vary<String>>) -> Self {
        self.common.name = Some(value.into());
        self
    }
    pub fn node_ref(mut self, value: glory::Cage<Option<glory::render::NodeId>>) -> Self {
        self.common.node_ref = Some(value);
        self
    }
    pub fn unstyled(mut self) -> Self {
        self.common.unstyled = true;
        self
    }
}
fn message(value: &str) -> Result {
    if value.trim().is_empty() || value.len() > MAX_ALERT_TEXT_BYTES {
        return Err(glory::render::Error::InvalidProps(
            "alert text must be nonempty and within 64 KiB",
        )
        .into());
    }
    Ok(())
}
impl Widget for Alert {
    fn build(&mut self, scope: &Scope) -> Result {
        let id = composite::id("alert", &self.key, "root", "root")?;
        let title_id = composite::id("alert", &self.key, "title", "root")?;
        let content_id = composite::id("alert", &self.key, "message", "root")?;
        let optional = self.optional_content;
        let content = composite::validated(scope, &self.content, move |value| {
            if optional && value.trim().is_empty() && value.len() <= MAX_ALERT_TEXT_BYTES {
                Ok(())
            } else {
                message(value)
            }
        })?;
        let title = self
            .title
            .as_ref()
            .map(|value| composite::validated(scope, value, |value| message(value)))
            .transpose()?;
        let close_label = if self.closable {
            Some(composite::validated(scope, &self.close_label, |value| {
                message(value)
            })?)
        } else {
            None
        };
        let visible = std::mem::take(&mut self.visible).mount(scope);
        let source = visible.source.clone();
        let mut root = glory::ui::view();
        if let Some(role) = match self.announcement {
            Announcement::Off => None,
            Announcement::Polite => Some(Role::Status),
            Announcement::Assertive => Some(Role::Alert),
        } {
            root = root.role(role);
        }
        let labelled_by = if self.common.name.is_none() && title.is_some() {
            Some(title_id.clone())
        } else {
            None
        };
        let description_id = content_id.clone();
        let busy = self.busy.clone();
        root = root.semantics(scope.bond(move || {
            Some(Box::new(Semantics {
                id: Some(id.clone()),
                labelled_by: labelled_by.clone(),
                described_by: (!optional || !content.get().trim().is_empty())
                    .then(|| description_id.clone()),
                hidden: !source.get(),
                busy: busy.as_ref().map(Vary::get),
                ..Default::default()
            }))
        }));
        let theme = &self.common.theme;
        let (background, color) = match self.severity {
            AlertSeverity::Info => (theme.accent_soft, theme.accent_text),
            AlertSeverity::Success => (theme.success_soft, theme.success),
            AlertSeverity::Warning => (theme.warning_soft, theme.warning),
            AlertSeverity::Danger => (theme.danger_soft, theme.danger),
        };
        let color = self.color.unwrap_or(color);
        let background = if self.color.is_some() {
            Color(color.0, color.1, color.2, 26)
        } else {
            background
        };
        let filled = self.variant == AlertVariant::Filled;
        let foreground = self.icon_color();
        let notification = self.notification_surface;
        let message_color = if notification && title.is_some() {
            theme.muted
        } else if filled {
            foreground
        } else {
            theme.foreground
        };
        let title_color = if notification {
            theme.foreground
        } else {
            foreground
        };
        let icon_size = if notification { 28. } else { 20. };
        let mut base = Style::new()
            .display(Display::Flex)
            .flex_direction(FlexDirection::Row)
            .gap(16.0)
            .min_width(Length::Px(0.0))
            .padding(Edges::all(16.0))
            .radius(theme.radius)
            .border_width(Edges::all(if self.variant == AlertVariant::Outline {
                1.0
            } else {
                0.0
            }))
            .border_style(BorderStyle::Solid)
            .border_color(color);
        base.align = Some(Align::Start);
        base.background = Some(match self.variant {
            AlertVariant::Light => background,
            AlertVariant::Filled => color,
            AlertVariant::Outline | AlertVariant::Transparent => Color(0, 0, 0, 0),
        });
        if notification {
            base.align = Some(Align::Center);
            base.background = Some(theme.surface);
            base.box_shadow = Some(crate::ShadowSize::Large.value(theme));
            base.padding = Some(Edges {
                top: 10.,
                right: 10.,
                bottom: 10.,
                left: 22.,
            });
        }
        base.color = Some(foreground);
        let (classes, style) = self.common.appearance(scope, "sapid-alert", base);
        root = root.class(classes).style(style);
        if let Some(name) = self.common.name.take() {
            root = root.name(name)
        }
        if let Some(reference) = self.common.node_ref {
            root = root.node_ref(reference)
        }
        let mut children = self.children.take();
        let mut icon = self.icon.take();
        let close = Callback::new(self.close.take());
        let weak = scope.downgrade();
        let owner = scope.id();
        let theme = self.common.theme.clone();
        let size = self.common.size;
        let unstyled = self.common.unstyled;
        let icon_visible = self.icon_visible.clone();
        let busy = self.busy.clone();
        let close_enabled: Vary<bool> = if let Some(busy) = self.busy.clone() {
            scope.bond(move || !busy.get()).into()
        } else {
            true.into()
        };
        // All child captures belong to the caller's mounting Owner.
        let wrapper = glory::ui::view().class("sapid-alert-wrapper").style(Style {
            grow: Some(1.),
            align: Some(if notification {
                Align::Center
            } else {
                Align::Start
            }),
            ..Style::new()
                .display(Display::Flex)
                .gap(16.)
                .min_width(Length::Px(0.))
        });
        root.fill(wrapper.fill(glory::ui::widget(move |scope| {
            if let Some(icon) = icon.take() {
                let icon_visible = icon_visible.clone();
                glory::ui::view()
                    .class("sapid-alert-icon")
                    .semantics(scope.bond(move || {
                        Some(Box::new(Semantics {
                            hidden: !icon_visible.get(),
                            ..Default::default()
                        }))
                    }))
                    .style(Style {
                        shrink: Some(0.0),
                        align: Some(Align::Center),
                        justify: Some(if notification {
                            glory::style::Justify::Center
                        } else {
                            glory::style::Justify::Start
                        }),
                        ..Style::new()
                            .display(Display::Flex)
                            .width(Length::Px(icon_size))
                            .height(Length::Px(icon_size))
                    })
                    .fill(icon)
                    .show_in(scope)?;
            }
            let title_id = title_id.clone();
            let content_id = content_id.clone();
            let mut children = children.take();
            let body_theme = theme.clone();
            glory::ui::view()
                .class("sapid-alert-body")
                .style(Style {
                    grow: Some(1.0),
                    ..composite::column(if notification { 2. } else { 8. })
                        .min_width(Length::Px(0.0))
                })
                .fill(glory::ui::widget(move |scope| {
                    if let Some(title) = title {
                        glory::ui::view()
                            .class("sapid-alert-title-row")
                            .fill(
                                glory::ui::text()
                                    .class("sapid-alert-title")
                                    .kind(TextKind::Heading(3))
                                    .content(title)
                                    .style(if unstyled {
                                        Style::new()
                                    } else {
                                        Style {
                                            color: Some(title_color),
                                            ..body_theme
                                                .label(size)
                                                .font_weight(if notification { 500. } else { 700. })
                                                .margin(Edges::all(0.))
                                        }
                                    })
                                    .semantics(Some(Box::new(Semantics {
                                        id: Some(title_id.clone()),
                                        ..Default::default()
                                    }))),
                            )
                            .show_in(scope)?;
                    }
                    let content_id = content_id.clone();
                    glory::ui::text()
                        .class("sapid-alert-content")
                        .content(content)
                        .style(if unstyled {
                            Style::new()
                        } else {
                            Style {
                                color: Some(message_color),
                                ..body_theme.label(size).font_weight(400.)
                            }
                        })
                        .semantics(scope.bond(move || {
                            Some(Box::new(Semantics {
                                id: Some(content_id.clone()),
                                hidden: optional && content.get().trim().is_empty(),
                                ..Default::default()
                            }))
                        }))
                        .show_in(scope)?;
                    if let Some(children) = children.take() {
                        glory::ui::group().fill(children).show_in(scope)?;
                    }
                    Ok(())
                }))
                .show_in(scope)?;
            if let Some(label) = close_label {
                let weak = weak.clone();
                let visible = visible.clone();
                let close = close.clone();
                let busy = busy.clone();
                let button = crate::button()
                    .theme(theme.clone())
                    .size(crate::Size::ExtraSmall)
                    .unstyled()
                    .enabled(close_enabled.clone())
                    .class("sapid-alert-close")
                    .name(label)
                    .label("")
                    .fill(
                        glory::ui::graphic()
                            .drawing(crate::Icon::Close.drawing(foreground))
                            .style(Style::new().width(Length::Px(16.)).height(Length::Px(16.))),
                    )
                    .style(Style {
                        color: (!unstyled).then_some(foreground),
                        shrink: Some(0.0),
                        font_size: Some(20.0),
                        line_height: Some(20.0),
                        border_width: Some(Edges::all(0.0)),
                        background: Some(Color(0, 0, 0, 0)),
                        align: Some(Align::Center),
                        justify: Some(glory::style::Justify::Center),
                        ..Style::new()
                            .display(Display::InlineFlex)
                            .width(Length::Px(if notification { 28.0 } else { 20.0 }))
                            .height(Length::Px(if notification { 28.0 } else { 20.0 }))
                            .min_width(Length::Px(if notification { 28.0 } else { 20.0 }))
                            .min_height(Length::Px(if notification { 28.0 } else { 20.0 }))
                            .padding(Edges::all(0.0))
                            .radius(3.0)
                    })
                    .on_press(move || {
                        let Some(scope) = weak.upgrade() else { return };
                        if busy.as_ref().is_some_and(Vary::get) || !visible.source.get() {
                            return;
                        }
                        match visible.accept(&false) {
                            Ok(()) => close.emit(owner, ()),
                            Err(error) => scope.report_error(error),
                        }
                    });
                button.show_in(scope)?;
            }
            Ok(())
        })))
        .show_in(scope)
    }
}
