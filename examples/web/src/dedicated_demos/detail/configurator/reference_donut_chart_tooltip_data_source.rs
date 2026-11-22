use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme:sapid::Theme) -> impl Widget {
 let plots=[false,true].map(|all|{
 let key=if all {"donut-tooltip-all"} else {"donut-tooltip-one"};
 let description=if all {"Data only for all segments"} else {"Data only for hovered segment"};
 let chart=donut_chart(key).theme(theme.clone()).name(description)
  .data(controls::segment_example_data()).chart_theme(controls::chart_palette(&theme))
  .frame(ChartFrame{width:160.,height:160.}).default_options(PlotOptions{
    tooltip:true, tooltip_all_segments:all,angles:Some(PlotAngles{start:0.,sweep:360.}),counterclockwise:true,donut_inner_ratio:Some(0.75), ..Default::default()
 }).style(Style::new().width(Length::Px(160.)).max_width(Limit::Percent(100.)))
 .navigation(false).legend(false).data_table(false);
 sapid::stack().theme(theme.clone()).gap(8.).fill((sapid::text().content(description),chart))
 });
 sapid::group().theme(theme).gap(50.).fill(plots.into_iter().collect::<Vec<_>>())
}
