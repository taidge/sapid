use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let raw = scope.cage(String::new());
        sapid::form("mask-submit-phone")
            .theme(theme.clone())
            .on_submit(move |_| {
                super::field_form_feedback::submit(serde_json::json!({"phone":raw.get()}))
            })
            .fill(
                sapid::stack()
                    .class("site-field-state-example site-example-validation-form")
                    .gap(16.)
                    .style(
                        Style::new()
                            .width(Length::Percent(100.))
                            .max_width(Limit::Px(340.)),
                    )
                    .fill((
                        sapid::field("mask-form-field")
                            .label("Phone")
                            .form_name("phone")
                            .control(move |field| {
                                sapid::mask_input(
                                    sapid::MaskPattern::parse("(###) ###-####")
                                        .expect("valid phone mask"),
                                )
                                .field(field)
                                .value(raw)
                                .placeholder("(___) ___-____")
                                .on_value_change(move |next| raw.revise(|value| *value = next))
                            }),
                        sapid::submit_button().theme(theme.clone()).label("Submit"),
                    )),
            )
            .show_in(scope)
    })
}
