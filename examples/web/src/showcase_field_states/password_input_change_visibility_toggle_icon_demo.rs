use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .theme(theme.clone())
        .class("site-field-state-example choice-demos-field")
        .gap(16.)
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .fill(
            sapid::field("visibility-custom-field")
                .theme(theme.clone())
                .label("Change visibility toggle icon")
                .control(move |field| {
                    sapid::password_input()
                        .default_value("secret")
                        .placeholder("Change visibility toggle icon")
                        .visibility_icon(|revealed, color| {
                            use glory::render::{
                                GraphicPoint as P, GraphicProps, GraphicStroke, PathCommand as C,
                                StrokeCap,
                            };
                            let mut commands = vec![
                                C::Move(P::new(2., 12.)),
                                C::Cubic {
                                    first: P::new(7., 2.),
                                    second: P::new(17., 2.),
                                    end: P::new(22., 12.),
                                },
                                C::Cubic {
                                    first: P::new(17., 22.),
                                    second: P::new(7., 22.),
                                    end: P::new(2., 12.),
                                },
                                C::Close,
                                C::Move(P::new(14.5, 12.)),
                                C::Cubic {
                                    first: P::new(14.5, 8.5),
                                    second: P::new(9.5, 8.5),
                                    end: P::new(9.5, 12.),
                                },
                                C::Cubic {
                                    first: P::new(9.5, 15.5),
                                    second: P::new(14.5, 15.5),
                                    end: P::new(14.5, 12.),
                                },
                                C::Close,
                            ];
                            if revealed {
                                commands
                                    .extend([C::Move(P::new(3., 3.)), C::Line(P::new(21., 21.))]);
                            }
                            GraphicProps {
                                commands,
                                stroke: Some(GraphicStroke {
                                    color,
                                    width: 2.,
                                    cap: StrokeCap::Round,
                                }),
                                ..Default::default()
                            }
                        })
                        .field(field)
                }),
        )
}
