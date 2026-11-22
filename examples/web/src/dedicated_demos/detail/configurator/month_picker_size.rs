use glory::prelude::*;
use sapid::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let size = scope.cage(1.);
        let selected = scope.cage(PeriodSelection::Single(None::<YearMonth>));
        let palette = theme.clone();
        let preview = each(size.map(|value| vec![*value as usize]))
            .key(|index| *index)
            .children(move |entry| {
                let provider = DatesProvider::new(CivilDate::parse_iso("2026-11-04").unwrap());
                sapid::month_picker("calendar-size", provider)
                    .size(controls::step_size(entry.value.get() as f64))
                    .theme(palette.clone())
                    .value(selected)
                    .on_value_change(move |value| selected.revise(|previous| *previous = value))
            });
        view()
            .class("site-configurator site-calendar-configurator")
            .fill(controls::playground(
                std::boxed::Box::new(preview),
                vec![controls::field_scale(
                    "calendar-size-control".into(),
                    "Size",
                    size,
                    &theme,
                )],
            ))
            .show_in(scope)
    })
}
