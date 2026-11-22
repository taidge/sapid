use glory::prelude::*;

#[derive(Clone, Copy)]
enum Age {
    Years(u8),
    Adult,
}
impl Age {
    fn key(self) -> String {
        match self {
            Self::Years(years) => years.to_string(),
            Self::Adult => "adult".into(),
        }
    }
}
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let age = scope.cage(Age::Years(16));
        sapid::segmented_control("age-selection")
            .theme(theme.clone())
            .name("Age")
            .value(age.map(|v| v.key()))
            .segments([
                sapid::segment("16").label("16"),
                sapid::segment("17").label("17"),
                sapid::segment("adult").label("18 or older"),
            ])
            .on_value_change(move |s| {
                age.revise(|v| *v = s.parse::<u8>().map_or(Age::Adult, Age::Years))
            })
            .show_in(scope)
    })
}
