use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let visible = scope.cage(true);
        let actions = scope.cage(0usize);
        let blur = scope.cage(15.);
        sapid::stack().theme(theme.clone()).gap(16.).fill((
   view().class("site-configurator").fill(controls::playground(Box::new(sapid::overlay().class(blur.map(|v|format!("site-overlay-blur-{}",*v as u8))).theme(theme.clone()).visible(visible).color(Color(0,0,0,255)).opacity(0.35)
    .fill(sapid::card().theme(theme.clone()).fill(sapid::stack().theme(theme.clone()).gap(12.).fill((
     sapid::text().theme(theme.clone()).content("A preview beneath a decorative shade"),
     sapid::text().theme(theme.clone()).content("This layer changes presentation while its content remains operable."),
     sapid::button().theme(theme.clone()).label("Try the preview action").on_press(move||actions.revise(|v|*v+=1)),
     ))))),vec![controls::range("overlay-blur-amount","Blur",blur,0.,30.,1.,&theme)])),
   sapid::checkbox().theme(theme.clone()).label("Show decorative shade").checked(visible).on_checked_change(move|v|visible.revise(|old|*old=v)),
   sapid::text().theme(theme.clone()).content(actions.map(|n|format!("Preview actions: {n}"))),
  )).show_in(scope)
    })
}

#[allow(unused_imports)]
mod controls {
    use glory::prelude::*;
    use glory::style::{Color, FlexWrap};
    use sapid::{Size, Theme};

    pub fn playground(preview: Box<dyn Widget>, controls: Vec<Box<dyn Widget>>) -> Box<dyn Widget> {
        Box::new(
            view()
                .class("site-button-playground")
                .style(
                    Style::new()
                        .display(Display::Flex)
                        .flex_wrap(FlexWrap::Wrap)
                        .gap(24.0),
                )
                .fill((
                    col()
                        .class("site-playground-preview")
                        .style(Style {
                            grow: Some(1.0),
                            align: Some(Align::Center),
                            justify: Some(Justify::Center),
                            ..Style::new()
                                .display(Display::Flex)
                                .flex_direction(FlexDirection::Column)
                                .gap(16.0)
                                .min_width(Length::Px(180.0))
                        })
                        .fill(preview),
                    col()
                        .class("site-playground-controls")
                        .style(
                            Style::new()
                                .display(Display::Flex)
                                .flex_direction(FlexDirection::Column)
                                .gap(16.0)
                                .width(Length::Px(240.0))
                                .max_width(Limit::Percent(100.0)),
                        )
                        .fill(controls),
                )),
        )
    }

    pub fn range(
        key: &str,
        label: &str,
        value: Cage<f64>,
        min: f64,
        max: f64,
        step: f64,
        theme: &Theme,
    ) -> Box<dyn Widget> {
        let label = label.to_owned();
        Box::new(
            sapid::stack().theme(theme.clone()).gap(8.).fill((
                sapid::text()
                    .theme(theme.clone())
                    .class("site-control-label")
                    .content(value.map({
                        let label = label.clone();
                        move |v| {
                            let decimals = if step.fract() == 0. {
                                0
                            } else if (step * 10.).fract() == 0. {
                                1
                            } else {
                                2
                            };
                            format!("{label} · {v:.decimals$}")
                        }
                    })),
                sapid::slider(key.to_owned())
                    .theme(theme.clone())
                    .name(label)
                    .min(min)
                    .max(max)
                    .step(step)
                    .value(value)
                    .on_value_change(move |next| value.revise(|v| *v = next)),
            )),
        )
    }
}
