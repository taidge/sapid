//! Deterministic current-snapshot hierarchy and DAG layouts; no retained source,
//! prior order, force simulation, or visited-node history.
use crate::charts::{ChartError, ChartTheme};
use crate::plot_model::*;
use crate::plot_scene::{PlotOptions, PlotScene, rect, ring, stroke};
use glory::render::{GraphicPoint as Point, PathCommand as Cmd};
use std::{
    collections::BTreeMap,
    f64::consts::{FRAC_PI_2, TAU},
};
#[derive(Clone, Copy)]
struct Box2 {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}
struct Hierarchy<'a> {
    nodes: &'a [HierarchyDatum],
    children: Vec<Vec<usize>>,
    weights: Vec<f64>,
    depths: Vec<usize>,
    theme: &'a ChartTheme,
    gap: f64,
    inner: f64,
    root_colors: Vec<usize>,
    show_labels: bool,
}
impl Hierarchy<'_> {
    // Squarify consecutive siblings in the caller's order. Each row consumes
    // the shorter side of the remaining rectangle.
    fn treemap(
        &self,
        scene: &mut PlotScene,
        indices: &[usize],
        mut area: Box2,
    ) -> Result<(), ChartError> {
        let visible: Vec<_> = indices
            .iter()
            .copied()
            .filter(|&i| self.weights[i] > 0.)
            .collect();
        let mut total: f64 = visible.iter().map(|&i| self.weights[i]).sum();
        let mut siblings = visible.as_slice();
        while let Some((&first, _)) = siblings.split_first() {
            let vertical = area.w >= area.h;
            let short = area.w.min(area.h);
            if short <= 0. || total <= 0. {
                break;
            }
            let density = area.w * area.h / total;
            let aspect = |sum: f64, min: f64, max: f64| {
                let span = (sum / short).powi(2);
                let target = (1. + 5_f64.sqrt()) / 2.;
                (max * target / span).max(span / (min * target))
            };
            let mut row_area = self.weights[first] * density;
            let (mut min, mut max) = (row_area, row_area);
            let mut score = aspect(row_area, min, max);
            let mut count = 1;
            for &index in &siblings[1..] {
                let size = self.weights[index] * density;
                let next = aspect(row_area + size, min.min(size), max.max(size));
                if next > score {
                    break;
                }
                row_area += size;
                min = min.min(size);
                max = max.max(size);
                score = next;
                count += 1;
            }
            let row = &siblings[..count];
            let thickness = row_area / short;
            let mut offset = 0.;
            for index in row {
                let weight = self.weights[*index];
                let length = weight * density / thickness;
                let child = if vertical {
                    Box2 {
                        x: area.x,
                        y: area.y + offset,
                        w: thickness,
                        h: length,
                    }
                } else {
                    Box2 {
                        x: area.x + offset,
                        y: area.y,
                        w: length,
                        h: thickness,
                    }
                };
                offset += length;
                let node = &self.nodes[*index];
                let mut color = self.theme.series[*index % 8];
                if !self.children[*index].is_empty() {
                    color.3 = 80;
                }
                scene.layer(
                    &node.key,
                    rect(child.x, child.y, child.w, child.h),
                    Some(color),
                    (self.gap > 0.).then(|| stroke(self.theme.grid, self.gap)),
                )?;
                scene.mark(
                    &node.key,
                    &node.label,
                    format!("Visible leaf weight: {weight}"),
                    if self.children[*index].is_empty() {
                        child.x + child.w / 2.
                    } else {
                        child.x + 2.
                    },
                    if self.children[*index].is_empty() {
                        child.y + child.h / 2.
                    } else {
                        child.y + 2.
                    },
                    self.theme.series[*index % 8],
                )?;
                if self.children[*index].is_empty() && child.w >= 28. && child.h >= 16. {
                    scene.labels.push(crate::charts::PlotLabel {
                        text: node.label.clone(),
                        x: child.x + child.w / 2.,
                        y: child.y + child.h / 2.,
                    });
                }
                let gap = 3f64.min(child.w / 4.).min(child.h / 4.);
                self.treemap(
                    scene,
                    &self.children[*index],
                    Box2 {
                        x: child.x + gap,
                        y: child.y + gap,
                        w: (child.w - 2. * gap).max(0.),
                        h: (child.h - 2. * gap).max(0.),
                    },
                )?;
            }
            if vertical {
                area.x += thickness;
                area.w -= thickness;
            } else {
                area.y += thickness;
                area.h -= thickness;
            }
            total -= row_area / density;
            siblings = &siblings[count..];
        }
        Ok(())
    }
    fn sunburst(
        &self,
        scene: &mut PlotScene,
        indices: &[usize],
        start: f64,
        sweep: f64,
        depth: usize,
        band: f64,
    ) -> Result<(), ChartError> {
        let total = indices.iter().map(|i| self.weights[*i]).sum::<f64>();
        if total <= 0. {
            return Ok(());
        }
        let (mut angle, cx, cy) = (start, scene.frame.width / 2., scene.frame.height / 2.);
        for index in indices {
            let weight = self.weights[*index];
            if weight <= 0. {
                continue;
            }
            let part = sweep * weight / total;
            let node = &self.nodes[*index];
            let (inner, outer) = (
                self.inner + depth as f64 * band,
                self.inner + (depth + 1) as f64 * band,
            );
            let color = self.theme.series[self.root_colors[*index] % 8];
            scene.layer(
                &node.key,
                ring(cx, cy, inner, outer, angle.min(angle + part), part.abs()),
                Some(color),
                (self.gap > 0.).then(|| stroke(self.theme.grid, self.gap)),
            )?;
            let middle = angle + part / 2.;
            if self.show_labels && part.abs() * (inner + outer) / 2. > 20. {
                let radius = (inner + outer) / 2.;
                scene.labels.push(crate::charts::PlotLabel {
                    text: node.label.clone(),
                    x: cx + radius * libm::cos(middle),
                    y: cy + radius * libm::sin(middle),
                });
            }
            scene.mark(
                &node.key,
                &node.label,
                format!("Visible leaf weight: {weight}"),
                cx + (inner + outer) / 2. * libm::cos(middle),
                cy + (inner + outer) / 2. * libm::sin(middle),
                color,
            )?;
            self.sunburst(scene, &self.children[*index], angle, part, depth + 1, band)?;
            angle += part;
        }
        Ok(())
    }
}
pub(crate) fn hierarchy(
    scene: &mut PlotScene,
    nodes: &[HierarchyDatum],
    kind: PlotKind,
    options: &PlotOptions,
    theme: &ChartTheme,
) -> Result<(), ChartError> {
    let indices = nodes
        .iter()
        .enumerate()
        .map(|(i, r)| (r.key.as_str(), i))
        .collect::<BTreeMap<_, _>>();
    let mut children = vec![Vec::new(); nodes.len()];
    let mut roots = Vec::new();
    let mut depths = vec![0; nodes.len()];
    let mut active = vec![true; nodes.len()];
    for (i, node) in nodes.iter().enumerate() {
        let mut cursor = Some(i);
        while let Some(j) = cursor {
            active[i] &= !options.hidden.contains(&nodes[j].key);
            depths[i] += 1;
            cursor = nodes[j]
                .parent
                .as_ref()
                .and_then(|p| indices.get(p.as_str()).copied());
        }
        if let Some(parent) = &node.parent {
            children[*indices.get(parent.as_str()).expect("admitted parent")].push(i);
        } else {
            roots.push(i);
        }
    }
    let mut weights = vec![0.; nodes.len()];
    let mut order = (0..nodes.len()).collect::<Vec<_>>();
    order.sort_by_key(|i| std::cmp::Reverse(depths[*i]));
    for i in order {
        if active[i] {
            weights[i] = nodes[i]
                .weight
                .unwrap_or_else(|| children[i].iter().map(|j| weights[*j]).sum());
        }
    }
    let mut root_colors = vec![0; nodes.len()];
    let mut stack: Vec<_> = roots
        .iter()
        .enumerate()
        .map(|(color, &i)| (i, color))
        .collect();
    while let Some((i, color)) = stack.pop() {
        root_colors[i] = color;
        stack.extend(children[i].iter().map(|&j| (j, color)));
    }
    let radius = (scene.frame.width.min(scene.frame.height) - 32.) / 2.;
    let inner = radius * options.sunburst_inner_ratio.unwrap_or(0.);
    let model = Hierarchy {
        root_colors,
        show_labels: options.hierarchy_labels,
        inner,
        nodes,
        children,
        weights,
        depths,
        theme,
        gap: options.hierarchy_gap.unwrap_or(1.),
    };
    if kind == PlotKind::Treemap {
        model.treemap(
            scene,
            &roots,
            Box2 {
                x: 0.,
                y: 0.,
                w: scene.frame.width,
                h: scene.frame.height,
            },
        )?;
    } else {
        let maximum = model
            .depths
            .iter()
            .enumerate()
            .filter(|(i, _)| active[*i])
            .map(|(_, d)| *d)
            .max()
            .unwrap_or(1);
        let band = (radius - inner) / maximum as f64;
        let (start, sweep) = if options.counterclockwise {
            (0., -TAU)
        } else {
            (-FRAC_PI_2, TAU)
        };
        model.sunburst(scene, &roots, start, sweep, 0, band)?;
    }
    Ok(())
}
pub(crate) fn flows(
    scene: &mut PlotScene,
    data: &FlowData,
    options: &PlotOptions,
    theme: &ChartTheme,
) -> Result<(), ChartError> {
    let indices = data
        .nodes
        .iter()
        .enumerate()
        .map(|(i, r)| (r.key.as_str(), i))
        .collect::<BTreeMap<_, _>>();
    let active = data
        .nodes
        .iter()
        .map(|n| !options.hidden.contains(&n.key))
        .collect::<Vec<_>>();
    let links = data
        .links
        .iter()
        .filter_map(|e| {
            let from = *indices.get(e.source.as_str()).expect("admitted source");
            let to = *indices.get(e.target.as_str()).expect("admitted target");
            (active[from] && active[to]).then_some((e, from, to))
        })
        .collect::<Vec<_>>();
    let mut incoming = vec![0f64; data.nodes.len()];
    let mut outgoing = incoming.clone();
    let mut indegree = vec![0usize; data.nodes.len()];
    let mut children = vec![Vec::new(); data.nodes.len()];
    for (e, from, to) in &links {
        incoming[*to] += e.value;
        outgoing[*from] += e.value;
        indegree[*to] += 1;
        children[*from].push(*to);
    }
    let mut levels = vec![0usize; data.nodes.len()];
    let mut ready = (0..data.nodes.len())
        .filter(|i| active[*i] && indegree[*i] == 0)
        .collect::<Vec<_>>();
    while let Some(i) = ready.pop() {
        for child in &children[i] {
            levels[*child] = levels[*child].max(levels[i] + 1);
            indegree[*child] -= 1;
            if indegree[*child] == 0 {
                ready.push(*child);
            }
        }
    }
    let columns = levels
        .iter()
        .enumerate()
        .filter(|(i, _)| active[*i])
        .map(|(_, l)| *l + 1)
        .max()
        .unwrap_or(1);
    // Align sinks to the final column even when a flow takes a shorter path.
    for (index, level) in levels.iter_mut().enumerate() {
        if children[index].is_empty() && incoming[index] > 0. {
            *level = columns - 1;
        }
    }
    let mut ranks = vec![Vec::new(); columns];
    for (i, level) in levels.iter().enumerate() {
        if active[i] {
            ranks[*level].push(i);
        }
    }
    for rank in &mut ranks {
        rank.sort_by(|&a, &b| {
            incoming[b]
                .max(outgoing[b])
                .total_cmp(&incoming[a].max(outgoing[a]))
        });
    }
    let (node_width, spacing) = options.sankey_node_size.unwrap_or((10., 10.));
    let margin = 5.;
    let height = scene.frame.height - 2. * margin;
    let mut scale = f64::INFINITY;
    for rank in &ranks {
        let n = rank.len().max(1) as f64;
        let gap = (height / n / 4.).min(spacing);
        let zero = (height / n / 2.).min(8.);
        let zeros = rank
            .iter()
            .filter(|i| incoming[**i].max(outgoing[**i]) == 0.)
            .count();
        let total = rank
            .iter()
            .map(|i| incoming[*i].max(outgoing[*i]))
            .sum::<f64>();
        if total > 0. {
            scale = scale.min((height - gap * (n - 1.) - zero * zeros as f64) / total);
        }
    }
    if !scale.is_finite() {
        scale = 0.;
    }
    let mut boxes = vec![
        Box2 {
            x: 0.,
            y: 0.,
            w: 0.,
            h: 0.
        };
        data.nodes.len()
    ];
    for (level, rank) in ranks.iter().enumerate() {
        let n = rank.len().max(1) as f64;
        let gap = (height / n / 4.).min(spacing);
        let zero = (height / n / 2.).min(8.);
        let mut y = margin;
        for i in rank {
            let weight = incoming[*i].max(outgoing[*i]);
            let h = if weight == 0. { zero } else { weight * scale };
            let x = margin
                + (scene.frame.width - 2. * margin - node_width) * level as f64
                    / (columns.saturating_sub(1)).max(1) as f64;
            boxes[*i] = Box2 {
                x,
                y,
                w: node_width,
                h,
            };
            y += h + gap;
        }
    }
    // At either end of a ribbon, stack its port in the vertical order of
    // the nodes on the opposite side. Input record order has no geometry.
    let port_offsets = |outbound: bool| {
        let mut order: Vec<_> = (0..links.len()).collect();
        let ends = |index: usize| {
            let (_, from, to) = links[index];
            if outbound { (from, to) } else { (to, from) }
        };
        order.sort_by(|&a, &b| {
            let (node_a, opposite_a) = ends(a);
            let (node_b, opposite_b) = ends(b);
            node_a
                .cmp(&node_b)
                .then_with(|| boxes[opposite_a].y.total_cmp(&boxes[opposite_b].y))
        });
        let mut used = vec![0.; data.nodes.len()];
        let mut positions = vec![0.; links.len()];
        for index in order {
            let node = ends(index).0;
            positions[index] = boxes[node].y + used[node];
            used[node] += links[index].0.value * scale;
        }
        positions
    };
    let outgoing_y = port_offsets(true);
    let incoming_y = port_offsets(false);
    for (index, (e, from, to)) in links.iter().enumerate() {
        let thickness = e.value * scale;
        if thickness <= 0. {
            continue;
        }
        let a = boxes[*from];
        let b = boxes[*to];
        let x1 = a.x + a.w;
        let x2 = b.x;
        let mid = (x1 + x2) / 2.;
        let y1 = outgoing_y[index];
        let y2 = incoming_y[index];
        let mut color = theme.series[*from % 8];
        color.3 = options
            .link_opacity
            .map_or(112, |opacity| (opacity * 255.).round() as u8);
        scene.layer(
            format!("edge-{}-{}", e.key.len(), e.key),
            vec![
                Cmd::Move(Point::new(x1, y1)),
                Cmd::Cubic {
                    first: Point::new(mid, y1),
                    second: Point::new(mid, y2),
                    end: Point::new(x2, y2),
                },
                Cmd::Line(Point::new(x2, y2 + thickness)),
                Cmd::Cubic {
                    first: Point::new(mid, y2 + thickness),
                    second: Point::new(mid, y1 + thickness),
                    end: Point::new(x1, y1 + thickness),
                },
                Cmd::Close,
            ],
            Some(color),
            None,
        )?;
    }
    for (i, node) in data.nodes.iter().enumerate() {
        if !active[i] {
            continue;
        }
        let b = boxes[i];
        let color = theme.series[i % 8];
        scene.layer(
            format!("node-{}-{}", node.key.len(), node.key),
            rect(b.x, b.y, b.w, b.h),
            Some(color),
            None,
        )?;
        if options.link_opacity.is_some() {
            let align_end = b.x + b.w > scene.frame.width * 0.8;
            let x = if align_end {
                (b.x - 90.).max(0.)
            } else {
                b.x + b.w + 4.
            };
            let y = (b.y + b.h / 2. - 12.).clamp(0., scene.frame.height - 28.);
            scene.labels.push(crate::charts::PlotLabel {
                text: node.label.clone(),
                x,
                y,
            });
            scene.labels.push(crate::charts::PlotLabel {
                text: incoming[i].max(outgoing[i]).to_string(),
                x,
                y: y + 13.,
            });
        }
        scene.mark(
            &node.key,
            &node.label,
            format!("Incoming {}, outgoing {}", incoming[i], outgoing[i]),
            b.x + b.w / 2.,
            b.y + b.h / 2.,
            color,
        )?;
    }
    Ok(())
}
