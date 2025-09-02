use piston_window::types::Color;

pub const FOOD_COLOR: Color = [1.0, 1.0, 0.50, 1.0];
pub const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
pub const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];
//pub const SNAKE_COLOR: Color = [0.00, 1.0, 0.00, 1.0];
pub const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];
pub const WALL_COLOR: Color = [1.0, 0.40, 1.0, 1.0];

pub fn snake_color(l: u8) -> Color {
    let mut r: f32 = 0.00;
    let mut g: f32 = 1.0;
    let mut b: f32 = 0.10;

    if l <= 10 {
        r += lissage(l) as f32 / 10.0;
    } else {
        r = 1.0;
    }
    if l > 10 {
        if l <= 20 {
            g -= (lissage(l) as f32 / 10.0) as f32;
        } else {
            g = 0.0;
        }
    }
    if l > 20 {
        b += lissage(l) as f32 / 10.0;
    }

    [r, g, b, 1.0]
}

fn lissage(l: u8) -> u8 {
    let mut ret = l;
    while ret > 10 {
        ret -= 10;
    }
    ret
}
