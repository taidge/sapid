use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let outside = scope.cage(true);
        let palette = theme.clone();
        let gallery = each(outside.map(|v| vec![*v]))
            .key(Clone::clone)
            .seed(move |close| {
                let media = option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets");
                sapid::lightbox("outside-gallery")
                    .theme(palette.clone())
                    .label("Open gallery")
                    .title("Gallery")
                    .close_on_click_outside(close)
                    .images(vec![sapid::lightbox_image(
                        "lake",
                        format!("{media}/alpine-lake-photo.png"),
                        "Alpine lake",
                    )])
            });
        sapid::stack()
            .theme(theme.clone())
            .gap(8.)
            .fill((
                gallery,
                sapid::switch()
                    .theme(theme.clone())
                    .label("Close on click outside")
                    .checked(outside)
                    .on_checked_change(move |v| outside.revise(|old| *old = v)),
            ))
            .show_in(scope)
    })
}
