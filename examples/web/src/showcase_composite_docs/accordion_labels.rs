use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let mut accordion = sapid::accordion("custom-labels")
        .theme(theme.clone())
        .variant(sapid::AccordionVariant::Contained)
        .class("site-accordion-characters");
    for (key, name, description, body, photo) in [
        (
            "bender",
            "Bender Bending Rodríguez",
            "A robot with a passion for cooking",
            "Bender is a bending robot who works with the Planet Express crew.",
            "menu-user-avatar.png",
        ),
        (
            "carol",
            "Carol Miller",
            "One of the richest people on Earth",
            "Carol runs a robotics company and is known to the public as Mom.",
            "team-member-photo.png",
        ),
        (
            "homer",
            "Homer Simpson",
            "Nuclear power plant employee",
            "Homer lives in Springfield with Marge, Bart, Lisa and Maggie.",
            "menu-user-avatar.png",
        ),
    ] {
        let media = option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets");
        let muted = theme.muted;
        accordion = accordion.item(
            sapid::accordion_item(key)
                .label(name)
                .trigger_content(move || {
                    glory::ui::group().fill((
                        glory::ui::image()
                            .decorative()
                            .source(format!("{media}/{photo}"))
                            .style(
                                Style::new()
                                    .width(Length::Px(56.))
                                    .height(Length::Px(56.))
                                    .radius(28.),
                            ),
                        text().class("site-character-name").content(name),
                        text()
                            .class("site-character-description")
                            .content(description)
                            .style(Style {
                                color: Some(muted),
                                font_size: Some(14.),
                                ..Style::new()
                            }),
                    ))
                })
                .content(move || text().content(body)),
        );
    }
    accordion
}
