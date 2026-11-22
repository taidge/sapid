use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let body = theme.clone();
    let id = glory::render::SemanticId::new("reference-drawer-auto-focus").unwrap();
    sapid::drawer("reference-drawer-focus")
        .theme(theme)
        .label("Open drawer")
        .title("Focus demo")
        .initial_focus(id.clone())
        .content(move || {
            sapid::stack().theme(body.clone()).gap(8.).fill((
                text().content("First input"),
                sapid::text_field()
                    .theme(body.clone())
                    .name("First input")
                    .placeholder("First input")
                    .class("site-reference-field"),
                text().content("Input with initial focus"),
                glory::ui::input()
                    .name("Input with initial focus")
                    .class("site-reference-field")
                    .semantics(Some(Box::new(glory::render::Semantics {
                        id: Some(id.clone()),
                        placeholder: Some("It has data-autofocus attribute".to_owned()),
                        ..Default::default()
                    }))),
            ))
        })
}
