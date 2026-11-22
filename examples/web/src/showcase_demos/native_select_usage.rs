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

#[allow(unused_imports)]
mod controls {
    use glory::prelude::*;
    use glory::style::{Color, FlexWrap};
    use sapid::{Size, Theme};

    pub fn playground(preview: Box<dyn Widget>, controls: Vec<Box<dyn Widget>>) -> Box<dyn Widget> {
        Box::new(
            view()
                .class("site-button-playground")
                .style(
                    Style::new()
                        .display(Display::Flex)
                        .flex_wrap(FlexWrap::Wrap)
                        .gap(24.0),
                )
                .fill((
                    col()
                        .class("site-playground-preview")
                        .style(Style {
                            grow: Some(1.0),
                            align: Some(Align::Center),
                            justify: Some(Justify::Center),
                            ..Style::new()
                                .display(Display::Flex)
                                .flex_direction(FlexDirection::Column)
                                .gap(16.0)
                                .min_width(Length::Px(180.0))
                        })
                        .fill(preview),
                    col()
                        .class("site-playground-controls")
                        .style(
                            Style::new()
                                .display(Display::Flex)
                                .flex_direction(FlexDirection::Column)
                                .gap(16.0)
                                .width(Length::Px(240.0))
                                .max_width(Limit::Percent(100.0)),
                        )
                        .fill(controls),
                )),
        )
    }

    pub fn switch(label: &str, value: Cage<bool>, theme: &Theme) -> Box<dyn Widget> {
        Box::new(
            sapid::switch()
                .theme(theme.clone())
                .size(Size::Small)
                .with_thumb_indicator(true)
                .label(label.to_owned())
                .checked(value)
                .on_checked_change(move |next| value.revise(|current| *current = next)),
        )
    }

    pub fn text_field(label: &str, value: Cage<String>, theme: &Theme) -> Box<dyn Widget> {
        Box::new(
            sapid::field(format!("control-{label}"))
                .theme(theme.clone())
                .label(label.to_owned())
                .control(move |field| {
                    sapid::text_field()
                        .field(field)
                        .placeholder("Enter prop value")
                        .value(value)
                        .on_value_change(move |next| value.revise(|current| *current = next))
                }),
        )
    }

    pub fn variant(value: Cage<Option<String>>, theme: &Theme) -> Box<dyn Widget> {
        let current = value.map(|value| value.as_deref().unwrap_or("default").to_owned());
        Box::new(
            sapid::stack().theme(theme.clone()).gap(8.0).fill((
                sapid::text()
                    .theme(theme.clone())
                    .class("site-control-label")
                    .content("Variant"),
                sapid::segmented_control("control-variant")
                    .theme(theme.clone())
                    .class("site-variant-control")
                    .name("Variant")
                    .value(current)
                    .segments([
                        sapid::segment("default").label("Default"),
                        sapid::segment("filled").label("Filled"),
                        sapid::segment("unstyled").label("Unstyled"),
                    ])
                    .on_value_change(move |next| value.revise(|current| *current = Some(next))),
            )),
        )
    }
}
