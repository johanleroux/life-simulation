use std::f64;
use std::f64::consts::PI;

use opengl_graphics::GlGraphics;
use piston_window::{Context, UpdateArgs};

pub mod player;
pub mod food;
mod vector;

const PI_MULT_2: f64 = 2.0 * PI;

pub trait Drawable {
    fn draw(&self, context: Context, graphics: &mut GlGraphics);
}

pub trait Updateable {
    fn update(&mut self, args: UpdateArgs, tick: u32);
}

pub trait Positioned {
    fn x(&self) -> f64 {
        self.pos().x
    }

    fn y(&self) -> f64 {
        self.pos().y
    }

    fn pos(&self) -> vector::Vector;
}

pub trait Collidable: Positioned {
    fn radius(&self) -> f64;

    fn collides_with<C: Collidable>(&self, other: &C) -> bool {
        let distance = ((self.x() - other.x()).powi(2) + (self.y() - other.y()).powi(2)).sqrt();
        distance < self.radius() + other.radius()
    }
}
