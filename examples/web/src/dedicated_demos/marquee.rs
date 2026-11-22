use glory::prelude::*;
pub fn demo(theme:sapid::Theme)->impl Widget {
 sapid::marquee().theme(theme).text("Sapid logos in eight colors").visual_content(||crate::showcase_misc_docs::marquee_logos::logo_strip(false,false)).repeat(4).gap(16.).class("sapid-marquee-fade-horizontal").period_millis(40000u32)
}
