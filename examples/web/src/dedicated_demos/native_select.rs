use glory::prelude::*;
use sapid::{NativeSelectVariant, Size, Theme};

const SIZES: [Size; 5] = [
    Size::ExtraSmall,
    Size::Small,
    Size::Medium,
    Size::Large,
    Size::ExtraLarge,
];
const HEIGHTS: [f64; 5] = [30., 36., 42., 50., 60.];
const RADII: [f64; 5] = [2., 4., 8., 16., 32.];
const STEPS: [&str; 5] = ["xs", "sm", "md", "lg", "xl"];

fn scale(key: &str, label: &'static str, step: Cage<f64>, theme: &Theme) -> Box<dyn Widget> {
    Box::new(
        sapid::stack().theme(theme.clone()).gap(6.).fill((
            sapid::text()
                .theme(theme.clone())
                .class("site-control-label")
                .content(label),
            sapid::slider(key.to_owned())
                .theme(theme.clone())
                .name(label)
                .min(0.)
                .max(4.)
                .step(1.)
                .marks((0..5).map(|i| sapid::SliderMark::unlabeled(f64::from(i))))
                .value_text(step.map(|i| Some(STEPS[*i as usize].to_owned())))
                .value(step)
                .on_value_change(move |next| step.revise(|value| *value = next)),
        )),
    )
}

pub fn demo(theme: Theme) -> impl Widget {
    widget(move |scope| {
        let selection = scope.cage(Some("react".to_owned()));
        let variant = scope.cage(Some("default".to_owned()));
        let size = scope.cage(1.);
        let radius = scope.cage(2.);
        let label = scope.cage("Input label".to_owned());
        let description = scope.cage("Input description".to_owned());
        let error = scope.cage(String::new());
        let asterisk = scope.cage(false);
        let palette = theme.clone();
        let preview = each(scope.bond(move || vec![(variant.get(), size.get() as usize)]))
            .key(Clone::clone)
            .children(move |entry| {
                let (variant, index) = entry.value.get();
                let variant = match variant.as_deref() {
                    Some("filled") => NativeSelectVariant::Filled,
                    Some("unstyled") => NativeSelectVariant::Unstyled,
                    _ => NativeSelectVariant::Default,
                };
                sapid::field("native-framework")
                    .theme(palette.clone())
                    .size(SIZES[index])
                    .style(
                        Style::new()
                            .width(Length::Percent(100.))
                            .max_width(Limit::Px(340.))
                            .gap(2.),
                    )
                    .label(label)
                    .with_asterisk(asterisk)
                    .description(description)
                    .error(error)
                    .control(move |field| {
                        sapid::native_select("native-framework-select")
                            .field(field)
                            .variant(variant)
                            .style(radius.map(move |i| {
                                Style::new()
                                    .radius(RADII[*i as usize])
                                    .min_height(Length::Px(HEIGHTS[index]))
                            }))
                            .options(
                                [("react", "React"), ("angular", "Angular"), ("vue", "Vue")]
                                    .map(|(key, label)| sapid::select_option(key, label)),
                            )
                            .value(selection)
                            .on_value_change(move |next| selection.revise(|value| *value = next))
                    })
            });
        view()
            .class("site-configurator site-native-select-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::variant(variant, &theme),
                    scale("native-size", "Size", size, &theme),
                    scale("native-radius", "Radius", radius, &theme),
                    controls::text_field("Label", label, &theme),
                    controls::switch("With asterisk", asterisk, &theme),
                    controls::text_field("Description", description, &theme),
                    controls::text_field("Error", error, &theme),
                ],
            ))
            .show_in(scope)
    })
}
