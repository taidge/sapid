use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let media = option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets");
        let images = [
            ("lake", "alpine-lake-photo.png", "Alpine lake"),
            ("ridge", "mountain-evening.svg", "Mountain evening"),
        ]
        .map(|(key, file, caption)| {
            sapid::lightbox_image(key, format!("{media}/{file}"), caption).caption(caption)
        })
        .to_vec();
        let count = scope.cage(0usize);
        let palette = theme.clone();
        sapid::lightbox("custom-slides-gallery")
            .theme(theme.clone())
            .label("Open lightbox with custom slides")
            .images(images)
            .render_slide(move |slide| -> Box<dyn Widget> {
                if slide.key == "lake" {
                    Box::new(
                        sapid::image(slide.source)
                            .theme(palette.clone())
                            .alt(slide.alternative)
                            .fit(glory::style::ObjectFit::Contain)
                            .style(
                                Style::new()
                                    .width(Length::Percent(100.))
                                    .height(Length::Px(320.)),
                            ),
                    )
                } else {
                    Box::new(
                        sapid::stack()
                            .theme(palette.clone())
                            .class("showcase-custom-slide")
                            .fill((
                                sapid::title(3)
                                    .theme(palette.clone())
                                    .content("Mountain field notes"),
                                text().content("This slide contains interactive content."),
                                sapid::button()
                                    .theme(palette.clone())
                                    .label(count.map(|n| format!("Save note ({n})")))
                                    .on_press(move || count.revise(|v| *v += 1)),
                            )),
                    )
                }
            })
            .show_in(scope)
    })
}
