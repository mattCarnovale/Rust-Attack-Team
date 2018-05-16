/*
Copyright (c) 2018 Matt Carnovale, Julie Rutherford-Fields, Joshua Sander
This work is available under the "3-clause ('new') BSD License”.
Please see the file LICENSE in this distribution
for license terms.
*/

extern crate ggez;

use ggez::event::{Keycode, Mod};
use ggez::{GameResult, Context};
use ggez::graphics::{self, set_color, Color, DrawMode, Point2};
use ggez::conf;
use ggez::event;

//Square size of blank graphic.  Will be useless with Sprites.
const SQUARE_SIZE: f32 = 10.0;

// Screen dimensions. Currently portrait mode.
const WIN_W: u32 = 400;
const WIN_H: u32 = 700;

struct Square {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    colour: Color,
}

impl Square {
    fn new(_ctx: &mut Context, x: f32, y: f32, width: f32, height: f32, colour: Color) -> Square {
        Square {
            x: x,
            y: y,
            width: width,
            height: height,
            colour: colour,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        set_color(ctx, self.colour)?;
        let square = graphics::Rect::new(self.x, self.y, self.width, self.height);
        graphics::rectangle(ctx, DrawMode::Fill, square)?;
        Ok(())
    }
}

struct Crab {
    form: Square,
}

impl Crab {
    fn new(ctx: &mut Context) -> Crab {
        Crab {
            form: Square::new(
                ctx,
                5.0 * SQUARE_SIZE,
                WIN_H as f32 - 1.0 * SQUARE_SIZE,
                SQUARE_SIZE,
                SQUARE_SIZE,
                Color::new(0.0, 1.0, 0.0, 1.0),
            )
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.form.draw(ctx)?;
        Ok(())
    }

    pub fn move_up(&mut self) {
        if self.form.y - SQUARE_SIZE > 0.0 {
            self.form.y -= SQUARE_SIZE;
        }
    }
    pub fn move_down(&mut self) {
        if self.form.y + SQUARE_SIZE < WIN_H as f32 {
            self.form.y += SQUARE_SIZE;
        }
    }
    pub fn move_right(&mut self) {
        if self.form.x + SQUARE_SIZE < WIN_W as f32 - SQUARE_SIZE {
            self.form.x += SQUARE_SIZE;
        }
    }
    pub fn move_left(&mut self) {
        if self.form.x - SQUARE_SIZE > 0.0 {
            self.form.x -= SQUARE_SIZE;
        }
    }
}

// Contains properties to track during gameplay
// In this example it is only tracking the x coord of the orb
struct MainState {
    pos_x: f32,
    player: Crab,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState {
            pos_x: 0.0,
            player: Crab::new(_ctx)
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.pos_x = self.pos_x % 400.0 + 1.0; // Increments x value
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::circle(ctx,
                         DrawMode::Fill,
                         Point2::new(self.pos_x, 350.0),
                         10.0,
                         2.0)?;

        self.player.draw(ctx)?;

        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut ggez::Context, keycode: Keycode, _: Mod, _: bool) {
        match keycode {
            Keycode::Up => self.player.move_up(),
            Keycode::Down => self.player.move_down(),
            Keycode::Right => self.player.move_right(),
            Keycode::Left => self.player.move_left(),

            _ => {}
        }
    }
}

pub fn main() {
    let mut c = conf::Conf::new();
    c.window_setup.title 	= "C R A B B E R".to_string();
	c.window_mode.width 	= WIN_W;
    c.window_mode.height 	= WIN_H;
    let ctx 				= &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state 				= &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
  