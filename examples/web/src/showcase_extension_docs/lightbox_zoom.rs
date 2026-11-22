use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let media = option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets");
        let photos = [
            ("lake", "alpine-lake-photo.png", "Alpine lake"),
            ("portrait", "team-member-photo.png", "Studio portrait"),
            ("mountain", "mountain-evening.svg", "Mountain evening"),
            (
                "village",
                "alpine-lake-photo.png#village",
                "Village waterfront",
            ),
            ("ridge", "alpine-lake-photo.png#ridge", "Mountain ridge"),
        ];
        let open = scope.cage(false);
        let view = scope.cage(sapid::LightboxView::default());
        let sources = photos
            .iter()
            .map(|&(key, file, label)| {
                sapid::lightbox_image(key, format!("{media}/{file}"), label).caption(label)
            })
            .collect::<Vec<_>>();
        let buttons = photos
            .into_iter()
            .map(|(key, file, label)| {
                sapid::button()
                    .theme(theme.clone())
                    .class("showcase-gallery-tile")
                    .name(format!("Open {label}"))
                    .label("")
                    .fill(
                        sapid::image(format!("{media}/{file}"))
                            .theme(theme.clone())
                            .alt(label)
                            .fit(glory::style::ObjectFit::Cover),
                    )
                    .on_press(move || {
                        view.revise(|value| {
                            *value = sapid::LightboxView {
                                current: Some(key.into()),
                                zoom: sapid::LightboxZoom::Fit,
                            }
                        });
                        open.revise(|value| *value = true)
                    })
            })
            .collect::<Vec<_>>();
        sapid::stack()
            .theme(theme.clone())
            .gap(8.)
            .fill((
                glory::ui::view()
                    .class("showcase-gallery-grid")
                    .fill(buttons),
                text().content("Open a photo, then use Zoom in, Zoom out or Fit image."),
                sapid::lightbox("zoom-gallery")
                    .theme(theme.clone())
                    .show_trigger(false)
                    .title("Image zoom")
                    .images(sources)
                    .view(view)
                    .open(open)
                    .on_view_change(move |value| view.revise(|old| *old = value))
                    .on_open_change(move |value| open.revise(|old| *old = value)),
            ))
            .show_in(scope)
    })
}
