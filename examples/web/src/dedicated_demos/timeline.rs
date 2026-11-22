use glory::prelude::*;
use glory::render::{
    GraphicPoint as Point, GraphicProps, GraphicStroke, PathCommand as Path, StrokeCap,
};
use glory::style::Color;

#[derive(Clone, Copy)]
enum Activity {
    Branch,
    Commit,
    PullRequest,
    Review,
}

pub fn demo(theme: sapid::Theme) -> impl Widget {
    const EVENTS: [(&str, &str, &str, &str, Activity); 4] = [
        (
            "branch",
            "Branch created",
            "Mira started a branch for the offline map.",
            "2 hours ago",
            Activity::Branch,
        ),
        (
            "commit",
            "Changes committed",
            "The new map tiles and route legend are ready.",
            "52 minutes ago",
            Activity::Commit,
        ),
        (
            "request",
            "Pull request opened",
            "The route is ready for someone else to try.",
            "34 minutes ago",
            Activity::PullRequest,
        ),
        (
            "review",
            "Review received",
            "Alex checked the trail labels on a smaller screen.",
            "12 minutes ago",
            Activity::Review,
        ),
    ];
    sapid::timeline("release-history")
        .theme(theme.clone())
        .class("site-timeline-usage")
        .name("Map release history")
        .active("commit")
        .bullet_size(24.)
        .line_width(2.)
        .items(EVENTS.map(|(key, title, description, time, activity)| {
            let ink = match activity {
                Activity::Branch => theme.contrasting_foreground(theme.accent, theme.on_accent),
                Activity::Commit | Activity::PullRequest | Activity::Review => theme.foreground,
            };
            let item = sapid::timeline_item(key)
                .title(title)
                .description(description)
                .timestamp(time)
                .marker(
                    glory::ui::graphic()
                        .class("site-timeline-activity-icon")
                        .drawing(activity.drawing(ink))
                        .style(Style::new().width(Length::Px(13.)).height(Length::Px(13.))),
                );
            if matches!(activity, Activity::PullRequest) {
                item.line_style(glory::style::BorderStyle::Dashed)
            } else {
                item
            }
        }))
}

impl Activity {
    fn drawing(self, color: Color) -> GraphicProps {
        let p = Point::new;
        let mut commands = Vec::new();
        match self {
            Self::Branch => {
                commands.extend([
                    Path::Move(p(10., 11.)),
                    Path::Line(p(10., 21.)),
                    Path::Move(p(10., 18.)),
                    curve((10., 16.9), (10.9, 16.), (12., 16.)),
                    Path::Line(p(23., 16.)),
                    curve((24.1, 16.), (25., 15.1), (25., 14.)),
                    Path::Line(p(25., 11.)),
                ]);
                circle(&mut commands, 10., 8., 3.);
                circle(&mut commands, 10., 24., 3.);
                circle(&mut commands, 25., 8., 3.);
            }
            Self::Commit => {
                commands.extend([
                    Path::Move(p(1., 16.)),
                    Path::Line(p(10., 16.)),
                    Path::Move(p(22., 16.)),
                    Path::Line(p(31., 16.)),
                ]);
                circle(&mut commands, 16., 16., 6.);
            }
            Self::PullRequest => {
                commands.extend([
                    Path::Move(p(9., 11.)),
                    Path::Line(p(9., 21.)),
                    Path::Move(p(25., 21.)),
                    Path::Line(p(25., 13.83)),
                    curve((25., 13.3), (24.79, 12.79), (24.41, 12.41)),
                    Path::Line(p(18., 6.)),
                    Path::Move(p(24., 6.)),
                    Path::Line(p(18., 6.)),
                    Path::Line(p(18., 12.)),
                ]);
                circle(&mut commands, 9., 8., 3.);
                circle(&mut commands, 9., 24., 3.);
                circle(&mut commands, 25., 24., 3.);
            }
            Self::Review => {
                commands.extend([
                    Path::Move(p(5.6, 22.)),
                    curve((4.55, 20.18), (4., 18.11), (4., 16.)),
                    curve((4., 9.37), (9.37, 4.), (16., 4.)),
                    curve((22.63, 4.), (28., 9.37), (28., 16.)),
                    curve((28., 22.63), (22.63, 28.), (16., 28.)),
                    curve((13.89, 28.), (11.82, 27.45), (10., 26.4)),
                    Path::Line(p(5.3, 27.97)),
                    curve((4.52, 28.23), (3.77, 27.48), (4.03, 26.7)),
                    Path::Close,
                ]);
                for x in [10.5, 16., 21.5] {
                    circle(&mut commands, x, 16., 0.5);
                }
            }
        }
        GraphicProps {
            width: 32.,
            height: 32.,
            commands,
            stroke: Some(GraphicStroke {
                color,
                width: 2.,
                cap: StrokeCap::Round,
            }),
            ..GraphicProps::default()
        }
    }
}

fn curve(first: (f64, f64), second: (f64, f64), end: (f64, f64)) -> Path {
    Path::Cubic {
        first: Point::new(first.0, first.1),
        second: Point::new(second.0, second.1),
        end: Point::new(end.0, end.1),
    }
}

fn circle(commands: &mut Vec<Path>, x: f64, y: f64, r: f64) {
    let k = r * 0.552_284_75;
    commands.extend([
        Path::Move(Point::new(x + r, y)),
        curve((x + r, y + k), (x + k, y + r), (x, y + r)),
        curve((x - k, y + r), (x - r, y + k), (x - r, y)),
        curve((x - r, y - k), (x - k, y - r), (x, y - r)),
        curve((x + k, y - r), (x + r, y - k), (x + r, y)),
        Path::Close,
    ]);
}
