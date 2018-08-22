use std::f64;

use opengl_graphics::GlGraphics;
use piston_window::{polygon, types, Context, Transformed, UpdateArgs};

use num;
use config;
use config::color;
use simulation::models::{Collidable, Drawable, PI_MULT_2, Positioned, Updateable};
use simulation::models::vector::Vector;

pub struct Player {
    pub pos: Vector,
    pub rot: f64,
    pub actions: Actions,
    pub food: i32,
    pub score: i32,
}

#[derive(Default)]
pub struct Actions {
    pub rotate_cw: bool,
    pub rotate_ccw: bool,
    pub move_forward: bool,
    pub move_backward: bool,
}

const ROTATION_INCREMENT: f64 = 5.0;
const MOVEMENT_INCREMENT: f64 = 2.5;

impl Player {
    pub fn new() -> Self {
        Player {
            pos: Vector {
                x: f64::from(config::WINDOW_SIZE.width) / 2.0,
                y: f64::from(config::WINDOW_SIZE.height) / 2.0,
            },
            rot: 0.0,
            actions: Actions::default(),
            food: 100,
            score: 0
        }
    }

    pub fn eat(&mut self) {
        self.score += 1;
        self.food = num::clamp(self.food + 5, 0, 100);
    }

    fn rotate_player(&mut self, rot: f64) {
        self.rot = (self.rot + rot) % PI_MULT_2;
    }

    fn rotate_cw(&mut self, delta: f64) {
        self.rotate_player(ROTATION_INCREMENT * delta)
    }

    fn rotate_ccw(&mut self, delta: f64) {
        self.rotate_player(-1.0 * ROTATION_INCREMENT * delta)
    }

    fn move_player(&mut self, rot: f64) {
        self.pos = Vector {
            x: num::clamp(
                self.pos.x + (rot * self.rot.cos()),
                0.0, 
                config::WINDOW_SIZE.width as f64
            ),
            y: num::clamp(
                self.pos.y + (rot * self.rot.sin()),
                0.0, 
                config::WINDOW_SIZE.height as f64
            )
        };
    }

    fn move_forward(&mut self) {
        self.move_player(MOVEMENT_INCREMENT);
    }

    fn move_backward(&mut self) {
        self.move_player(-1.0 * MOVEMENT_INCREMENT);
    }
}

impl Updateable for Player {
    fn update(&mut self, args: UpdateArgs, tick: u32) {
        if self.actions.rotate_cw {
            self.rotate_cw(args.dt)
        }
        if self.actions.rotate_ccw {
            self.rotate_ccw(args.dt)
        }
        if self.actions.move_forward {
            self.move_forward()
        }
        if self.actions.move_backward {
            self.move_backward()
        }

        if tick % (30.0 * 1.2) as u32 == 0 {
            self.food = num::clamp(self.food - 1, 0, 100);
        }
    }
}

const PLAYER_HEIGHT: f64 = 16.0;
const PLAYER_WIDTH: f64 = 20.0;
const PLAYER: &types::Triangle = &[
    [0.0, -1.0 * PLAYER_HEIGHT / 2.0],
    [PLAYER_WIDTH, 0.0],
    [0.0, PLAYER_HEIGHT / 2.0],
];
impl Drawable for Player {
    fn draw(&self, context: Context, graphics: &mut GlGraphics) {
        polygon(
            color::CYAN,
            PLAYER,
            context.transform
                    .trans(self.pos.x, self.pos.y)
                    .rot_rad(self.rot)
                    .trans(-1.0 * PLAYER_HEIGHT / 2.0, 0.0),
            graphics,
        );
    }
}

impl Positioned for Player {
    fn pos(&self) -> Vector {
        self.pos
    }
}

impl Collidable for Player {
    fn radius(&self) -> f64 {
        PLAYER_WIDTH / 2.0
    }
}
