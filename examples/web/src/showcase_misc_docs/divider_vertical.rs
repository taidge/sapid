use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let label = || {
        sapid::text()
            .theme(theme.clone())
            .font_size(16.)
            .line_height(24.8)
            .content("Label")
    };
    let mut children: Vec<Box<dyn Widget>> = vec![Box::new(label())];
    for thickness in [1., 2., 3., 4., 5.] {
        children.push(Box::new(
            sapid::separator()
                .theme(theme.clone())
                .orientation(glory::render::Orientation::Vertical)
                .length(Length::Px(24.8))
                .thickness(thickness),
        ));
        children.push(Box::new(label()));
    }
    sapid::group()
        .theme(theme)
        .justify(Justify::Start)
        .align(Align::Center)
        .gap(16.)
        .fill(children)
}
