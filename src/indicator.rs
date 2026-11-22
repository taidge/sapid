//! Passive, anchored status markers around an independent content slot.
use crate::appearance::{Common, configure};
use glory::render::{NodeId, Semantics};
use glory::style::{Align, Display, Edges, Justify, Length, Position, Style, WhiteSpace};
use glory::ui::{Children, IntoChildren, Result, Widget};
use glory::{Cage, Scope, Vary};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IndicatorPosition {
    TopLeft,
    TopCenter,
    #[default]
    TopRight,
    MiddleLeft,
    MiddleCenter,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}
impl IndicatorPosition {
    fn coordinates(self) -> (f64, f64) {
        match self {
            Self::TopLeft => (0.0, 0.0),
            Self::TopCenter => (50.0, 0.0),
            Self::TopRight => (100.0, 0.0),
            Self::MiddleLeft => (0.0, 50.0),
            Self::MiddleCenter => (50.0, 50.0),
            Self::MiddleRight => (100.0, 50.0),
            Self::BottomLeft => (0.0, 100.0),
            Self::BottomCenter => (50.0, 100.0),
            Self::BottomRight => (100.0, 100.0),
        }
    }
}

pub struct Indicator {
    common: Common,
    position: IndicatorPosition,
    diameter: f64,
    radius: Option<f64>,
    with_border: bool,
    processing: Vary<bool>,
    offset: f64,
    color: Option<glory::style::Color>,
    visible: Vary<bool>,
    label: Option<Vary<String>>,
    count: Option<Vary<i64>>,
    max_value: Option<i64>,
    show_zero: bool,
    auto_contrast: bool,
    children: Children,
    marker_ref: Option<Cage<Option<NodeId>>>,
}
pub fn indicator() -> Indicator {
    Indicator {
        common: Common::default(),
        position: IndicatorPosition::default(),
        diameter: 12.0,
        radius: None,
        with_border: false,
        processing: false.into(),
        offset: 0.,
        color: None,
        visible: true.into(),
        label: None,
        count: None,
        max_value: None,
        show_zero: true,
        auto_contrast: true,
        children: ().into_children(),
        marker_ref: None,
    }
}
impl Indicator {
    pub fn theme(mut self, theme: crate::Theme) -> Self {
        self.common.theme = theme;
        self
    }
    pub fn unstyled(mut self) -> Self {
        self.common.unstyled = true;
        self
    }
    pub fn class(mut self, class: impl Into<Vary<String>>) -> Self {
        self.common.class = class.into();
        self
    }
    /// Styles the content wrapper. The anchored marker keeps its own geometry.
    pub fn style(mut self, style: impl Into<Vary<Style>>) -> Self {
        self.common.style = style.into();
        self
    }
    pub fn node_ref(mut self, reference: Cage<Option<NodeId>>) -> Self {
        self.common.node_ref = Some(reference);
        self
    }
    pub fn marker_ref(mut self, reference: Cage<Option<NodeId>>) -> Self {
        self.marker_ref = Some(reference);
        self
    }
    /// An optional passive marker name; this does not make a live region.
    pub fn name(mut self, name: impl Into<Vary<String>>) -> Self {
        self.common.name = Some(name.into());
        self
    }
    pub fn position(mut self, position: IndicatorPosition) -> Self {
        self.position = position;
        self
    }
    /// Fixed square marker box in pixels, in 4..=128. Labels do not resize it.
    pub fn diameter(mut self, diameter: f64) -> Self {
        self.diameter = diameter;
        self
    }
    pub fn radius(mut self, radius: f64) -> Self {
        self.radius = Some(radius);
        self
    }
    pub fn with_border(mut self, with_border: bool) -> Self {
        self.with_border = with_border;
        self
    }
    pub fn processing(mut self, processing: impl Into<Vary<bool>>) -> Self {
        self.processing = processing.into();
        self
    }
    /// Positive offsets inset from the selected edges; center axes stay centered.
    /// Signed finite CSS pixels are bounded to -128..=128.
    pub fn offset(mut self, offset: f64) -> Self {
        self.offset = offset;
        self
    }
    pub fn color(mut self, color: glory::style::Color) -> Self {
        self.color = Some(color);
        self
    }
    pub fn visible(mut self, visible: impl Into<Vary<bool>>) -> Self {
        self.visible = visible.into();
        self
    }
    /// Optional nonempty label, at most 32 UTF-8 bytes. A dot has no text.
    pub fn label(mut self, label: impl Into<Vary<String>>) -> Self {
        self.count = None;
        self.label = Some(label.into());
        self
    }
    /// Numeric marker text. A count can be capped independently of its underlying value.
    pub fn count(mut self, value: impl Into<Vary<i64>>) -> Self {
        self.label = None;
        self.count = Some(value.into());
        self
    }
    pub fn max_value(mut self, value: i64) -> Self {
        self.max_value = Some(value);
        self
    }
    pub fn show_zero(mut self, value: bool) -> Self {
        self.show_zero = value;
        self
    }
    pub fn auto_contrast(mut self, value: bool) -> Self {
        self.auto_contrast = value;
        self
    }
    pub fn fill(mut self, children: impl IntoChildren) -> Self {
        self.children = children.into_children();
        self
    }
}
fn valid_text(value: &str, max: usize) -> Result {
    if value.trim().is_empty() || value.len() > max {
        return Err(glory::render::Error::InvalidProps(
            "indicator text must be nonempty and within its byte budget",
        )
        .into());
    }
    Ok(())
}
fn accepted_text(scope: &Scope, source: Vary<String>, max: usize) -> Result<Vary<String>> {
    let initial = source.try_with(|value| valid_text(value, max).map(|()| value.clone()))??;
    if !source.is_reactive() {
        return Ok(initial.into());
    }
    let accepted = scope.cage(initial);
    let weak = scope.downgrade();
    scope.effect(move || {
        let Some(scope) = weak.upgrade() else { return };
        let result = (|| -> Result {
            let next =
                source.try_with(|value| valid_text(value, max).map(|()| value.clone()))??;
            if accepted.try_with_untracked(|old| *old != next)? {
                accepted.try_revise(|old| *old = next)?;
            }
            Ok(())
        })();
        if let Err(error) = result {
            scope.report_error(error);
        }
    });
    Ok(accepted.into())
}
impl Widget for Indicator {
    fn build(&mut self, scope: &Scope) -> Result {
        if self
            .radius
            .is_some_and(|r| !r.is_finite() || !(0.0..=128.0).contains(&r))
        {
            return Err(
                glory::render::Error::InvalidProps("indicator radius must be in 0..=128").into(),
            );
        }
        if !self.diameter.is_finite() || !(4.0..=128.0).contains(&self.diameter) {
            return Err(glory::render::Error::InvalidProps(
                "indicator diameter must be finite and in 4..=128 pixels",
            )
            .into());
        }
        if !self.offset.is_finite() || !(-128. ..=128.).contains(&self.offset) {
            return Err(glory::render::Error::InvalidProps(
                "indicator offset must be finite and in -128..=128 pixels",
            )
            .into());
        }
        // Validate all sources before any visible nodes are created.
        if let Some(label) = &self.label {
            label.try_with(|value| valid_text(value, 32))??;
        }
        if let Some(name) = &self.common.name {
            name.try_with(|value| valid_text(value, 256))??;
        }
        let numeric = self.count.is_some();
        if let Some(count) = self.count.take() {
            let reactive = count.is_reactive();
            let maximum = self.max_value;
            let text = move || {
                let current = count.get();
                match maximum {
                    Some(limit) if current > limit => format!("{limit}+"),
                    _ => current.to_string(),
                }
            };
            self.label = Some(if reactive {
                scope.bond(text).into()
            } else {
                text().into()
            });
        }
        let label = self
            .label
            .take()
            .map(|source| accepted_text(scope, source, 32))
            .transpose()?;
        let name = self
            .common
            .name
            .take()
            .map(|source| accepted_text(scope, source, 256))
            .transpose()?;
        let (left, top) = self.position.coordinates();
        let inset = |axis: f64| {
            if axis == 0. {
                self.offset
            } else if axis == 100. {
                -self.offset
            } else {
                0.
            }
        };
        let mut marker_style = Style {
            position: Some(Position::Absolute),
            pointer_events: Some(glory::style::PointerEvents::None),
            left: Some(Length::Percent(left)),
            top: Some(Length::Percent(top)),
            width: Some(Length::Px(self.diameter)),
            height: Some(Length::Px(self.diameter)),
            display: Some(Display::Flex),
            align: Some(Align::Center),
            justify: Some(Justify::Center),
            margin: Some(Edges {
                top: -self.diameter / 2.0 + inset(top),
                left: -self.diameter / 2.0 + inset(left),
                right: 0.0,
                bottom: 0.0,
            }),
            ..Style::new()
        };
        if numeric {
            marker_style.width = None;
            marker_style.min_width = Some(Length::Px(self.diameter));
            marker_style.margin.as_mut().unwrap().left = inset(left);
            marker_style.padding = Some(Edges {
                top: 0.,
                right: 4.,
                bottom: 0.,
                left: 4.,
            });
        }
        if !self.common.unstyled {
            marker_style.radius = Some(self.radius.unwrap_or(self.diameter / 2.0));
            if self.with_border {
                marker_style.border_color = Some(self.common.theme.surface);
                marker_style.border_style = Some(glory::style::BorderStyle::Solid);
                marker_style.border_width = Some(Edges::all(2.));
            }
            let color = self.color.unwrap_or(self.common.theme.accent);
            marker_style.background = Some(color);
            marker_style.color = Some(if self.auto_contrast {
                self.common
                    .theme
                    .contrasting_foreground(color, self.common.theme.on_accent)
            } else {
                self.common.theme.on_accent
            });
            marker_style.font_family = Some(vec![glory::style::Family::System]);
            marker_style.font_size = Some((self.diameter * 0.65).min(18.0));
            marker_style.font_weight = Some(600.0);
            marker_style.white_space = Some(WhiteSpace::NoWrap);
        }
        if self.common.unstyled
            && let Some(color) = self.color
        {
            marker_style.background = Some(color);
        }
        let visible = self.visible.clone();
        let check_zero = !self.show_zero;
        let zero_label = label.clone();
        let dynamic_visibility = visible.is_reactive()
            || check_zero && zero_label.as_ref().is_some_and(Vary::is_reactive);
        let semantics = move || {
            Some(Box::new(Semantics {
                hidden: !visible.get()
                    || check_zero && zero_label.as_ref().is_some_and(|value| value.get() == "0"),
                ..Default::default()
            }))
        };
        let processing = self.processing.clone();
        let styled = !self.common.unstyled;
        let class = move || {
            let mut classes = "sapid-indicator-marker".to_owned();
            if numeric {
                classes.push_str(" sapid-indicator-count");
            }
            if styled && processing.get() {
                classes.push_str(" sapid-indicator-processing");
            }
            classes
        };
        let marker_class: Vary<String> = if styled && self.processing.is_reactive() {
            scope.bond(class).into()
        } else {
            class().into()
        };
        let mut marker = glory::ui::view().class(marker_class).style(marker_style);
        marker = if dynamic_visibility {
            marker.semantics(scope.bond(semantics))
        } else {
            marker.semantics(semantics())
        };
        if let Some(name) = name {
            marker = marker.name(name);
        }
        if let Some(reference) = self.marker_ref {
            marker = marker.node_ref(reference);
        }
        if let Some(label) = label {
            marker = marker.fill(
                glory::ui::text()
                    .class("sapid-indicator-label")
                    .content(label),
            );
        }
        // Positioning is structural and stays meaningful in unstyled mode.
        let custom = self.common.style.clone();
        self.common.style = if custom.is_reactive() {
            scope
                .bond(move || {
                    crate::appearance::merge(
                        Style::new()
                            .position(Position::Relative)
                            .display(Display::InlineFlex),
                        custom.get(),
                    )
                })
                .into()
        } else {
            crate::appearance::merge(
                Style::new()
                    .position(Position::Relative)
                    .display(Display::InlineFlex),
                custom.get(),
            )
            .into()
        };
        configure!(
            self.common,
            glory::ui::view(),
            scope,
            "sapid-indicator",
            Style::new()
        )
        .fill(std::mem::replace(&mut self.children, ().into_children()))
        .fill(marker)
        .show_in(scope)
    }
}

impl Default for Indicator {
    fn default() -> Self {
        indicator()
    }
}
