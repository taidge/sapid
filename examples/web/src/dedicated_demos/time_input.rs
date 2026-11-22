use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(Some(LocalTime::new(9, 30, 0).unwrap()));
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                controls::field_playground(
                    theme.clone(),
                    "meeting-time-editor",
                    "Meeting time",
                    "Enter a local time in 24-hour format.",
                    move |field, style| {
                        sapid::time_input("meeting-time-editor")
                            .field(field)
                            .style(style)
                            .value(value)
                            .placeholder("HH:MM")
                            .on_value_change(move |v| value.revise(|s| *s = v))
                    },
                ),
                sapid::time_value(value)
                    .theme(theme.clone())
                    .empty_label("No meeting time selected"),
            ))
            .show_in(scope)
    })
}
