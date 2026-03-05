use rerun::components::Color as RerunColor;

#[derive(Copy, Clone, Debug)]
pub enum Color {
    Blue,
    Red,
    Green,
    White,
    Rgb(u8, u8, u8),
    Hsv(f32, f32, f32)
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    // Assuming: 
    // h is 0.0 - 360.0
    // s is 0.0 - 1.0
    // v is 0.0 - 1.0
    
    let c = v * s; // Chroma
    let hp = h / 60.0;
    let x = c * (1.0 - (hp % 2.0 - 1.0).abs());
    let m = v - c;

    let (r_prime, g_prime, b_prime) = if (0.0..1.0).contains(&hp) {
        (c, x, 0.0)
    } else if (1.0..2.0).contains(&hp) {
        (x, c, 0.0)
    } else if (2.0..3.0).contains(&hp) {
        (0.0, c, x)
    } else if (3.0..4.0).contains(&hp) {
        (0.0, x, c)
    } else if (4.0..5.0).contains(&hp) {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    let r = ((r_prime + m) * 255.0).round() as u8;
    let g = ((g_prime + m) * 255.0).round() as u8;
    let b = ((b_prime + m) * 255.0).round() as u8;
    (r,g,b)
}

impl From<Color> for RerunColor {
    fn from(value: Color) -> Self {
        match value {
            Color::Blue => RerunColor::from_rgb(0,0,255),
            Color::Green => RerunColor::from_rgb(0,255,0),
            Color::Red => RerunColor::from_rgb(255,0,0),
            Color::White => RerunColor::from_rgb(255,255,255),
            Color::Rgb(r, g, b) => RerunColor::from_rgb(r, g, b),
            Color::Hsv(h, s, v) => {
                let (r,g,b) = hsv_to_rgb(h, s, v);
                RerunColor::from_rgb(r, g, b)
            }
        }
    }
}
