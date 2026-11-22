use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let count = scope.cage(3usize);
        let content = theme.clone();
        let list = sapid::scroll_area("autosize-preview")
            .theme(theme.clone())
            .name("Autosizing list")
            .style(
                Style::new()
                    .height(Length::Auto)
                    .max_height(Limit::Px(300.))
                    .width(Length::Percent(100.)).max_width(Limit::Px(400.)),
            )
            .content(move |_| {
                let theme = content.clone();
                each(count.map(|n| (1..=*n).collect::<Vec<_>>()))
                    .key(|n| *n)
                    .seed(move |_| {
                        sapid::text()
                            .theme(theme.clone())
                            .content("A short walk through the garden reveals a different scene each day. New buds open while yesterday’s blossoms fade, their petals scattered along the path. The light changes as clouds pass overhead, showing familiar shapes in a different way.")
                            .style(Style::new().display(Display::Block).padding(Edges {top:0.,right:0.,bottom:12.,left:0.}))
                    })
            });
        let actions = sapid::group().theme(theme.clone()).gap(8.).fill((
            sapid::button()
                .theme(theme.clone())
                .label("Remove paragraph")
                .style(Style {
                    background: Some(Color(224, 49, 49, 255)),
                    ..Style::new()
                })
                .enabled(count.map(|v| *v > 0))
                .on_press(move || count.revise(|v| *v = v.saturating_sub(1))),
            sapid::button()
                .theme(theme.clone())
                .label("Add paragraph")
                .enabled(count.map(|v| *v < 100))
                .on_press(move || count.revise(|v| *v += 1)),
        ));
        sapid::stack()
            .align(Align::Center)
            .theme(theme.clone())
            .gap(14.)
            .fill((list, actions))
            .show_in(scope)
    })
}
