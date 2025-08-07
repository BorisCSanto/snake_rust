use piston_window::types::Color;
use piston_window::{Context, G2d, Glyphs, Transformed, rectangle, text};

const BLOCK_SIZE: f64 = 10.0;

pub fn to_coord(game_coord: i32) -> f64 {
    // cast en f64 puis multiplie par le block_size
    (game_coord as f64) * BLOCK_SIZE
}

pub fn draw_block(color: Color, x: i32, y: i32, context: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        context.transform,
        g,
    );
}

pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    context: &Context,
    g: &mut G2d,
) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        color,
        [
            gui_x,
            gui_y,
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        context.transform,
        g,
    );
}

pub fn draw_text(
    color: Color,
    x: i32,
    y: i32,
    text_to_write: &str,
    size: u32,
    cache: &mut Glyphs,
    context: &Context,
    g: &mut G2d,
) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    text(
        color,
        size,
        text_to_write,
        cache,
        context.transform.trans(gui_x, gui_y),
        g,
    )
    .expect("et merde");
}
