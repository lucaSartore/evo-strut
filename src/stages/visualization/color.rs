use rerun::components::Color as RerunColor;

#[derive(Copy, Clone, Debug)]
pub enum Color {
    Blue,
    Red,
    Green,
    Rgb(u8, u8, u8)
}

impl From<Color> for RerunColor {
    fn from(value: Color) -> Self {
        match value {
            Color::Blue => RerunColor::from_rgb(0,0,255),
            Color::Green => RerunColor::from_rgb(0,255,0),
            Color::Red => RerunColor::from_rgb(255,0,0),
            Color::Rgb(r, g, b) => RerunColor::from_rgb(r, g, b)
        }
    }
}
