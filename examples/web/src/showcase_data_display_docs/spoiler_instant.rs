use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let media = option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets");
    sapid::spoiler("instant-guide").class("showcase-walking-guide")
        .style(Style::new().width(Length::Percent(100.)).max_width(Limit::Px(520.)))
        .theme(theme.clone()).max_height(120.).animated(false).labels("Show more","Hide")
        .illustration(sapid::image(format!("{media}/alpine-lake-photo.png")).theme(theme).alt("Alpine village beside the lake").height(Length::Px(150.)).width(Length::Percent(42.)).fit(glory::style::ObjectFit::Cover))
        .content("The walking trail leaves the lakeside village behind and climbs through larch forest. The trees open onto a terrace with a view across the water. From here the footpath follows a stone wall to the old chapel before dropping back toward the shore. Follow the wooden signs through the orchard and past the boat house to finish at the ferry landing.")
}
