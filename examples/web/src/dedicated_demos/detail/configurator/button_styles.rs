use glory::prelude::*;

const PARTS: &[(&str, &str, &str)] = &[
    (
        "root",
        "Root element",
        ".sapid-button:has(.site-email-section)",
    ),
    (
        "loader",
        "Loader component, displayed only when `loading` prop is set",
        ".sapid-button-loader",
    ),
    (
        "inner",
        "Contains all other elements, child of the `root` element",
        ".sapid-button:has(.site-email-section) > *",
    ),
    (
        "section",
        "Left and right sections of the button",
        ".site-email-section",
    ),
    ("label", "Button children", ".sapid-button-label"),
];

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let preview = widget(move |scope| {
        let theme = theme.clone();
        let loading = scope.cage(false);
        let button = sapid::button()
            .theme(theme.clone())
            .label("Your email")
            .left_section(text().class("site-email-section").content("@"))
            .loading(loading)
            .on_press(move || loading.revise(|v| *v = true));
        (sapid::stack()
            .theme(theme.clone())
            .class("site-standalone-action")
            .fill((
                button,
                when(loading, move || {
                    sapid::button()
                        .theme(theme.clone())
                        .label("Reset")
                        .on_press(move || loading.revise(|v| *v = false))
                }),
            )))
        .show_in(scope)
    });
    crate::showcase_style_explorer::explorer("button-styles", Box::new(preview), PARTS)
}
