use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::field_playground(
        theme,
        "mask-phone",
        "Input label",
        "Input description",
        move |field, style| {
            sapid::mask_input(sapid::MaskPattern::parse("(###) ###-####").unwrap())
                .field(field)
                .style(style)
                .placeholder("(___) ___-____")
        },
    )
}
