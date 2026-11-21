use super::{Color, Theme};

fn luminance(color: Color) -> f64 {
    let channel = |value: u8| {
        let value = f64::from(value) / 255.;
        if value <= 0.04045 {
            value / 12.92
        } else {
            ((value + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * channel(color.0) + 0.7152 * channel(color.1) + 0.0722 * channel(color.2)
}
impl Theme {
    /// Keep the preferred foreground when its contrast is at least 4.5:1.
    /// Translucent backgrounds are composited over this theme's surface.
    pub fn contrasting_foreground(&self, background: Color, preferred: Color) -> Color {
        let blend = |front: Color, back: Color| {
            let alpha = u32::from(front.3);
            let channel = |value: u8, back: u8| {
                ((u32::from(value) * alpha + u32::from(back) * (255 - alpha) + 127) / 255) as u8
            };
            Color(
                channel(front.0, back.0),
                channel(front.1, back.1),
                channel(front.2, back.2),
                255,
            )
        };
        let background = blend(background, self.surface);
        let base = luminance(background);
        let contrast = |color: Color| {
            let text = luminance(blend(color, background));
            (text.max(base) + 0.05) / (text.min(base) + 0.05)
        };
        if contrast(preferred) >= 4.5 {
            preferred
        } else {
            let black = Color(0, 0, 0, 255);
            let white = Color(255, 255, 255, 255);
            if contrast(black) >= contrast(white) {
                black
            } else {
                white
            }
        }
    }
}
