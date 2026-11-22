use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let grow = scope.cage(true);
        let gap = scope.cage(Some("md".to_owned()));
        let palette = theme.clone();
        let preview = each(scope.bond(move || vec![grow.get()]))
            .key(Clone::clone)
            .children(move |row| {
                let grow = row.value.get();
                let theme = palette.clone();
                let colors = theme.clone();
                sapid::grid("growing-grid")
                    .theme(theme.clone())
                    .columns(12)
                    .style(gap.map(move |v| {
                        Style::new()
                            .width(Length::Percent(100.))
                            .gap(colors.spacing(controls::size_value(v.as_deref().unwrap_or("md"))))
                    }))
                    .items((0..5).map(|i| {
                        sapid::grid_item(i.to_string())
                            .span(if i < 3 || !grow { 4 } else { 6 })
                            .fill(controls::grid_demo_cell((i + 1).to_string(), &theme))
                    }))
            });
        let panel = view()
            .class("site-configurator site-action-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::switch("Grow", grow, &theme),
                    controls::scale_slider("grow-grid-gap", "Gap", gap, &theme),
                ],
            ));
        panel.show_in(scope)
    })
}
