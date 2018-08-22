use opengl_graphics::GlGraphics;
use piston_window::{ellipse, types, Context, Transformed};

use config;
use config::color;
use simulation::models::{Collidable, Drawable, Positioned};
use simulation::models::vector::Vector;

pub struct Food {
    pos: Vector
}

impl Food {
    pub fn new() -> Self {
        Food {
            pos: Vector::new_rand(0.0, 0.0, config::WINDOW_SIZE.width as f64, config::WINDOW_SIZE.height as f64)
        }
    }
}

const FOOD_DIAMETER: f64 = 3.0;
impl Drawable for Food {
    fn draw(&self, context: Context, graphics: &mut GlGraphics) {
        const FOOD: types::Rectangle = [0.0, 0.0, FOOD_DIAMETER, FOOD_DIAMETER];

        ellipse(
            color::WHITE,
            FOOD,
            context.transform.trans(self.pos.x, self.pos.y),
            graphics,
        )
    }
}

impl Positioned for Food {
    fn pos(&self) -> Vector {
        self.pos
    }
}

impl Collidable for Food {
    fn radius(&self) -> f64 {
        FOOD_DIAMETER / 2.0
    }
}
