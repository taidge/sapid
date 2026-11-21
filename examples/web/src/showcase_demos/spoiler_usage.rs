use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let media = option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets");
    sapid::spoiler("lakeside-guide")
        .theme(theme.clone())
        .class("showcase-walking-guide")
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(560.)),
        )
        .max_height(112.)
        .labels("Read the full walking guide", "Show a shorter guide")
        .illustration(
            sapid::image(format!("{media}/alpine-lake-photo.png"))
                .theme(theme)
                .alt("The lakeside village below the alpine peaks")
                .width(Length::Percent(44.))
                .height(Length::Px(138.))
                .fit(glory::style::ObjectFit::Cover),
        )
        .content(GUIDE)
}

const GUIDE: &str = "Start at the ferry landing and follow the wooden boardwalk along the lake. From the last bench you can see the village across the water and the peaks reflected behind it.\n\nAt the boathouse, leave the shore and take the stone steps between the houses. The path opens onto a terrace above the rooftops. Stop here for the view before following the lane back to the bakery.\n\nFor the longer return, follow the pine trail beyond the chapel. At the fork, take the lower path and cross the footbridge to rejoin the boardwalk near the ferry landing.";
