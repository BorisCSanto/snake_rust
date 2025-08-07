use piston_window::color::WHITE;
use piston_window::*;

use rand::{Rng, rng};

use crate::color::{BORDER_COLOR, FOOD_COLOR, GAMEOVER_COLOR};
use crate::draw::{draw_block, draw_rectangle, draw_text};
use crate::snake::{Direction, Snake};
use crate::wall::Wall;

const RESTART_TIME: f64 = 5.0;

pub struct Game {
    snake: Snake,
    wall: Wall,

    food_exists: bool,
    food_x: i32,
    food_y: i32,

    width: i32,
    height: i32,

    game_over: bool,
    waiting_time: f64,
    moving_period: f64,

    go_on: bool,
}

impl Game {
    pub fn new(width: i32, height: i32, waiting_time: f64, level: i32) -> Game {
        let mut g = Game {
            snake: Snake::new(2, 2),
            wall: Wall::new(level, width, height),
            food_exists: true,
            food_x: 6,
            food_y: 4,
            width,
            height,
            game_over: false,
            waiting_time,
            moving_period: waiting_time,
            go_on: true,
        };
        g.wall.make_grille();
        return g;
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            match key {
                Key::Return => {
                    self.go_on = false;
                }
                _ => (),
            }
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => Some(self.snake.head_direction()),
        };

        if let Some(dir) = dir {
            if dir == self.snake.head_direction().opposite() {
                return;
            }
        }

        self.update_snake(dir);
    }

    pub fn draw(&mut self, context: &Context, g: &mut G2d, glyph_cache: &mut Glyphs) {
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, context, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, context, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, context, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, context, g);
        self.wall.draw(context, g);
        self.snake.draw(context, g);
        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, context, g);
        }

        if self.game_over {
            let text_x = self.width / 3;
            let text_y = self.height / 2;
            let text_size = self.width * 2 / 3;
            let text_size_2 = self.width / 3;

            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, context, g);
            draw_text(
                WHITE,
                text_x,
                text_y,
                "GAME OVER",
                text_size as u32,
                glyph_cache,
                context,
                g,
            );

            draw_text(
                WHITE,
                text_x,
                text_y + 10,
                "Press Enter to go to Menu",
                text_size_2 as u32,
                glyph_cache,
                context,
                g,
            );
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;
        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.food_exists {
            self.add_food();
        }

        if self.waiting_time > self.moving_period {
            self.update_snake(None);
        }
    }

    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }

    fn check_snake_is_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.overlap_tail(next_x, next_y) || self.wall.is_wall(next_x, next_y) {
            return false;
        }

        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    fn add_food(&mut self) {
        let mut random = rng();

        let mut food_x = random.random_range(1..self.width - 1);
        let mut food_y = random.random_range(1..self.height - 1);

        while self.snake.overlap_tail(food_x, food_y) || self.wall.is_wall(food_x, food_y) {
            food_x = random.random_range(1..self.width - 1);
            food_y = random.random_range(1..self.height - 1);
        }

        self.food_x = food_x;
        self.food_y = food_y;
        self.food_exists = true;
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_snake_is_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;
        self.game_over = false;
    }

    pub fn return_to_menu(&self) -> bool {
        !self.go_on
    }
}
