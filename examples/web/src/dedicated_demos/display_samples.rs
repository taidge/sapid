fn portrait(theme: sapid::Theme, alternate: bool, round: bool) -> impl Widget {
    let media = option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets");
    let file = if alternate {
        "team-member-photo.png"
    } else {
        "menu-user-avatar.png"
    };
    sapid::avatar("Portrait")
        .theme(theme)
        .source(format!("{media}/{file}"))
        .shape(if round {
            sapid::AvatarShape::Circle
        } else {
            sapid::AvatarShape::Square
        })
        .style(
            Style::new()
                .width(Length::Px(56.))
                .height(Length::Px(56.))
                .radius(if round { 28. } else { 4. }),
        )
}
const FRUIT: [(&str, &str, &str, &str); 3] = [
    (
        "apples",
        "🍎",
        "Apples",
        "Apples are crisp fruit that range from tart to sweet. Eat them fresh, bake them into pies, or slice them into salads. Several thousand varieties grow around the world.",
    ),
    (
        "bananas",
        "🍌",
        "Bananas",
        "Bananas have sweet flesh beneath a bright yellow peel. They are a convenient snack, blend well in smoothies, and soften as they ripen.",
    ),
    (
        "broccoli",
        "🥦",
        "Broccoli",
        "Broccoli has green florets and firm, edible stems. Steam it, roast it, or add it to a stir-fry to preserve its texture and color.",
    ),
];
fn fruit_items(mut accordion: sapid::Accordion, disabled: Option<&str>) -> sapid::Accordion {
    for (key, glyph, label, description) in FRUIT {
        accordion = accordion.item(
            sapid::accordion_item(key)
                .label(label)
                .enabled(disabled != Some(key))
                .trigger_content(move || text().content(format!("{glyph}　{label}")))
                .content(move || text().content(description)),
        );
    }
    accordion
}
fn fruit_preview(
    theme: sapid::Theme,
    accordion: sapid::Accordion,
    disabled: Option<&str>,
    width: f64,
) -> impl Widget {
    sapid::group().theme(theme).justify(Justify::Center).fill(
        fruit_items(accordion, disabled).style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(width)),
        ),
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

fn focus_form(theme: sapid::Theme, key: &'static str, target_second: bool) -> impl Widget {
    widget(move |scope| {
        let active = scope.cage(false);
        let preferred = scope.cage(None::<glory::render::SemanticId>);
        let configuration = scope.bond(move || glory::render::FocusScopeConfig {
            initial_focus: preferred.get(),
            ..Default::default()
        });
        let toggle = sapid::button()
            .theme(theme.clone())
            .class("site-focus-form-toggle")
            .label(active.map(|on| {
                if *on {
                    String::from("Deactivate focus trap")
                } else {
                    String::from("Activate focus trap")
                }
            }))
            .on_press(move || active.revise(|v| *v = !*v));
        let fields = ["First input", "Second input", "Third input"]
            .into_iter()
            .enumerate()
            .map(|(index, name)| {
                sapid::field(format!("{key}-{index}"))
                    .theme(theme.clone())
                    .label(name)
                    .control(move |field| {
                        if target_second && index == 1 {
                            preferred.revise(|id| *id = Some(field.id.clone()));
                        }
                        sapid::text_field().field(field).placeholder(name)
                    })
            })
            .collect::<Vec<_>>();
        let form = sapid::focus_scope()
            .configuration(configuration)
            .active(active)
            .loop_tab(true)
            .contain_focus(true)
            .auto_focus(true)
            .restore_focus(true)
            .style(
                Style::new()
                    .display(Display::Flex)
                    .flex_direction(FlexDirection::Column)
                    .gap(12.),
            )
            .fill(fields);
        sapid::group()
            .theme(theme.clone())
            .justify(Justify::Center)
            .fill(
                sapid::stack()
                    .theme(theme.clone())
                    .gap(12.)
                    .style(
                        Style::new()
                            .width(Length::Percent(100.))
                            .max_width(Limit::Px(400.)),
                    )
                    .fill((toggle, form)),
            )
            .show_in(scope)
    })
}

fn collapse_paragraph(theme: sapid::Theme) -> impl Widget {
    sapid::text().theme(theme).content("Bulbasaur is a small Pokémon with a plant bulb on its back. The plant grows with its host, storing energy from sunlight. Two slender vines beneath the bulb help it grasp nearby objects.")
}
