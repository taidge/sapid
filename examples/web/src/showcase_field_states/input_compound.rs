use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let id = |part| glory::render::SemanticId::new(part).expect("valid example id");
    sapid::stack().gap(0.).fill((
        sapid::label(id("compound-caption"), id("compound-control"))
            .theme(theme.clone())
            .required(true)
            .content("Input label"),
        sapid::field_description(id("compound-description"))
            .theme(theme.clone())
            .content("Input description"),
        sapid::field_error(id("compound-error"))
            .theme(theme)
            .content("Input error"),
    ))
}
