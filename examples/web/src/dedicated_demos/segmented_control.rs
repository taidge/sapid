use glory::prelude::*;
use glory::render::Orientation;
use sapid::{Size, Theme};

const SCALE: [Size; 5] = [
    Size::ExtraSmall,
    Size::Small,
    Size::Medium,
    Size::Large,
    Size::ExtraLarge,
];
const LABELS: [&str; 5] = ["XS", "S", "M", "L", "XL"];
const RADII: [f64; 5] = [2., 4., 6., 12., 24.];

fn scale(key: &str, label: &str, step: Cage<f64>, theme: &Theme) -> impl Widget {
    let label = label.to_owned();
    sapid::stack().theme(theme.clone()).gap(6.).fill((
        sapid::text()
            .theme(theme.clone())
            .class("site-control-label")
            .content(step.map({
                let label = label.clone();
                move |index| format!("{label} · {}", LABELS[*index as usize])
            })),
        sapid::slider(key.to_owned())
            .theme(theme.clone())
            .name(label)
            .min(0.)
            .max(4.)
            .step(1.)
            .marks((0..5).map(|index| sapid::SliderMark::unlabeled(f64::from(index))))
            .value(step)
            .value_text(step.map(|index| Some(LABELS[*index as usize].to_owned())))
            .on_value_change(move |next| step.revise(|value| *value = next)),
    ))
}

pub fn demo(theme: Theme) -> impl Widget {
    widget(move |scope| {
        let interval = scope.cage("react".to_owned());
        let orientation = scope.cage("horizontal".to_owned());
        let full = scope.cage(false);
        let borders = scope.cage(true);
        let size = scope.cage(1.);
        let radius = scope.cage(2.);

        let layout = scope.bond(move || vec![(orientation.get(), size.get() as usize)]);
        let preview_theme = theme.clone();
        let preview = each(layout)
            .key(|layout| layout.clone())
            .children(move |entry| {
                let (axis, size) = entry.value.get();
                sapid::segmented_control("billing-interval")
                    .theme(preview_theme.clone())
                    .name("Framework")
                    .value(interval)
                    .orientation(if axis == "vertical" {
                        Orientation::Vertical
                    } else {
                        Orientation::Horizontal
                    })
                    .size(SCALE[size])
                    .radius(radius.map(|index| RADII[*index as usize]))
                    .full_width(full)
                    .with_item_borders(borders)
                    .segments([
                        sapid::segment("react").label("React"),
                        sapid::segment("angular").label("Angular"),
                        sapid::segment("vue").label("Vue"),
                    ])
                    .on_value_change(move |next| interval.revise(|value| *value = next))
            });

        view()
            .class("site-configurator site-field-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    Box::new(
                        sapid::segmented_control("billing-orientation")
                            .theme(theme.clone())
                            .size(Size::Small)
                            .label("Orientation")
                            .full_width(true)
                            .value(orientation)
                            .segments([
                                sapid::segment("horizontal").label("Horizontal"),
                                sapid::segment("vertical").label("Vertical"),
                            ])
                            .on_value_change(move |next| orientation.revise(|value| *value = next)),
                    ),
                    controls::switch("Full width", full, &theme),
                    controls::switch("With items borders", borders, &theme),
                    Box::new(scale("billing-size", "Size", size, &theme)),
                    Box::new(scale("billing-radius", "Radius", radius, &theme)),
                ],
            ))
            .show_in(scope)
    })
}
