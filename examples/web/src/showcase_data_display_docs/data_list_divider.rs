use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let mut list = sapid::data_list()
        .theme(theme.clone())
        .orientation(sapid::DataListOrientation::Horizontal)
        .label_width(130.)
        .with_divider(true)
        .name("Contact details");
    for (term, value) in [
        ("Name", "John Doe"),
        ("Email", "john@example.com"),
        ("Role", "Software Engineer"),
    ] {
        list = list.item(
            sapid::data_list_item()
                .term(term)
                .fill(sapid::text().theme(theme.clone()).content(value)),
        );
    }
    list
}
