use glory::render::{GraphicPoint as Point, PathCommand as Cmd};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChartCurve {
    #[default]
    Linear,
    Bump,
    Natural,
    Monotone,
    Step,
    StepBefore,
    StepAfter,
}

// Each point carries its stacked baseline as well as its plotted x and y.
type Sample = (f64, f64, f64);

impl ChartCurve {
    pub(crate) fn commands_per_edge(self) -> usize {
        match self {
            Self::Step => 3,
            Self::StepBefore | Self::StepAfter => 2,
            _ => 1,
        }
    }

    pub(crate) fn tangents(self, points: &[Sample]) -> Vec<f64> {
        if points.len() < 2 || !matches!(self, Self::Natural | Self::Monotone) {
            return vec![];
        }
        // The category axis is uniformly spaced within a continuous run.
        // Connected gaps can change the distance between neighboring samples.
        let widths: Vec<_> = points.windows(2).map(|w| w[1].0 - w[0].0).collect();
        let slopes: Vec<_> = points
            .windows(2)
            .zip(&widths)
            .map(|(w, h)| (w[1].1 - w[0].1) / h)
            .collect();
        let n = points.len();
        let mut m = vec![0.; n];
        if self == Self::Monotone {
            m[0] = slopes[0];
            m[n - 1] = slopes[n - 2];
            for i in 1..n - 1 {
                let (a, b) = (slopes[i - 1], slopes[i]);
                if a * b > 0. {
                    let p = 2. * widths[i] + widths[i - 1];
                    let q = widths[i] + 2. * widths[i - 1];
                    m[i] = (p + q) / (p / a + q / b);
                }
            }
        } else {
            // Solve for first derivatives of the natural cubic spline.
            let mut upper = vec![0.; n];
            upper[0] = 0.5;
            m[0] = 1.5 * slopes[0];
            for i in 1..n {
                let (a, b, c, rhs) = if i == n - 1 {
                    (1., 2., 0., 3. * slopes[n - 2])
                } else {
                    let (l, r) = (widths[i - 1], widths[i]);
                    (r, 2. * (r + l), l, 3. * (r * slopes[i - 1] + l * slopes[i]))
                };
                let diagonal = b - a * upper[i - 1];
                upper[i] = c / diagonal;
                m[i] = (rhs - a * m[i - 1]) / diagonal;
            }
            for i in (0..n - 1).rev() {
                m[i] -= upper[i] * m[i + 1];
            }
        }
        m
    }

    pub(crate) fn edge(self, span: &[Sample], i: usize, m: &[f64], commands: &mut Vec<Cmd>) {
        let (x, y, _) = span[i - 1];
        let (u, v, _) = span[i];
        let end = Point::new(u, v);
        match self {
            Self::Linear => commands.push(Cmd::Line(end)),
            Self::StepBefore | Self::StepAfter | Self::Step => {
                match self {
                    Self::StepBefore => commands.push(Cmd::Line(Point::new(x, v))),
                    Self::StepAfter => commands.push(Cmd::Line(Point::new(u, y))),
                    _ => {
                        let mid = (x + u) * 0.5;
                        commands
                            .extend([Cmd::Line(Point::new(mid, y)), Cmd::Line(Point::new(mid, v))]);
                    }
                }
                commands.push(Cmd::Line(end));
            }
            Self::Bump => {
                let mid = (x + u) * 0.5;
                commands.push(Cmd::Cubic {
                    first: Point::new(mid, y),
                    second: Point::new(mid, v),
                    end,
                });
            }
            Self::Monotone | Self::Natural => {
                let dx = (u - x) / 3.;
                commands.push(Cmd::Cubic {
                    first: Point::new(x + dx, y + dx * m[i - 1]),
                    second: Point::new(u - dx, v - dx * m[i]),
                    end,
                });
            }
        }
    }
}
