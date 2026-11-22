//! Portable geometry for the finite typed plot records.
use crate::charts::{ChartDomain, ChartError, ChartFrame, ChartPath, ChartTheme, LegendItem};
use crate::plot_model::*;
use glory::render::{
    GraphicPoint as Point, GraphicProps, GraphicStroke, PathCommand as Cmd, ProgressValue,
    StrokeCap,
};
use glory::style::Color;
use std::{
    collections::BTreeSet,
    f64::consts::{FRAC_PI_2, TAU},
};
/// Clockwise arc presentation in degrees. Start is bounded to one revolution;
/// sweep is greater than zero and at most 360 degrees.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlotAngles {
    pub start: f64,
    pub sweep: f64,
}
impl Default for PlotAngles {
    fn default() -> Self {
        Self {
            start: -90.,
            sweep: 360.,
        }
    }
}
/// A compact bullet track with a quantitative label at each threshold.
#[derive(Clone, Debug, PartialEq)]
pub struct BulletTrack {
    pub vertical: bool,
    pub thickness: f64,
    pub bar: Color,
    pub target: Color,
    pub label_divisor: f64,
    pub label_prefix: String,
    pub label_suffix: String,
}
impl BulletTrack {
    fn label(&self, value: f64) -> String {
        format!(
            "{}{}{}",
            self.label_prefix,
            value / self.label_divisor,
            self.label_suffix
        )
    }
}

/// Fixed-size matrix tiles. Color encodes value, while omitted cells use the neutral grid color.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MatrixTiles {
    pub size: f64,
    pub gap: f64,
    pub x_labels: bool,
    pub y_labels: bool,
    pub legend: bool,
}

/// A finite grid of at most 20 by 20 cells. Gap is in the declared frame's units.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WaffleGrid {
    pub rows: u8,
    pub columns: u8,
    pub gap: f64,
}
impl Default for WaffleGrid {
    fn default() -> Self {
        Self {
            rows: 10,
            columns: 10,
            gap: 2.,
        }
    }
}
/// Radius limits in frame units. Square-root scaling continues to encode area.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BubbleRadius {
    pub min: f64,
    pub max: f64,
}
impl Default for BubbleRadius {
    fn default() -> Self {
        Self { min: 0., max: 24. }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SegmentLabelValue {
    Value,
    Percent,
    Name,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SegmentLabelPosition {
    Inside,
    Outside,
    Left,
    Right,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SegmentLabels {
    pub value: SegmentLabelValue,
    pub position: SegmentLabelPosition,
    pub leader: bool,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WaffleDirection {
    #[default]
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}
impl WaffleDirection {
    fn cell(self, index: usize, rows: usize, columns: usize) -> (usize, usize) {
        match self {
            Self::LeftToRight => (index % columns, index / columns),
            Self::RightToLeft => (columns - 1 - index % columns, index / columns),
            Self::TopToBottom => (index / rows, index % rows),
            Self::BottomToTop => (index / rows, rows - 1 - index % rows),
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RadarParts {
    pub grid: bool,
    pub angles: bool,
    pub radius: bool,
    pub dots: bool,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CandleAxes {
    pub grid: crate::charts::ChartAxes,
    pub ticks: crate::charts::ChartAxes,
    pub x: bool,
    pub y: bool,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SparklineStyle {
    pub curve: crate::charts::ChartCurve,
    pub stroke_width: f64,
    pub fill_opacity: f64,
    pub gradient: bool,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PlotOptions {
    pub hidden: BTreeSet<String>,
    pub domain: Option<ChartDomain>,
    pub x_domain: Option<ChartDomain>,
    pub angles: Option<PlotAngles>,
    /// Donut inner radius as a fraction of the outer radius, in 0..1.
    pub donut_inner_ratio: Option<f64>,
    /// Center text for a donut or gauge, at most 64 UTF-8 bytes on one line.
    pub center_label: Option<String>,
    pub waffle_grid: Option<WaffleGrid>,
    pub matrix_tiles: Option<MatrixTiles>,
    pub bullet_track: Option<BulletTrack>,
    /// Fraction of the outer radius left clear in the middle of a sunburst.
    pub sunburst_inner_ratio: Option<f64>,
    /// Draw pie and donut values in counterclockwise order from the configured start.
    pub counterclockwise: bool,
    /// Optional gauge ring thickness in frame units.
    pub gauge_thickness: Option<f64>,
    pub gauge_value_label: bool,
    /// Draw rounded endpoints on the gauge track and its filled sections.
    pub gauge_round_caps: bool,
    /// Optional Sankey node width and vertical spacing in frame units.
    pub sankey_node_size: Option<(f64, f64)>,
    /// Display names at the centers of visible sunburst sectors.
    pub hierarchy_labels: bool,
    /// Link opacity as a fraction of full color strength.
    pub link_opacity: Option<f64>,
    pub bubble_radius: Option<BubbleRadius>,
    pub segment_labels: Option<SegmentLabels>,
    /// Angular gap between neighboring wedges, in degrees.
    pub padding_angle: Option<f64>,
    pub segment_stroke: Option<f64>,
    pub waffle_direction: WaffleDirection,
    pub hierarchy_gap: Option<f64>,
    pub radar_stroke: Option<Color>,
    pub radar_parts: Option<RadarParts>,
    pub candle_axes: Option<CandleAxes>,
    pub sparkline: Option<SparklineStyle>,
    pub tooltip: bool,
    /// Include each visible segment in the tooltip instead of only the active segment.
    pub tooltip_all_segments: bool,
    pub bar_labels: bool,
}
pub(crate) fn path_key(key: &str, part: &str) -> String {
    format!("record-{}_{key}-{}_{part}", key.len(), part.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generated_layers_do_not_collide_with_valid_business_keys() {
        let data = PlotData::values(
            [
                "track",
                "axes",
                "radar",
                "a-track",
                "a-band-0",
                "record-1_a-5_shape",
            ]
            .into_iter()
            .map(|k| ValueDatum::new(k, k, 1.)),
        );
        for kind in [
            PlotKind::Pie,
            PlotKind::Donut,
            PlotKind::RadialBar,
            PlotKind::Radar,
            PlotKind::Waffle,
        ] {
            let scene = build(&data, kind);
            let keys = scene.paths.iter().map(|p| &p.key).collect::<BTreeSet<_>>();
            assert_eq!(keys.len(), scene.paths.len(), "{kind:?}");
        }
    }
    fn theme() -> ChartTheme {
        ChartTheme::from_theme(&crate::Theme::green_dark())
    }
    fn values() -> PlotData {
        PlotData::values([
            ValueDatum::new("one", "One", 1.),
            ValueDatum::new("two", "Two", 3.),
            ValueDatum::new("three", "Three", 2.),
        ])
    }
    fn build(data: &PlotData, kind: PlotKind) -> PlotScene {
        PlotScene::build(
            data,
            kind,
            ChartFrame::default(),
            &PlotOptions::default(),
            &theme(),
        )
        .unwrap()
    }
    fn sample(kind: PlotKind) -> PlotData {
        match kind {
            PlotKind::Scatter | PlotKind::Bubble => PlotData::Points(vec![
                PointDatum {
                    key: "a".into(),
                    label: "A".into(),
                    x: 1.,
                    y: 2.,
                    size: 1.,
                },
                PointDatum {
                    key: "b".into(),
                    label: "B".into(),
                    x: 3.,
                    y: 4.,
                    size: 4.,
                },
            ]),
            PlotKind::Candlestick => PlotData::Candles(vec![CandleDatum {
                key: "a".into(),
                label: "A".into(),
                open: 2.,
                high: 4.,
                low: 1.,
                close: 3.,
            }]),
            PlotKind::Heatmap | PlotKind::Matrix => PlotData::Matrix(MatrixData {
                rows: vec![crate::charts::ChartCategory::new("r", "Row")],
                columns: vec![
                    crate::charts::ChartCategory::new("a", "A"),
                    crate::charts::ChartCategory::new("b", "B"),
                ],
                cells: vec![
                    MatrixCell {
                        key: "one".into(),
                        row: "r".into(),
                        column: "a".into(),
                        value: 1.,
                    },
                    MatrixCell {
                        key: "two".into(),
                        row: "r".into(),
                        column: "b".into(),
                        value: 4.,
                    },
                ],
            }),
            PlotKind::Bullet | PlotKind::Gauge => PlotData::Targets(vec![TargetDatum {
                key: "a".into(),
                label: "A".into(),
                min: 0.,
                max: 100.,
                value: 30.,
                target: Some(70.),
                bands: vec![40., 80., 100.],
            }]),
            PlotKind::Treemap | PlotKind::Sunburst => PlotData::Hierarchy(vec![
                HierarchyDatum {
                    key: "root".into(),
                    label: "Root".into(),
                    parent: None,
                    weight: None,
                },
                HierarchyDatum {
                    key: "one".into(),
                    label: "One".into(),
                    parent: Some("root".into()),
                    weight: Some(1.),
                },
                HierarchyDatum {
                    key: "two".into(),
                    label: "Two".into(),
                    parent: Some("root".into()),
                    weight: Some(3.),
                },
            ]),
            PlotKind::Sankey => PlotData::Flows(FlowData {
                nodes: vec![
                    crate::charts::ChartCategory::new("a", "A"),
                    crate::charts::ChartCategory::new("b", "B"),
                    crate::charts::ChartCategory::new("c", "C"),
                ],
                links: vec![
                    FlowLink {
                        key: "ab".into(),
                        source: "a".into(),
                        target: "b".into(),
                        value: 2.,
                    },
                    FlowLink {
                        key: "bc".into(),
                        source: "b".into(),
                        target: "c".into(),
                        value: 2.,
                    },
                ],
            }),
            _ => values(),
        }
    }
    const KINDS: [PlotKind; 18] = [
        PlotKind::Pie,
        PlotKind::Donut,
        PlotKind::RadialBar,
        PlotKind::Scatter,
        PlotKind::Bubble,
        PlotKind::Candlestick,
        PlotKind::Radar,
        PlotKind::Funnel,
        PlotKind::Sparkline,
        PlotKind::Heatmap,
        PlotKind::Matrix,
        PlotKind::Waffle,
        PlotKind::BarsList,
        PlotKind::Bullet,
        PlotKind::Gauge,
        PlotKind::Treemap,
        PlotKind::Sunburst,
        PlotKind::Sankey,
    ];
    #[test]
    fn all_eighteen_kinds_generate_distinct_valid_geometry_and_finite_current_marks() {
        for kind in KINDS {
            let scene = build(&sample(kind), kind);
            assert!(!scene.paths.is_empty(), "{kind:?}");
            assert!(!scene.marks.is_empty(), "{kind:?}");
            for path in &scene.paths {
                path.drawing.validate().unwrap();
                assert!(path.drawing.alternative.is_none());
            }
            for mark in &scene.marks {
                assert!(mark.x.is_finite() && mark.y.is_finite());
                assert!(
                    (0. ..=640.).contains(&mark.x) && (0. ..=320.).contains(&mark.y),
                    "{kind:?} {mark:?}"
                );
            }
            for range in &scene.ranges {
                range.value.validate().unwrap();
            }
        }
        assert_ne!(
            build(&values(), PlotKind::Pie).paths,
            build(&values(), PlotKind::Donut).paths
        );
        assert_ne!(
            build(&sample(PlotKind::Scatter), PlotKind::Scatter).paths,
            build(&sample(PlotKind::Bubble), PlotKind::Bubble).paths
        );
        assert_ne!(
            build(&sample(PlotKind::Matrix), PlotKind::Matrix).paths,
            build(&sample(PlotKind::Heatmap), PlotKind::Heatmap).paths
        );
    }
    #[test]
    fn pie_angles_and_zero_totals_do_not_invent_percentages_or_slices() {
        let data = PlotData::values([
            ValueDatum::new("a", "A", 1.),
            ValueDatum::new("b", "B", 3.),
            ValueDatum::new("zero", "Zero", 0.),
        ]);
        let scene = build(&data, PlotKind::Pie);
        assert_eq!(scene.paths.len(), 2);
        assert!(scene.marks[0].value_text.contains("25%"));
        assert!(scene.marks[1].value_text.contains("75%"));
        for kind in [PlotKind::Pie, PlotKind::Donut, PlotKind::Waffle] {
            let scene = build(
                &PlotData::values([ValueDatum::new("zero", "Zero", 0.)]),
                kind,
            );
            assert!(scene.paths.is_empty() && scene.marks.is_empty());
        }
    }
    #[test]
    fn waffle_uses_exactly_one_hundred_cells_and_deterministic_remainders() {
        let data = PlotData::values((0..3).map(|i| ValueDatum::new(format!("k{i}"), "Equal", 1.)));
        let scene = build(&data, PlotKind::Waffle);
        assert_eq!(
            scene
                .paths
                .iter()
                .flat_map(|p| &p.drawing.commands)
                .filter(|c| matches!(c, Cmd::Close))
                .count(),
            100
        );
        assert!(scene.marks[0].value_text.contains("34 of 100"));
        assert!(scene.marks[1].value_text.contains("33 of 100"));
    }
    #[test]
    fn bubble_area_tracks_size_instead_of_radius_and_zero_size_has_no_fake_circle() {
        let data = sample(PlotKind::Bubble);
        let scene = build(&data, PlotKind::Bubble);
        let radius = |i: usize| {
            let Cmd::Move(p) = scene
                .paths
                .iter()
                .filter(|p| p.key != "axes")
                .nth(i)
                .unwrap()
                .drawing
                .commands[0]
            else {
                panic!()
            };
            p.x - scene.marks[i].x
        };
        assert!((radius(0) - 12.).abs() < 1e-9);
        assert!((radius(1) - 24.).abs() < 1e-9);
        let PlotData::Points(mut points) = data else {
            panic!()
        };
        points[0].size = 0.;
        assert_eq!(
            build(&PlotData::Points(points), PlotKind::Bubble)
                .marks
                .len(),
            1
        );
    }
    #[test]
    fn sparkline_nulls_create_true_disconnected_paths_and_hidden_keys_are_checked() {
        let data = PlotData::Values(vec![
            ValueDatum::new("a", "A", 1.),
            ValueDatum {
                key: "gap".into(),
                label: "Gap".into(),
                value: None,
            },
            ValueDatum::new("b", "B", 2.),
        ]);
        let scene = build(&data, PlotKind::Sparkline);
        assert_eq!(
            scene.paths[0]
                .drawing
                .commands
                .iter()
                .filter(|c| matches!(c, Cmd::Move(_)))
                .count(),
            2
        );
        assert_eq!(scene.marks.len(), 2);
        assert_eq!(
            PlotScene::build(
                &data,
                PlotKind::Sparkline,
                ChartFrame::default(),
                &PlotOptions {
                    hidden: BTreeSet::from(["unknown".into()]),
                    ..Default::default()
                },
                &theme()
            ),
            Err(ChartError::Key)
        );
    }
    #[test]
    fn explicit_point_domains_clip_and_disclose_values_while_tiny_domains_reject_overflow() {
        let data = sample(PlotKind::Scatter);
        let options = PlotOptions {
            x_domain: Some(ChartDomain::new(0., 2.).unwrap()),
            domain: Some(ChartDomain::new(0., 3.).unwrap()),
            ..Default::default()
        };
        let scene = PlotScene::build(
            &data,
            PlotKind::Scatter,
            ChartFrame::default(),
            &options,
            &theme(),
        )
        .unwrap();
        assert_eq!(scene.clipped_values, 1);
        assert_eq!(scene.marks[1].x, 608.);
        assert_eq!(scene.marks[1].y, 28.);
        let options = PlotOptions {
            x_domain: Some(ChartDomain::new(0., f64::from_bits(1)).unwrap()),
            ..Default::default()
        };
        assert!(
            PlotScene::build(
                &data,
                PlotKind::Scatter,
                ChartFrame::default(),
                &options,
                &theme()
            )
            .is_err()
        );
    }
    #[test]
    fn treemap_parent_weights_and_sunburst_bands_follow_only_visible_descendants() {
        let data = sample(PlotKind::Treemap);
        let scene = build(&data, PlotKind::Treemap);
        assert!(scene.marks[0].value_text.contains('4'));
        assert_eq!(scene.marks.len(), 3);
        for kind in [PlotKind::Treemap, PlotKind::Sunburst] {
            let scene = PlotScene::build(
                &data,
                kind,
                ChartFrame::default(),
                &PlotOptions {
                    hidden: BTreeSet::from(["root".into()]),
                    ..Default::default()
                },
                &theme(),
            )
            .unwrap();
            assert!(scene.paths.is_empty() && scene.marks.is_empty());
        }
        let scene = PlotScene::build(
            &data,
            PlotKind::Treemap,
            ChartFrame::default(),
            &PlotOptions {
                hidden: BTreeSet::from(["two".into()]),
                ..Default::default()
            },
            &theme(),
        )
        .unwrap();
        assert!(scene.marks[0].value_text.contains('1'));
        assert_eq!(scene.marks.len(), 2);
    }
    #[test]
    fn sankey_preserves_port_thickness_across_stages_and_omits_zero_flow_ribbons() {
        let data = sample(PlotKind::Sankey);
        let scene = build(&data, PlotKind::Sankey);
        let edges = scene
            .paths
            .iter()
            .filter(|p| p.key.starts_with("edge-"))
            .collect::<Vec<_>>();
        assert_eq!(edges.len(), 2);
        let thickness = |p: &ChartPath| {
            let Cmd::Move(a) = p.drawing.commands[0] else {
                panic!()
            };
            let Cmd::Line(b) = p.drawing.commands[2] else {
                panic!()
            };
            b.y - a.y
        };
        assert!((thickness(edges[0]) - thickness(edges[1])).abs() < 1e-9);
        let PlotData::Flows(mut data) = data else {
            panic!()
        };
        for e in &mut data.links {
            e.value = 0.;
        }
        let scene = build(&PlotData::Flows(data), PlotKind::Sankey);
        assert!(scene.paths.iter().all(|p| p.key.starts_with("node-")));
        assert_eq!(scene.marks.len(), 3);
    }
    #[test]
    fn maximum_matrix_and_dense_zero_graph_remain_within_protocol_and_frame_budgets() {
        let axes = (0..32)
            .map(|i| crate::charts::ChartCategory::new(format!("k{i}"), format!("Axis {i}")))
            .collect::<Vec<_>>();
        let data = PlotData::Matrix(MatrixData {
            rows: axes.clone(),
            columns: axes,
            cells: (0..1024)
                .map(|i| MatrixCell {
                    key: format!("c{i}"),
                    row: format!("k{}", i / 32),
                    column: format!("k{}", i % 32),
                    value: i as f64,
                })
                .collect(),
        });
        for kind in [PlotKind::Matrix, PlotKind::Heatmap] {
            let scene = build(&data, kind);
            assert_eq!(scene.marks.len(), 1024);
            assert!(scene.paths.len() <= 1024);
            for path in scene.paths {
                path.drawing.validate().unwrap();
            }
        }
        let data = PlotData::Flows(FlowData {
            nodes: (0..128)
                .map(|i| crate::charts::ChartCategory::new(format!("n{i}"), "Node"))
                .collect(),
            links: Vec::new(),
        });
        let scene = build(&data, PlotKind::Sankey);
        assert_eq!(scene.marks.len(), 128);
        assert!(scene.marks.iter().all(|m| m.y <= 320.));
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PlotMark {
    pub key: String,
    pub label: String,
    pub value_text: String,
    pub x: f64,
    pub y: f64,
    pub color: Color,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PlotLabel {
    pub text: String,
    pub x: f64,
    pub y: f64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PlotScaleStop {
    pub value: f64,
    pub color: Color,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PlotRange {
    pub key: String,
    pub label: String,
    pub value: ProgressValue,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PlotScene {
    pub frame: ChartFrame,
    pub paths: Vec<ChartPath>,
    pub marks: Vec<PlotMark>,
    pub labels: Vec<PlotLabel>,
    pub scale_legend: Vec<PlotScaleStop>,
    pub ranges: Vec<PlotRange>,
    pub legend: Vec<LegendItem>,
    pub clipped_values: usize,
}
pub(crate) fn rect(x: f64, y: f64, w: f64, h: f64) -> Vec<Cmd> {
    vec![
        Cmd::Move(Point::new(x, y)),
        Cmd::Line(Point::new(x + w, y)),
        Cmd::Line(Point::new(x + w, y + h)),
        Cmd::Line(Point::new(x, y + h)),
        Cmd::Close,
    ]
}
fn arc(out: &mut Vec<Cmd>, cx: f64, cy: f64, r: f64, start: f64, sweep: f64) {
    let segments = (sweep.abs() / FRAC_PI_2).ceil().max(1.) as usize;
    let step = sweep / segments as f64;
    for i in 0..segments {
        let a = start + i as f64 * step;
        let b = a + step;
        let k = 4. / 3. * libm::tan(step / 4.);
        out.push(Cmd::Cubic {
            first: Point::new(
                cx + r * (libm::cos(a) - k * libm::sin(a)),
                cy + r * (libm::sin(a) + k * libm::cos(a)),
            ),
            second: Point::new(
                cx + r * (libm::cos(b) + k * libm::sin(b)),
                cy + r * (libm::sin(b) - k * libm::cos(b)),
            ),
            end: Point::new(cx + r * libm::cos(b), cy + r * libm::sin(b)),
        });
    }
}
pub(crate) fn ring(cx: f64, cy: f64, inner: f64, outer: f64, start: f64, sweep: f64) -> Vec<Cmd> {
    if sweep <= 0. || outer <= 0. {
        return Vec::new();
    }
    let mut out = vec![Cmd::Move(Point::new(
        cx + outer * libm::cos(start),
        cy + outer * libm::sin(start),
    ))];
    arc(&mut out, cx, cy, outer, start, sweep);
    let end = start + sweep;
    out.push(Cmd::Line(Point::new(
        cx + inner * libm::cos(end),
        cy + inner * libm::sin(end),
    )));
    if inner > 0. {
        arc(&mut out, cx, cy, inner, end, -sweep);
    }
    out.push(Cmd::Close);
    out
}
pub(crate) fn stroke(color: Color, width: f64) -> GraphicStroke {
    GraphicStroke {
        color,
        width,
        cap: StrokeCap::Round,
    }
}
fn scale(values: impl Iterator<Item = f64>) -> Result<ChartDomain, ChartError> {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for v in values {
        min = min.min(v);
        max = max.max(v);
    }
    if !min.is_finite() {
        return ChartDomain::new(0., 1.);
    }
    if min == max {
        let pad = (min.abs() * 0.05).max(1.);
        min -= pad;
        max += pad;
    }
    ChartDomain::new(min, max)
}
fn color_mix(a: Color, b: Color, fraction: f64) -> Color {
    let f = fraction.clamp(0., 1.);
    let c = |a: u8, b: u8| ((f64::from(a) * (1. - f) + f64::from(b) * f).round()) as u8;
    Color(c(a.0, b.0), c(a.1, b.1), c(a.2, b.2), 255)
}
impl PlotScene {
    pub(crate) fn layer(
        &mut self,
        key: impl Into<String>,
        commands: Vec<Cmd>,
        fill: Option<Color>,
        stroke: Option<GraphicStroke>,
    ) -> Result<(), ChartError> {
        if self.paths.len() >= 1024 {
            return Err(ChartError::Budget);
        }
        let drawing = GraphicProps {
            width: self.frame.width,
            height: self.frame.height,
            commands,
            fill,
            stroke,
            ..Default::default()
        };
        drawing.validate().map_err(|_| ChartError::Geometry)?;
        self.paths.push(ChartPath {
            key: key.into(),
            series: None,
            drawing,
        });
        Ok(())
    }
    pub(crate) fn mark(
        &mut self,
        key: &str,
        label: &str,
        value_text: String,
        x: f64,
        y: f64,
        color: Color,
    ) -> Result<(), ChartError> {
        if self.marks.len() >= MAX_MATRIX_CELLS
            || ![x, y].into_iter().all(|v| v.is_finite() && v.abs() <= 1e6)
            || value_text.len() > 768
        {
            return Err(ChartError::Geometry);
        }
        self.marks.push(PlotMark {
            key: key.into(),
            label: label.into(),
            value_text,
            x,
            y,
            color,
        });
        Ok(())
    }
    pub fn build(
        data: &PlotData,
        kind: PlotKind,
        frame: ChartFrame,
        options: &PlotOptions,
        theme: &ChartTheme,
    ) -> Result<Self, ChartError> {
        frame.validate()?;
        data.validate(kind)?;
        if options.hidden.len() > MAX_MATRIX_CELLS {
            return Err(ChartError::Budget);
        }
        if options.radar_parts.is_some() && kind != PlotKind::Radar
            || options.candle_axes.is_some() && kind != PlotKind::Candlestick
            || options.sparkline.is_some_and(|s| {
                kind != PlotKind::Sparkline
                    || !(0.0..=32.).contains(&s.stroke_width)
                    || !(0.0..=1.).contains(&s.fill_opacity)
            })
        {
            return Err(ChartError::Presentation);
        }
        if options.padding_angle.is_some_and(|angle| {
            !matches!(kind, PlotKind::Pie | PlotKind::Donut) || !(0.0..=45.0).contains(&angle)
        }) || options.segment_stroke.is_some_and(|width| {
            !matches!(kind, PlotKind::Pie | PlotKind::Donut | PlotKind::Funnel)
                || !(0.0..=32.0).contains(&width)
        }) || options.hierarchy_gap.is_some_and(|width| {
            !matches!(kind, PlotKind::Sunburst | PlotKind::Treemap)
                || !(0.0..=32.0).contains(&width)
        }) || options
            .segment_labels
            .is_some_and(|_| !matches!(kind, PlotKind::Pie | PlotKind::Donut | PlotKind::Funnel))
            || (options.radar_stroke.is_some() && kind != PlotKind::Radar)
            || (options.waffle_direction != WaffleDirection::LeftToRight
                && kind != PlotKind::Waffle)
        {
            return Err(ChartError::Presentation);
        }
        if let Some(angles) = options.angles
            && (!matches!(
                kind,
                PlotKind::Pie | PlotKind::Donut | PlotKind::RadialBar | PlotKind::Gauge
            ) || !angles.start.is_finite()
                || !(-360.0..=360.0).contains(&angles.start)
                || !angles.sweep.is_finite()
                || !(0.0..=360.0).contains(&angles.sweep)
                || angles.sweep == 0.)
        {
            return Err(ChartError::Presentation);
        }
        if options.tooltip_all_segments
            && !matches!(kind, PlotKind::Pie | PlotKind::Donut | PlotKind::Funnel)
        {
            return Err(ChartError::Presentation);
        }
        if let Some(ratio) = options.donut_inner_ratio
            && (kind != PlotKind::Donut || !ratio.is_finite() || !(0.0..1.0).contains(&ratio))
        {
            return Err(ChartError::Presentation);
        }
        if let Some(label) = &options.center_label
            && (!matches!(kind, PlotKind::Donut | PlotKind::Gauge)
                || label.len() > 64
                || label.contains(['\0', '\r', '\n', '\u{2028}', '\u{2029}']))
        {
            return Err(ChartError::CenterLabel);
        }
        if options.sankey_node_size.is_some_and(|(width, gap)| {
            kind != PlotKind::Sankey || !(1. ..=64.).contains(&width) || !(0. ..=64.).contains(&gap)
        }) || (options.gauge_round_caps && kind != PlotKind::Gauge)
            || (options.hierarchy_labels && kind != PlotKind::Sunburst)
        {
            return Err(ChartError::Presentation);
        }
        if let Some(track) = &options.bullet_track
            && (kind != PlotKind::Bullet
                || !(2.0..=240.0).contains(&track.thickness)
                || !track.label_divisor.is_finite()
                || track.label_divisor < 1.
                || track.label_prefix.len() > 16
                || track.label_suffix.len() > 16
                || track.label_prefix.contains(['\0', '\n', '\r'])
                || track.label_suffix.contains(['\0', '\n', '\r']))
        {
            return Err(ChartError::Presentation);
        }
        if let Some(tiles) = options.matrix_tiles
            && (kind != PlotKind::Matrix
                || !(4.0..=64.).contains(&tiles.size)
                || !(0.0..=16.).contains(&tiles.gap))
        {
            return Err(ChartError::Presentation);
        }
        if options
            .sunburst_inner_ratio
            .is_some_and(|ratio| kind != PlotKind::Sunburst || !(0.0..1.0).contains(&ratio))
            || options.gauge_thickness.is_some_and(|width| {
                kind != PlotKind::Gauge
                    || !(1.0..=frame.width.min(frame.height) / 2.).contains(&width)
            })
            || options
                .link_opacity
                .is_some_and(|v| kind != PlotKind::Sankey || !(0.0..=1.0).contains(&v))
        {
            return Err(ChartError::Presentation);
        }
        if let Some(grid) = options.waffle_grid {
            if kind != PlotKind::Waffle
                || !(1..=20).contains(&grid.rows)
                || !(1..=20).contains(&grid.columns)
                || !grid.gap.is_finite()
                || !(0.0..=16.0).contains(&grid.gap)
            {
                return Err(ChartError::Presentation);
            }
            let cell = ((frame.width - 32.) / f64::from(grid.columns))
                .min((frame.height - 32.) / f64::from(grid.rows));
            if grid.gap >= cell {
                return Err(ChartError::Presentation);
            }
        }
        if let Some(radius) = options.bubble_radius
            && (kind != PlotKind::Bubble
                || !radius.min.is_finite()
                || !radius.max.is_finite()
                || radius.min < 0.
                || radius.max <= 0.
                || radius.max > 64.
                || radius.min > radius.max)
        {
            return Err(ChartError::Presentation);
        }
        let keys = data.keys().into_iter().collect::<BTreeSet<_>>();
        if options
            .hidden
            .iter()
            .any(|key| !keys.contains(key.as_str()))
        {
            return Err(ChartError::Key);
        }
        if options.x_domain.is_some() && !matches!(kind, PlotKind::Scatter | PlotKind::Bubble) {
            return Err(ChartError::Domain);
        }
        if options.domain.is_some()
            && !matches!(
                kind,
                PlotKind::Scatter
                    | PlotKind::Bubble
                    | PlotKind::Candlestick
                    | PlotKind::Sparkline
                    | PlotKind::Radar
                    | PlotKind::RadialBar
                    | PlotKind::Heatmap
                    | PlotKind::Matrix
                    | PlotKind::BarsList
            )
        {
            return Err(ChartError::Domain);
        }
        let mut scene = Self {
            frame,
            paths: Vec::new(),
            marks: Vec::new(),
            labels: Vec::new(),
            scale_legend: Vec::new(),
            ranges: Vec::new(),
            legend: Vec::new(),
            clipped_values: 0,
        };
        match data {
            PlotData::Values(values) => scene.values(values, kind, options, theme)?,
            PlotData::Points(values) | PlotData::SampledPoints { points: values, .. } => {
                scene.points(values, kind, options, theme)?
            }
            PlotData::Candles(values) => scene.candles(values, options, theme)?,
            PlotData::Matrix(values) => scene.matrix(values, kind, options, theme)?,
            PlotData::Targets(values) => scene.targets(values, kind, options, theme)?,
            PlotData::Hierarchy(values) => {
                crate::plot_hierarchy::hierarchy(&mut scene, values, kind, options, theme)?
            }
            PlotData::Flows(values) => {
                crate::plot_hierarchy::flows(&mut scene, values, options, theme)?
            }
        }
        let mut paths = BTreeSet::new();
        let mut marks = BTreeSet::new();
        if scene.paths.iter().any(|p| !paths.insert(&p.key))
            || scene.marks.iter().any(|m| !marks.insert(&m.key))
        {
            return Err(ChartError::Key);
        }
        Ok(scene)
    }
    fn values(
        &mut self,
        values: &[ValueDatum],
        kind: PlotKind,
        options: &PlotOptions,
        theme: &ChartTheme,
    ) -> Result<(), ChartError> {
        let current = values
            .iter()
            .enumerate()
            .filter(|(_, r)| !options.hidden.contains(&r.key))
            .collect::<Vec<_>>();
        for (i, r) in values.iter().enumerate() {
            self.legend.push(LegendItem {
                key: r.key.clone(),
                label: r.label.clone(),
                color: theme.series[i % 8],
            });
        }
        let (w, h) = (self.frame.width, self.frame.height);
        let (cx, cy, radius) = (w / 2., h / 2., (w.min(h) - 40.) / 2.);
        match kind {
            PlotKind::Pie | PlotKind::Donut => {
                let total = current.iter().filter_map(|(_, r)| r.value).sum::<f64>();
                let angles = options.angles.unwrap_or_default();
                let mut start = angles.start.to_radians();
                let total_sweep = angles.sweep.to_radians();
                if let Some(label) = &options.center_label {
                    self.labels.push(PlotLabel {
                        text: label.clone(),
                        x: cx,
                        y: cy,
                    });
                }
                if total > 0. {
                    for (i, r) in &current {
                        let Some(value) = r.value else {
                            continue;
                        };
                        let sweep = value / total * total_sweep;
                        if sweep <= 0. {
                            continue;
                        }
                        let radius = if options
                            .segment_labels
                            .is_some_and(|labels| labels.position == SegmentLabelPosition::Outside)
                        {
                            radius * 0.8
                        } else {
                            radius
                        };
                        let inner = if kind == PlotKind::Donut {
                            radius * options.donut_inner_ratio.unwrap_or(0.62)
                        } else {
                            0.
                        };
                        let color = theme.series[i % 8];
                        let gap = options.padding_angle.unwrap_or(0.).to_radians().min(sweep);
                        let width = options.segment_stroke.unwrap_or(1.);
                        let from = if options.counterclockwise {
                            start - sweep
                        } else {
                            start
                        };
                        self.layer(
                            path_key(&r.key, "shape"),
                            ring(cx, cy, inner, radius, from + gap / 2., sweep - gap),
                            Some(color),
                            (width > 0.).then(|| stroke(theme.grid, width)),
                        )?;
                        let angle = from + sweep / 2.;
                        let middle = (inner + radius) / 2.;
                        self.mark(
                            &r.key,
                            &r.label,
                            format!("{value} ({}%)", (value / total * 10000.).round() / 100.),
                            cx + middle * libm::cos(angle),
                            cy + middle * libm::sin(angle),
                            color,
                        )?;
                        if let Some(labels) = options.segment_labels {
                            let outside = labels.position == SegmentLabelPosition::Outside;
                            let distance = if outside { radius + 18. } else { middle };
                            let x = libm::cos(angle);
                            let y = libm::sin(angle);
                            if outside && labels.leader {
                                self.layer(
                                    path_key(&r.key, "leader"),
                                    vec![
                                        Cmd::Move(Point::new(cx + radius * x, cy + radius * y)),
                                        Cmd::Line(Point::new(
                                            cx + (distance - 3.) * x,
                                            cy + (distance - 3.) * y,
                                        )),
                                    ],
                                    None,
                                    Some(stroke(color, 1.)),
                                )?;
                            }
                            let text = match labels.value {
                                SegmentLabelValue::Name => r.label.clone(),
                                SegmentLabelValue::Value => value.to_string(),
                                SegmentLabelValue::Percent => {
                                    format!("{:.0}%", value * 100. / total)
                                }
                            };
                            self.labels.push(PlotLabel {
                                text,
                                x: cx + distance * x,
                                y: cy + distance * y,
                            });
                        }
                        start += if options.counterclockwise {
                            -sweep
                        } else {
                            sweep
                        };
                    }
                }
            }
            PlotKind::RadialBar => {
                let max = current
                    .iter()
                    .filter_map(|(_, r)| r.value)
                    .fold(0f64, f64::max)
                    .max(1.);
                let domain = options.domain.unwrap_or(ChartDomain::new(0., max)?);
                if domain.min() != 0. {
                    return Err(ChartError::Domain);
                }
                let angles = options.angles.unwrap_or_default();
                let start = angles.start.to_radians();
                let sweep = angles.sweep.to_radians();
                let thickness = radius / current.len().max(1) as f64;
                for (row, (i, r)) in current.iter().enumerate() {
                    let Some(value) = r.value else {
                        continue;
                    };
                    let outer = radius - row as f64 * thickness;
                    let inner = (outer - thickness * 0.75).max(0.);
                    let fraction = domain.fraction(value)?.clamp(0., 1.);
                    if value > domain.max() {
                        self.clipped_values += 1;
                    }
                    self.layer(
                        path_key(&r.key, "track"),
                        ring(cx, cy, inner, outer, start, sweep),
                        Some(theme.grid),
                        None,
                    )?;
                    self.layer(
                        path_key(&r.key, "shape"),
                        ring(cx, cy, inner, outer, start, sweep * fraction),
                        Some(theme.series[i % 8]),
                        None,
                    )?;
                    let angle = start + sweep * fraction;
                    self.mark(
                        &r.key,
                        &r.label,
                        value.to_string(),
                        cx + (inner + outer) / 2. * libm::cos(angle),
                        cy + (inner + outer) / 2. * libm::sin(angle),
                        theme.series[i % 8],
                    )?;
                }
            }
            PlotKind::Radar => {
                let max = current
                    .iter()
                    .filter_map(|(_, r)| r.value)
                    .fold(0f64, f64::max)
                    .max(1.);
                let domain = options.domain.unwrap_or(ChartDomain::new(0., max)?);
                if domain.min() != 0. {
                    return Err(ChartError::Domain);
                }
                let parts = options.radar_parts;
                let mut axes = Vec::new();
                let mut polygon = Vec::new();
                let mut open = false;
                let mut gaps = false;
                for (index, (i, r)) in current.iter().enumerate() {
                    let angle = -FRAC_PI_2 + TAU * index as f64 / current.len().max(1) as f64;
                    axes.extend([
                        Cmd::Move(Point::new(cx, cy)),
                        Cmd::Line(Point::new(
                            cx + radius * libm::cos(angle),
                            cy + radius * libm::sin(angle),
                        )),
                    ]);
                    if parts.is_none_or(|parts| parts.angles) {
                        self.labels.push(PlotLabel {
                            text: r.label.clone(),
                            x: cx + (radius + 8.) * libm::cos(angle),
                            y: cy + (radius + 8.) * libm::sin(angle),
                        });
                    }
                    if let Some(value) = r.value {
                        let fraction = domain.fraction(value)?.clamp(0., 1.);
                        if value > domain.max() {
                            self.clipped_values += 1;
                        }
                        let (x, y) = (
                            cx + radius * fraction * libm::cos(angle),
                            cy + radius * fraction * libm::sin(angle),
                        );
                        polygon.push(if open {
                            Cmd::Line(Point::new(x, y))
                        } else {
                            Cmd::Move(Point::new(x, y))
                        });
                        open = true;
                        self.mark(
                            &r.key,
                            &r.label,
                            value.to_string(),
                            x,
                            y,
                            theme.series[i % 8],
                        )?;
                    } else {
                        open = false;
                        gaps = true;
                    }
                }
                if parts.is_none_or(|p| p.grid) && !axes.is_empty() {
                    self.layer("axes", axes, None, Some(stroke(theme.grid, 1.)))?;
                }
                if let Some(parts) = parts {
                    for tick in 1..=4 {
                        let r = radius * f64::from(tick) / 4.;
                        if parts.grid && current.len() >= 3 {
                            let mut ring = Vec::new();
                            for i in 0..current.len() {
                                let a = -FRAC_PI_2 + TAU * i as f64 / current.len() as f64;
                                let point =
                                    Point::new(cx + r * libm::cos(a), cy + r * libm::sin(a));
                                ring.push(if i == 0 {
                                    Cmd::Move(point)
                                } else {
                                    Cmd::Line(point)
                                });
                            }
                            ring.push(Cmd::Close);
                            self.layer(
                                format!("radar-grid-{tick}"),
                                ring,
                                None,
                                Some(stroke(theme.grid, 1.)),
                            )?;
                        }
                        if parts.radius {
                            self.labels.push(PlotLabel {
                                text: format!("{:.0}", domain.max() * f64::from(tick) / 4.),
                                x: cx + r,
                                y: cy,
                            });
                        }
                    }
                }
                if !polygon.is_empty() {
                    let mut color = theme.series[0];
                    color.3 = 80;
                    if !gaps {
                        polygon.push(Cmd::Close);
                    }
                    self.layer(
                        "radar",
                        polygon,
                        (!gaps).then_some(color),
                        Some(stroke(options.radar_stroke.unwrap_or(theme.series[0]), 2.)),
                    )?;
                }
            }
            PlotKind::Funnel => {
                let max = current
                    .iter()
                    .filter_map(|(_, r)| r.value)
                    .fold(0f64, f64::max)
                    .max(1.);
                let height = (h - 32.) / current.len().max(1) as f64;
                for (row, (i, r)) in current.iter().enumerate() {
                    let Some(value) = r.value else {
                        continue;
                    };
                    let plot_width = if options
                        .segment_labels
                        .is_some_and(|labels| labels.position != SegmentLabelPosition::Inside)
                    {
                        w * 0.67
                    } else {
                        w - 48.
                    };
                    let top = plot_width * value / max;
                    let next = current
                        .get(row + 1)
                        .and_then(|(_, r)| r.value)
                        .unwrap_or(value);
                    let bottom = plot_width * next / max;
                    let y = 16. + row as f64 * height;
                    let color = theme.series[i % 8];
                    self.layer(
                        path_key(&r.key, "shape"),
                        vec![
                            Cmd::Move(Point::new(cx - top / 2., y)),
                            Cmd::Line(Point::new(cx + top / 2., y)),
                            Cmd::Line(Point::new(cx + bottom / 2., y + height * 0.85)),
                            Cmd::Line(Point::new(cx - bottom / 2., y + height * 0.85)),
                            Cmd::Close,
                        ],
                        Some(color),
                        options
                            .segment_stroke
                            .filter(|w| *w > 0.)
                            .map(|w| stroke(theme.grid, w)),
                    )?;
                    if let Some(labels) = options.segment_labels {
                        let x = match labels.position {
                            SegmentLabelPosition::Left => cx - (top + bottom) / 4. - 24.,
                            SegmentLabelPosition::Inside => cx,
                            _ => cx + (top + bottom) / 4. + 24.,
                        };
                        self.labels.push(PlotLabel {
                            text: r.label.clone(),
                            x,
                            y: y + height * 0.42,
                        });
                    }
                    self.mark(
                        &r.key,
                        &r.label,
                        value.to_string(),
                        cx,
                        y + height / 2.,
                        color,
                    )?;
                }
            }
            PlotKind::Sparkline => {
                let domain = options
                    .domain
                    .unwrap_or(scale(current.iter().filter_map(|(_, r)| r.value))?);
                let mut drawing = Vec::new();
                let mut open = false;
                let mut runs = Vec::new();
                let mut run = Vec::new();
                for (index, (i, r)) in current.iter().enumerate() {
                    if let Some(value) = r.value {
                        let fraction = domain.fraction(value)?.clamp(0., 1.);
                        if value < domain.min() || value > domain.max() {
                            self.clipped_values += 1;
                        }
                        let x = 16.
                            + (w - 32.) * index as f64
                                / (current.len().saturating_sub(1)).max(1) as f64;
                        let y = h - 16. - (h - 32.) * fraction;
                        run.push((x, y, h - 16.));
                        drawing.push(if open {
                            Cmd::Line(Point::new(x, y))
                        } else {
                            Cmd::Move(Point::new(x, y))
                        });
                        open = true;
                        self.mark(
                            &r.key,
                            &r.label,
                            value.to_string(),
                            x,
                            y,
                            theme.series[i % 8],
                        )?;
                    } else {
                        open = false;
                        if !run.is_empty() {
                            runs.push(std::mem::take(&mut run));
                        }
                    }
                }
                if !run.is_empty() {
                    runs.push(run);
                }
                if let Some(style) = options.sparkline {
                    for (index, run) in runs.iter().enumerate() {
                        let tangents = style.curve.tangents(run);
                        let mut start = 0;
                        let limit = 250 / style.curve.commands_per_edge();
                        while start < run.len() {
                            let end = (start + limit).min(run.len());
                            let mut line = vec![Cmd::Move(Point::new(run[start].0, run[start].1))];
                            for i in start + 1..end {
                                style.curve.edge(run, i, &tangents, &mut line);
                            }
                            if style.fill_opacity > 0. {
                                let mut fill = line.clone();
                                let base = h - 16.;
                                fill.extend([
                                    Cmd::Line(Point::new(run[end - 1].0, base)),
                                    Cmd::Line(Point::new(run[start].0, base)),
                                    Cmd::Close,
                                ]);
                                let mut color = theme.series[0];
                                color.3 = (255. * style.fill_opacity).round() as u8;
                                self.layer(
                                    format!("spark-fill-{index}-{start}"),
                                    fill,
                                    Some(color),
                                    None,
                                )?;
                            }
                            if style.stroke_width > 0. {
                                self.layer(
                                    format!("spark-line-{index}-{start}"),
                                    line,
                                    None,
                                    Some(stroke(theme.series[0], style.stroke_width)),
                                )?;
                            }
                            if end == run.len() {
                                break;
                            }
                            start = end - 1;
                        }
                    }
                } else if !drawing.is_empty() {
                    self.layer(
                        "sparkline",
                        drawing,
                        None,
                        Some(stroke(theme.series[0], 2.)),
                    )?;
                }
            }
            PlotKind::Waffle => {
                let grid = options.waffle_grid.unwrap_or_default();
                let rows = usize::from(grid.rows);
                let columns = usize::from(grid.columns);
                let units = rows * columns;
                let total = current.iter().filter_map(|(_, r)| r.value).sum::<f64>();
                if total <= 0. {
                    return Ok(());
                }
                let mut allocations = current
                    .iter()
                    .map(|(i, r)| {
                        let units = r.value.unwrap_or(0.) / total * units as f64;
                        (*i, units.floor() as usize, units - units.floor())
                    })
                    .collect::<Vec<_>>();
                let assigned: usize = allocations.iter().map(|(_, n, _)| n).sum();
                let mut order = (0..allocations.len()).collect::<Vec<_>>();
                order.sort_by(|a, b| {
                    allocations[*b]
                        .2
                        .total_cmp(&allocations[*a].2)
                        .then(a.cmp(b))
                });
                for i in order.into_iter().take(units.saturating_sub(assigned)) {
                    allocations[i].1 += 1;
                }
                let cell = ((w - 32.) / columns as f64).min((h - 32.) / rows as f64);
                let left = cx - cell * columns as f64 / 2.;
                let top = cy - cell * rows as f64 / 2.;
                let mut next = 0;
                for (i, count, _) in allocations {
                    let r = &values[i];
                    let mut commands = Vec::new();
                    let (first_col, first_row) =
                        options
                            .waffle_direction
                            .cell(next.min(units - 1), rows, columns);
                    for _ in 0..count {
                        let (col, row) = options.waffle_direction.cell(next, rows, columns);
                        if commands.len() + 5 > 256 {
                            self.layer(
                                path_key(&r.key, &format!("cells-{next}")),
                                std::mem::take(&mut commands),
                                Some(theme.series[i % 8]),
                                None,
                            )?;
                        }
                        commands.extend(rect(
                            left + col as f64 * cell,
                            top + row as f64 * cell,
                            cell - grid.gap,
                            cell - grid.gap,
                        ));
                        next += 1;
                    }
                    if !commands.is_empty() {
                        self.layer(
                            path_key(&r.key, "shape"),
                            commands,
                            Some(theme.series[i % 8]),
                            None,
                        )?;
                        self.mark(
                            &r.key,
                            &r.label,
                            format!("{}; {count} of {units} cells", r.value.unwrap_or(0.)),
                            left + first_col as f64 * cell + cell / 2.,
                            top + first_row as f64 * cell + cell / 2.,
                            theme.series[i % 8],
                        )?;
                    }
                }
            }
            PlotKind::BarsList => {
                let natural = scale(current.iter().filter_map(|(_, r)| r.value).chain([0.]))?;
                let domain = options.domain.unwrap_or(natural);
                let x = |v: f64| -> Result<f64, ChartError> {
                    Ok(24. + domain.fraction(v)?.clamp(0., 1.) * (w - 48.))
                };
                let height = (h - 24.) / current.len().max(1) as f64;
                for (row, (i, r)) in current.iter().enumerate() {
                    let Some(value) = r.value else {
                        continue;
                    };
                    let start = x(0.)?;
                    let end = x(value)?;
                    let y = 12. + row as f64 * height;
                    if value < domain.min() || value > domain.max() {
                        self.clipped_values += 1;
                    }
                    self.layer(
                        path_key(&r.key, "shape"),
                        rect(start.min(end), y, (end - start).abs(), height * 0.72),
                        Some(theme.series[i % 8]),
                        None,
                    )?;
                    if options.bar_labels {
                        self.labels.push(PlotLabel {
                            text: r.label.clone(),
                            x: 28.,
                            y: y + 3.,
                        });
                        self.labels.push(PlotLabel {
                            text: format!("{value}"),
                            x: w - 73.,
                            y: y + 3.,
                        });
                    }
                    self.mark(
                        &r.key,
                        &r.label,
                        value.to_string(),
                        end,
                        y + height * 0.36,
                        theme.series[i % 8],
                    )?;
                    self.ranges.push(PlotRange {
                        key: r.key.clone(),
                        label: r.label.clone(),
                        value: ProgressValue {
                            min: domain.min(),
                            max: domain.max(),
                            current: Some(value.clamp(domain.min(), domain.max())),
                            value_text: Some(value.to_string()),
                        },
                        x: 24.,
                        y,
                        width: w - 48.,
                        height: height * 0.72,
                    });
                }
            }
            _ => return Err(ChartError::Schema),
        }
        Ok(())
    }
    fn points(
        &mut self,
        values: &[PointDatum],
        kind: PlotKind,
        options: &PlotOptions,
        theme: &ChartTheme,
    ) -> Result<(), ChartError> {
        let current = values
            .iter()
            .enumerate()
            .filter(|(_, r)| !options.hidden.contains(&r.key))
            .collect::<Vec<_>>();
        let xd = options
            .x_domain
            .unwrap_or(scale(current.iter().map(|(_, r)| r.x))?);
        let yd = options
            .domain
            .unwrap_or(scale(current.iter().map(|(_, r)| r.y))?);
        let maximum = current
            .iter()
            .map(|(_, r)| r.size)
            .fold(0f64, f64::max)
            .max(f64::MIN_POSITIVE);
        let (w, h) = (self.frame.width, self.frame.height);
        let mut grid = Vec::new();
        for index in 0..5 {
            let fraction = f64::from(index) / 4.;
            let x = 32. + fraction * (w - 64.);
            let y = h - 28. - fraction * (h - 56.);
            grid.extend([
                Cmd::Move(Point::new(x, 28.)),
                Cmd::Line(Point::new(x, h - 28.)),
                Cmd::Move(Point::new(32., y)),
                Cmd::Line(Point::new(w - 32., y)),
            ]);
            let compact = |value: f64| {
                if value.abs() >= 10000. || (value != 0. && value.abs() < 0.01) {
                    format!("{value:.0e}")
                } else {
                    let label = format!("{value:.2}");
                    label.trim_end_matches('0').trim_end_matches('.').to_owned()
                }
            };
            self.labels.push(PlotLabel {
                text: compact(xd.min() + (xd.max() - xd.min()) * fraction),
                x: x - 12.,
                y: h - 43.,
            });
            self.labels.push(PlotLabel {
                text: compact(yd.min() + (yd.max() - yd.min()) * fraction),
                x: 2.,
                y: y - 8.,
            });
        }
        self.layer("axes", grid, None, Some(stroke(theme.grid, 1.)))?;
        for (i, r) in current {
            let x = 32. + xd.fraction(r.x)?.clamp(0., 1.) * (w - 64.);
            let y = h - 28. - yd.fraction(r.y)?.clamp(0., 1.) * (h - 56.);
            let radius = if kind == PlotKind::Bubble {
                {
                    let radius = options.bubble_radius.unwrap_or_default();
                    radius.min + (r.size / maximum).sqrt() * (radius.max - radius.min)
                }
            } else {
                4.
            };
            if r.x < xd.min() || r.x > xd.max() || r.y < yd.min() || r.y > yd.max() {
                self.clipped_values += 1;
            }
            if radius > 0. {
                self.layer(
                    path_key(&r.key, "shape"),
                    ring(x, y, 0., radius, 0., TAU),
                    Some(theme.series[i % 8]),
                    None,
                )?;
                self.mark(
                    &r.key,
                    &r.label,
                    format!("x={}, y={}, size={}", r.x, r.y, r.size),
                    x,
                    y,
                    theme.series[i % 8],
                )?;
            }
        }
        self.labels.extend([
            PlotLabel {
                text: format!("x: {} .. {}", xd.min(), xd.max()),
                x: 32.,
                y: h - 20.,
            },
            PlotLabel {
                text: format!("y: {} .. {}", yd.min(), yd.max()),
                x: 32.,
                y: 8.,
            },
        ]);
        Ok(())
    }
    fn candles(
        &mut self,
        values: &[CandleDatum],
        options: &PlotOptions,
        theme: &ChartTheme,
    ) -> Result<(), ChartError> {
        let current = values
            .iter()
            .filter(|r| !options.hidden.contains(&r.key))
            .collect::<Vec<_>>();
        let domain = options
            .domain
            .unwrap_or(scale(current.iter().flat_map(|r| [r.low, r.high]))?);
        let (w, h) = (self.frame.width, self.frame.height);
        let left = if options.candle_axes.is_some_and(|a| a.y) {
            46.
        } else {
            24.
        };
        let right = w - 24.;
        let band = (right - left) / current.len().max(1) as f64;
        let y = |v: f64| -> Result<f64, ChartError> {
            Ok(h - 24. - domain.fraction(v)?.clamp(0., 1.) * (h - 48.))
        };
        if let Some(axes) = options.candle_axes {
            let mut grid = Vec::new();
            for v in domain.ticks(5)? {
                let py = y(v)?;
                if axes.grid.x() {
                    grid.extend([
                        Cmd::Move(Point::new(left, py)),
                        Cmd::Line(Point::new(right, py)),
                    ]);
                }
                if axes.y {
                    self.labels.push(PlotLabel {
                        text: format!("{v:.1}"),
                        x: 0.,
                        y: py - 6.,
                    });
                    if axes.ticks.y() {
                        grid.extend([
                            Cmd::Move(Point::new(left - 4., py)),
                            Cmd::Line(Point::new(left, py)),
                        ]);
                    }
                }
            }
            for (i, record) in current
                .iter()
                .enumerate()
                .step_by(current.len().div_ceil(8).max(1))
            {
                let px = left + band * (i as f64 + 0.5);
                if axes.grid.y() {
                    grid.extend([
                        Cmd::Move(Point::new(px, 24.)),
                        Cmd::Line(Point::new(px, h - 24.)),
                    ]);
                }
                if axes.x {
                    self.labels.push(PlotLabel {
                        text: record.label.clone(),
                        x: px - 10.,
                        y: h - 21.,
                    });
                    if axes.ticks.x() {
                        grid.extend([
                            Cmd::Move(Point::new(px, h - 24.)),
                            Cmd::Line(Point::new(px, h - 20.)),
                        ]);
                    }
                }
            }
            if !grid.is_empty() {
                self.layer("candle-grid", grid, None, Some(stroke(theme.grid, 1.)))?;
            }
        }
        for (index, r) in current.into_iter().enumerate() {
            let x = left + band * (index as f64 + 0.5);
            let color = theme.series[if r.close >= r.open { 2 } else { 1 }];
            let width = band * 0.6;
            let mut commands = vec![
                Cmd::Move(Point::new(x, y(r.high)?)),
                Cmd::Line(Point::new(x, y(r.low)?)),
            ];
            commands.extend(rect(
                x - width / 2.,
                y(r.open)?.min(y(r.close)?),
                width,
                (y(r.close)? - y(r.open)?).abs().max(1.),
            ));
            self.layer(
                path_key(&r.key, "shape"),
                commands,
                Some(color),
                Some(stroke(color, 1.)),
            )?;
            self.mark(
                &r.key,
                &r.label,
                format!(
                    "open {}, high {}, low {}, close {}",
                    r.open, r.high, r.low, r.close
                ),
                x,
                y(r.close)?,
                color,
            )?;
            if r.low < domain.min() || r.high > domain.max() {
                self.clipped_values += 1;
            }
        }
        Ok(())
    }
    fn matrix(
        &mut self,
        data: &MatrixData,
        kind: PlotKind,
        options: &PlotOptions,
        theme: &ChartTheme,
    ) -> Result<(), ChartError> {
        let current = data
            .cells
            .iter()
            .filter(|c| !options.hidden.contains(&c.key))
            .collect::<Vec<_>>();
        let domain = options
            .domain
            .unwrap_or(scale(current.iter().map(|r| r.value))?);
        if let Some(tiles) = options.matrix_tiles {
            return self.tile_matrix(data, options, theme, domain, tiles);
        }
        for i in 0..5 {
            let fraction = f64::from(i) / 4.;
            self.scale_legend.push(PlotScaleStop {
                value: domain.min() + (domain.max() - domain.min()) * fraction,
                color: color_mix(theme.series[0], theme.series[1], fraction),
            });
        }
        let width = (self.frame.width - 144.) / data.columns.len().max(1) as f64;
        let height = (self.frame.height - 72.) / data.rows.len().max(1) as f64;
        for (row, category) in data.rows.iter().enumerate() {
            self.labels.push(PlotLabel {
                text: category.label.clone(),
                x: 2.,
                y: 56. + (row as f64 + 0.5) * height,
            });
        }
        for (column, category) in data.columns.iter().enumerate() {
            self.labels.push(PlotLabel {
                text: category.label.clone(),
                x: 128. + column as f64 * width,
                y: 8.,
            });
        }
        let maximum = current
            .iter()
            .map(|r| r.value.abs())
            .fold(0f64, f64::max)
            .max(f64::MIN_POSITIVE);
        for c in current {
            let row = data
                .rows
                .iter()
                .position(|r| r.key == c.row)
                .expect("admitted row");
            let col = data
                .columns
                .iter()
                .position(|r| r.key == c.column)
                .expect("admitted column");
            let x = 128. + (col as f64 + 0.5) * width;
            let y = 56. + (row as f64 + 0.5) * height;
            let fraction = domain.fraction(c.value)?.clamp(0., 1.);
            let color = color_mix(theme.series[0], theme.series[1], fraction);
            let scale = if kind == PlotKind::Matrix {
                (c.value.abs() / maximum).sqrt()
            } else {
                1.
            };
            if scale > 0. {
                self.layer(
                    path_key(&c.key, "shape"),
                    rect(
                        x - width * scale / 2.,
                        y - height * scale / 2.,
                        (width * scale - 2.).max(0.),
                        (height * scale - 2.).max(0.),
                    ),
                    Some(color),
                    None,
                )?;
            }
            self.mark(
                &c.key,
                &format!("{} · {}", data.rows[row].label, data.columns[col].label),
                c.value.to_string(),
                x,
                y,
                color,
            )?;
            self.labels.push(PlotLabel {
                text: c.value.to_string(),
                x: 134. + col as f64 * width,
                y,
            });
            if c.value < domain.min() || c.value > domain.max() {
                self.clipped_values += 1;
            }
        }
        Ok(())
    }
    fn tile_matrix(
        &mut self,
        data: &MatrixData,
        options: &PlotOptions,
        theme: &ChartTheme,
        domain: ChartDomain,
        tiles: MatrixTiles,
    ) -> Result<(), ChartError> {
        let step = tiles.size + tiles.gap;
        let left = if tiles.y_labels { 48. } else { 0. };
        let top = if tiles.x_labels { 22. } else { 0. };
        let cells = data
            .cells
            .iter()
            .map(|c| ((c.row.as_str(), c.column.as_str()), c))
            .collect::<std::collections::BTreeMap<_, _>>();
        if tiles.legend {
            for i in 0..5 {
                let fraction = f64::from(i) / 4.;
                self.scale_legend.push(PlotScaleStop {
                    value: domain.min() + (domain.max() - domain.min()) * fraction,
                    color: color_mix(theme.series[0], theme.series[1], fraction),
                });
            }
        }
        if tiles.y_labels {
            for (i, row) in data.rows.iter().enumerate() {
                self.labels.push(PlotLabel {
                    text: row.label.clone(),
                    x: 0.,
                    y: top + i as f64 * step + tiles.size / 2.,
                });
            }
        }
        if tiles.x_labels {
            for (j, column) in data.columns.iter().enumerate() {
                self.labels.push(PlotLabel {
                    text: column.label.clone(),
                    x: left + j as f64 * step,
                    y: 0.,
                });
            }
        }
        for (i, row) in data.rows.iter().enumerate() {
            for (j, column) in data.columns.iter().enumerate() {
                let x = left + j as f64 * step;
                let y = top + i as f64 * step;
                let cell = cells
                    .get(&(row.key.as_str(), column.key.as_str()))
                    .filter(|c| !options.hidden.contains(&c.key));
                let color = if let Some(c) = cell {
                    let fraction = domain.fraction(c.value)?;
                    if !(0.0..=1.).contains(&fraction) {
                        self.clipped_values += 1;
                    }
                    let color = color_mix(theme.series[0], theme.series[1], fraction.clamp(0., 1.));
                    self.mark(
                        &c.key,
                        &format!("{} · {}", row.label, column.label),
                        c.value.to_string(),
                        x + tiles.size / 2.,
                        y + tiles.size / 2.,
                        color,
                    )?;
                    color
                } else {
                    theme.grid
                };
                self.layer(
                    format!("cell-{i}-{j}"),
                    rect(x, y, tiles.size, tiles.size),
                    Some(color),
                    None,
                )?;
            }
        }
        Ok(())
    }
    fn bullet_track(
        &mut self,
        data: &TargetDatum,
        track: &BulletTrack,
        top: f64,
        row_height: f64,
        theme: &ChartTheme,
    ) -> Result<(), ChartError> {
        let width = self.frame.width;
        let domain = ChartDomain::new(data.min, data.max)?;
        let thickness = track.thickness.min(if track.vertical {
            width * 0.5
        } else {
            row_height * 0.65
        });
        let length = if track.vertical {
            row_height - 12.
        } else {
            width - 32.
        };
        let cross = if track.vertical {
            width / 2.
        } else {
            top + row_height / 2.
        };
        let axis = |v: f64| -> Result<f64, ChartError> {
            let fraction = domain.fraction(v)?.clamp(0., 1.);
            Ok(if track.vertical {
                top + row_height - 6. - fraction * length
            } else {
                16. + fraction * length
            })
        };
        let box_path = |a: f64, b: f64, th: f64| {
            if track.vertical {
                rect(cross - th / 2., a.min(b), th, (a - b).abs())
            } else {
                rect(a.min(b), cross - th / 2., (a - b).abs(), th)
            }
        };
        let mut previous = axis(data.min)?;
        for (i, threshold) in data.bands.iter().enumerate() {
            let end = axis(*threshold)?;
            self.layer(
                path_key(&data.key, &format!("band-{i}")),
                box_path(previous, end, thickness),
                Some(theme.series[i % 8]),
                None,
            )?;
            let (x, y) = if track.vertical {
                (cross + thickness / 2. + 24., end)
            } else {
                (end, cross - thickness / 2. - 11.)
            };
            self.labels.push(PlotLabel {
                text: track.label(*threshold),
                x,
                y,
            });
            previous = end;
        }
        let value = axis(data.value)?;
        self.layer(
            path_key(&data.key, "shape"),
            box_path(axis(data.min)?, value, thickness * 0.34),
            Some(track.bar),
            None,
        )?;
        if let Some(target) = data.target {
            let point = axis(target)?;
            self.layer(
                path_key(&data.key, "target"),
                box_path(point - 1.5, point + 1.5, thickness * 0.8),
                Some(track.target),
                None,
            )?;
            let (x, y) = if track.vertical {
                (cross + thickness / 2. + 24., point)
            } else {
                (point, cross - thickness / 2. - 11.)
            };
            self.labels.push(PlotLabel {
                text: track.label(target),
                x,
                y,
            });
        }
        let (x, y) = if track.vertical {
            (cross - thickness / 2. - 20., value)
        } else {
            (value, cross + thickness / 2. + 13.)
        };
        self.labels.push(PlotLabel {
            text: track.label(data.value),
            x,
            y,
        });
        let (x, y) = if track.vertical {
            (cross, value)
        } else {
            (value, cross)
        };
        self.mark(
            &data.key,
            &data.label,
            track.label(data.value),
            x,
            y,
            track.bar,
        )?;
        self.ranges.push(PlotRange {
            key: data.key.clone(),
            label: data.label.clone(),
            value: ProgressValue {
                min: data.min,
                max: data.max,
                current: Some(data.value),
                value_text: Some(track.label(data.value)),
            },
            x: 0.,
            y: top,
            width,
            height: row_height,
        });
        Ok(())
    }
    fn targets(
        &mut self,
        values: &[TargetDatum],
        kind: PlotKind,
        options: &PlotOptions,
        theme: &ChartTheme,
    ) -> Result<(), ChartError> {
        let current = values
            .iter()
            .filter(|r| !options.hidden.contains(&r.key))
            .collect::<Vec<_>>();
        let (w, h) = (self.frame.width, self.frame.height);
        let row_height = (h - 32.) / current.len().max(1) as f64;
        for (index, r) in current.iter().enumerate() {
            let domain = ChartDomain::new(r.min, r.max)?;
            let fraction = domain.fraction(r.value)?;
            let top = 16. + index as f64 * row_height;
            if let Some(track) = &options.bullet_track {
                self.bullet_track(r, track, top, row_height, theme)?;
                continue;
            }
            if !options.gauge_value_label {
                self.labels.extend([
                    PlotLabel {
                        text: r.min.to_string(),
                        x: 24.,
                        y: top + row_height * 0.78,
                    },
                    PlotLabel {
                        text: r.max.to_string(),
                        x: w - 80.,
                        y: top + row_height * 0.78,
                    },
                ]);
            }
            if kind == PlotKind::Gauge {
                let (radius, cx, cy) = if options.gauge_value_label {
                    // A partial dial can use the entire width and end above the bottom
                    // of a square. The frame leaves room for its value and needle.
                    let radius = (w / 2.).min(h * 0.65);
                    (radius, w / 2., radius)
                } else {
                    ((w.min(h) - 48.) / 2., w / 2., h / 2.)
                };
                let inner = options
                    .gauge_thickness
                    .map_or(radius * 0.78, |w| (radius - w).max(0.));
                let angles = options.angles.unwrap_or(PlotAngles {
                    start: 180.,
                    sweep: 180.,
                });
                let start = angles.start.to_radians();
                let sweep = angles.sweep.to_radians();
                self.layer(
                    "track",
                    ring(cx, cy, inner, radius, start, sweep),
                    Some(theme.grid),
                    None,
                )?;
                let center_radius = (inner + radius) / 2.;
                let cap_radius = (radius - inner) / 2.;
                let cap = |angle: f64| {
                    ring(
                        cx + center_radius * libm::cos(angle),
                        cy + center_radius * libm::sin(angle),
                        0.,
                        cap_radius,
                        0.,
                        TAU,
                    )
                };
                if options.gauge_round_caps {
                    for (key, angle) in [("track-start", start), ("track-end", start + sweep)] {
                        self.layer(key, cap(angle), Some(theme.grid), None)?;
                    }
                }
                if r.bands.is_empty() {
                    self.layer(
                        path_key(&r.key, "shape"),
                        ring(cx, cy, inner, radius, start, sweep * fraction),
                        Some(theme.series[0]),
                        None,
                    )?;
                    if options.gauge_round_caps && fraction > 0. {
                        for (key, angle) in [
                            ("fill-start", start),
                            ("fill-end", start + sweep * fraction),
                        ] {
                            self.layer(key, cap(angle), Some(theme.series[0]), None)?;
                        }
                    }
                } else {
                    let mut previous = 0.;
                    for (i, &limit) in r.bands.iter().enumerate() {
                        let end = domain.fraction(limit)?;
                        let color = theme.series[i % theme.series.len()];
                        self.layer(
                            format!("section-{i}"),
                            ring(
                                cx,
                                cy,
                                inner,
                                radius,
                                start + previous * sweep,
                                (end - previous) * sweep,
                            ),
                            Some(color),
                            None,
                        )?;
                        if options.gauge_round_caps && (i == 0 || i + 1 == r.bands.len()) {
                            if i == 0 {
                                self.layer("section-start", cap(start), Some(color), None)?;
                            }
                            if i + 1 == r.bands.len() {
                                self.layer(
                                    "section-end",
                                    cap(start + end * sweep),
                                    Some(color),
                                    None,
                                )?;
                            }
                        }
                        previous = end;
                    }
                }
                if let Some(target) = r.target {
                    let angle = start + sweep * domain.fraction(target)?;
                    self.layer(
                        "target",
                        vec![
                            Cmd::Move(Point::new(
                                cx + inner * libm::cos(angle),
                                cy + inner * libm::sin(angle),
                            )),
                            Cmd::Line(Point::new(
                                cx + (radius + 5.) * libm::cos(angle),
                                cy + (radius + 5.) * libm::sin(angle),
                            )),
                        ],
                        None,
                        Some(stroke(theme.series[1], 3.)),
                    )?;
                }
                self.mark(
                    &r.key,
                    &r.label,
                    format!(
                        "{} in {} .. {}; target {}",
                        r.value,
                        r.min,
                        r.max,
                        r.target.map_or_else(|| "none".into(), |v| v.to_string())
                    ),
                    cx,
                    cy,
                    theme.series[0],
                )?;
                self.labels.push(PlotLabel {
                    text: if options.gauge_value_label {
                        options
                            .center_label
                            .clone()
                            .unwrap_or_else(|| r.value.to_string())
                    } else {
                        format!("{} / {}", r.value, r.max)
                    },
                    x: if options.gauge_value_label {
                        cx
                    } else {
                        cx - 32.
                    },
                    y: if options.gauge_value_label {
                        cy
                    } else {
                        cy + 16.
                    },
                });
            } else {
                let mut previous = r.min;
                for (band, end) in r.bands.iter().enumerate() {
                    let start = domain.fraction(previous)?;
                    let end_fraction = domain.fraction(*end)?;
                    let mut color = theme.series[band % 8];
                    color.3 = 80;
                    self.layer(
                        path_key(&r.key, &format!("band-{band}")),
                        rect(
                            24. + start * (w - 48.),
                            top,
                            (end_fraction - start) * (w - 48.),
                            row_height * 0.75,
                        ),
                        Some(color),
                        None,
                    )?;
                    previous = *end;
                }
                self.layer(
                    path_key(&r.key, "shape"),
                    rect(
                        24.,
                        top + row_height * 0.2,
                        fraction * (w - 48.),
                        row_height * 0.35,
                    ),
                    Some(theme.series[0]),
                    None,
                )?;
                if let Some(target) = r.target {
                    let x = 24. + domain.fraction(target)? * (w - 48.);
                    self.layer(
                        path_key(&r.key, "target"),
                        vec![
                            Cmd::Move(Point::new(x, top)),
                            Cmd::Line(Point::new(x, top + row_height * 0.75)),
                        ],
                        None,
                        Some(stroke(theme.series[1], 3.)),
                    )?;
                }
                self.mark(
                    &r.key,
                    &r.label,
                    format!(
                        "{} in {} .. {}; target {}",
                        r.value,
                        r.min,
                        r.max,
                        r.target.map_or_else(|| "none".into(), |v| v.to_string())
                    ),
                    24. + fraction * (w - 48.),
                    top + row_height * 0.375,
                    theme.series[0],
                )?;
            }
            self.ranges.push(PlotRange {
                key: r.key.clone(),
                label: r.label.clone(),
                value: ProgressValue {
                    min: r.min,
                    max: r.max,
                    current: Some(r.value),
                    value_text: Some(format!(
                        "{}; target {}",
                        r.value,
                        r.target.map_or_else(|| "none".into(), |v| v.to_string())
                    )),
                },
                x: 24.,
                y: if kind == PlotKind::Gauge { 16. } else { top },
                width: w - 48.,
                height: if kind == PlotKind::Gauge {
                    h - 32.
                } else {
                    row_height * 0.75
                },
            });
        }
        Ok(())
    }
}
