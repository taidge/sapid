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
