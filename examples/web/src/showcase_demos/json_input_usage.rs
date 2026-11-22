use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("json-settings")
        .class("site-json-usage")
        .theme(theme)
        .label("Your package.json")
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(400.)),
        )
        .control(|field| {
            sapid::json_input("json-settings-editor")
                .field(field)
                .rows(4)
                .autosize(true)
                .placeholder("Textarea will autosize to fit the content")
                .format_on_blur(true)
                .show_formatter(false)
        })
}
