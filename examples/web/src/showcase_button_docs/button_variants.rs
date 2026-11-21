use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let mut examples = [
        (sapid::ButtonVariant::Primary, "Filled"),
        (sapid::ButtonVariant::Secondary, "Default"),
        (sapid::ButtonVariant::Light, "Light"),
        (sapid::ButtonVariant::Outline, "Outline"),
        (sapid::ButtonVariant::Subtle, "Subtle"),
        (sapid::ButtonVariant::Danger, "Delete"),
    ]
    .into_iter()
    .map(|(variant, label)| {
        sapid::button()
            .theme(theme.clone())
            .variant(variant)
            .label(label)
    })
    .collect::<Vec<_>>();
    examples.push(
        sapid::button()
            .theme(theme.clone())
            .label("Custom fill")
            .auto_contrast(true)
            .style(Style {
                background: Some(glory::style::Color(252, 215, 80, 255)),
                ..Style::new()
            }),
    );
    sapid::group().theme(theme).gap(12.).fill(examples)
}
