use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group()
        .theme(theme.clone())
        .justify(Justify::Center)
        .fill(
            sapid::scroll_area("horizontal-reading")
                .theme(theme.clone())
                .name("Charizard reading area")
                .axes(glory::render::ScrollAxes::Both)
                .style(
                    Style::new()
                        .width(Length::Px(300.))
                        .max_width(Limit::Percent(100.))
                        .height(Length::Px(200.)),
                )
                .content({
                    let article_theme = theme.clone();
                    move |_| reading_article(article_theme.clone(), true)
                }),
        )
}

fn reading_article(theme: sapid::Theme, wide: bool) -> impl Widget {
    let width = if wide {
        Length::Px(600.)
    } else {
        Length::Percent(100.)
    };
    let heading = sapid::text()
        .theme(theme.clone())
        .weight(700.)
        .style(Style {
            font_size: Some(20.),
            ..Style::new()
        })
        .content("Charizard (Pokémon)");
    let attribution = sapid::text()
        .theme(theme.clone())
        .style(Style {
            color: Some(theme.muted),
            ..Style::new()
        })
        .content("Charizard description from Bulbapedia");
    let paragraphs = [
        "Charizard is an orange, two-legged Pokémon with a long tail tipped by a flame. Its wings have blue-green undersides, and its light-colored belly stretches from its chest to the base of its tail. Two horns rise from the back of its head. It flies in search of strong opponents, breathing fire that can melt rock and carrying itself high above mountain ridges. Each foot has three claws, and two pointed teeth show above its lower jaw. Its narrow arms contrast with a broad chest and powerful back legs. When it rests on the ground, its folded wings rise above its shoulders. The fire at the tip of its tail burns on land and in flight.",
            "Mega Charizard X has dark skin, a blue underside, and blue flames that trail from its mouth. Its broad shoulders carry a pair of spikes, and its wings and horns change shape. Its claws and brow grow larger as its flames intensify. Its horns curve upward, and small spines appear near the neck. The fire at the sides of its mouth glows blue. The lower edge of each wing follows a rounded outline, while the pointed tips reach beyond its shoulders. From the ground its dark silhouette looks quite different from its familiar orange form."
    ].into_iter().map(|paragraph| {
        sapid::text().theme(theme.clone()).style(Style {
            font_size: Some(14.),
            margin: Some(glory::style::Edges { top:16., right:0., bottom:0., left:0. }),
            ..Style::new()
        }).content(paragraph)
    }).collect::<Vec<_>>();
    sapid::stack()
        .theme(theme.clone())
        .gap(0.)
        .style(Style::new().width(width))
        .fill((
            heading,
            attribution,
            sapid::stack().theme(theme).gap(0.).fill(paragraphs),
        ))
}
