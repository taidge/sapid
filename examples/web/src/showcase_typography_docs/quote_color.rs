use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let wrap = scope.cage("wrap".to_owned());
        let quote=sapid::blockquote().theme(theme.clone()).icon(sapid::Icon::Info).balanced(wrap.map(|s|s=="balance"))
            .style(Style::new().max_width(Limit::Px(440.)).radius(8.).padding(glory::style::Edges::all(28.)))
            .fill(sapid::text().theme(theme.clone()).content("A sketch gives the team something to question. A prototype lets someone try the idea, find its rough edges, and show us what to change before we build it."))
            .citation(sapid::text().theme(theme.clone()).variant(sapid::TextVariant::Small).content("— Design system handbook, chapter two"));
        let control = sapid::stack().theme(theme.clone()).gap(6.).fill((
            sapid::text().theme(theme.clone()).content("Text wrap"),
            sapid::segmented_control("quote-text-wrap")
                .theme(theme.clone())
                .name("Text wrap")
                .value(wrap)
                .segments([
                    sapid::segment("wrap").label("Wrap"),
                    sapid::segment("balance").label("Balance"),
                ])
                .on_value_change(move |next| wrap.revise(|v| *v = next)),
        ));
        view()
            .class("site-configurator site-display-configurator")
            .fill(controls::playground(
                Box::new(quote),
                vec![Box::new(control)],
            ))
            .show_in(scope)
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
}
