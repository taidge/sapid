use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let fruits = [
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
    // The example stylesheet enables horizontal resizing on the outer container.
    view().class("site-overflow-resizer").fill(
        sapid::overflow_list("fruit-badges")
            .theme(theme.clone())
            .gap(4.)
            .overflow_count_label(|hidden| format!("+{hidden} more"))
            .items(fruits.into_iter().map(move |name| {
                let theme = theme.clone();
                sapid::overflow_item(name.to_ascii_lowercase().replace(' ', "-")).content(
                    move || {
                        sapid::badge()
                            .theme(theme.clone())
                            .variant(sapid::BadgeVariant::Solid)
                            .content(name)
                    },
                )
            })),
    )
}
