use glory::prelude::*;
use sapid::{DateLocale, HourCycle, LocalTime, Size, TimeFormat, TimeGridSpec, TimePrecision};

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let selected = scope.cage(None::<LocalTime>);
        let cycle = scope.cage("24h".to_owned());
        let seconds = scope.cage(false);
        let size = scope.cage(1.);
        let radius = scope.cage(1.);
        let palette = theme.clone();
        let preview = each(scope.bond(move || {
            vec![(
                cycle.get(),
                seconds.get(),
                size.get() as usize,
                radius.get() as usize,
            )]
        }))
        .key(Clone::clone)
        .children(move |entry| {
            let (cycle, seconds, index, radius) = entry.value.get();
            let mut theme = palette.clone();
            theme.radius = [2., 4., 8., 16., 32.][radius];
            sapid::time_grid("appointment-time")
                .theme(theme)
                .size(
                    [
                        Size::ExtraSmall,
                        Size::Small,
                        Size::Medium,
                        Size::Large,
                        Size::ExtraLarge,
                    ][index],
                )
                .name("Appointment time")
                .spec(
                    TimeGridSpec::new(
                        LocalTime::new(10, 0, 0).unwrap(),
                        LocalTime::new(21, 0, 0).unwrap(),
                        3600,
                        3,
                    )
                    .unwrap(),
                )
                .format(TimeFormat::new(
                    if cycle == "12h" {
                        HourCycle::H12
                    } else {
                        HourCycle::H24
                    },
                    if seconds {
                        TimePrecision::Second
                    } else {
                        TimePrecision::Minute
                    },
                    DateLocale::En,
                ))
                .value(selected)
                .on_value_change(move |next| selected.revise(|old| *old = next))
        });
        view()
            .class("site-configurator site-time-grid-configurator")
            .fill(controls::playground(
                Box::new(
                    sapid::stack().theme(theme.clone()).gap(8.).fill((
                        preview,
                        sapid::text()
                            .theme(theme.clone())
                            .content(selected.map(|value| {
                                value.map_or_else(String::new, |time| format!("Selected: {time}"))
                            })),
                    )),
                ),
                vec![
                    controls::segmented_choice(
                        "time-grid-format",
                        "Format",
                        cycle,
                        &[("12h", "12h"), ("24h", "24h")],
                        &theme,
                    ),
                    controls::switch("With seconds", seconds, &theme),
                    controls::field_scale("time-grid-size".into(), "Size", size, &theme),
                    controls::field_scale("time-grid-radius".into(), "Radius", radius, &theme),
                ],
            ))
            .show_in(scope)
    })
}
