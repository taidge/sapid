use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::text_list()
        .theme(theme.clone())
        .ordered(true)
        .start(3)
        .reversed(true)
        .items(
            [None, None, None]
                .into_iter()
                .enumerate()
                .map(|(i, reset)| {
                    let item = sapid::text_list_item().fill(
                        sapid::text()
                            .theme(theme.clone())
                            .inline(true)
                            .content(format!("Item {}", i + 1)),
                    );
                    if let Some(number) = reset {
                        item.value(number)
                    } else {
                        item
                    }
                }),
        )
}
