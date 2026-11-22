use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let fruit = [
        "Apple",
        "Banana",
        "Cherry",
        "Date",
        "Elderberry",
        "Fig",
        "Grape",
        "Honeydew",
        "Indian Fig",
        "Jackfruit",
        "Kiwi",
        "Lemon",
        "Mango",
        "Nectarine",
        "Orange",
        "Papaya",
    ];
    sapid::overflow_list("overflow-rows")
        .theme(theme.clone())
        .class("showcase-fruit-overflow")
        .max_rows(2)
        .gap(4.)
        .overflow_label("More")
        .items(fruit.into_iter().map(move |name| {
            let theme = theme.clone();
            sapid::overflow_item(name.to_ascii_lowercase().replace(' ', "-"))
                .content(move || sapid::badge().theme(theme.clone()).content(name))
        }))
}
