use opengl_graphics::{GlGraphics, GlyphCache};
use piston_window::{clear, text, Button, Context, Key, PistonWindow, PressEvent, ReleaseEvent, RenderEvent, Transformed, UpdateArgs, UpdateEvent};

use config::color;
use self::models::{food, player, Collidable, Drawable, Updateable};
mod models;

pub struct Simulation {
    player: player::Player,
    food: Vec<food::Food>
}

impl Simulation {
    pub fn new() -> Self {
        Simulation {
            player: player::Player::new(),
            food: Vec::new()
        }
    }

    pub fn is_game_over(&self) -> bool {
        return self.player.food == 0;
    }

    pub fn run(
        &mut self,
        window: &mut PistonWindow,
        opengl: &mut GlGraphics,
        glyph_cache: &mut GlyphCache,
    ) {
        let mut tick: u32 = 0;
        for _i in 1..10 {
            self.food.push(food::Food::new());
        }

        while let Some(event) = window.next() {
            if let Some(args) = event.render_args() {
                opengl.draw(args.viewport(), |context, graphics| {
                    self.draw(context, graphics, glyph_cache)
                });
            }

            if let Some(args) = event.update_args() {
                self.update(args, tick);

                tick = (tick + 1) % 120;
            }

            if let Some(Button::Keyboard(key)) = event.press_args() {
                match key {
                    Key::D => self.player.actions.rotate_cw = true,
                    Key::A => self.player.actions.rotate_ccw = true,
                    Key::S => self.player.actions.move_backward = true,
                    Key::W => self.player.actions.move_forward = true,
                    _ => {}
                }
            }

            if let Some(Button::Keyboard(key)) = event.release_args() {
                match key {
                    Key::D => self.player.actions.rotate_cw = false,
                    Key::A => self.player.actions.rotate_ccw = false,
                    Key::S => self.player.actions.move_backward = false,
                    Key::W => self.player.actions.move_forward = false,
                    _ => {}
                }
            }

            if self.is_game_over() {
                break;
            }
        }
    }

    fn draw(&self, context: Context, graphics: &mut GlGraphics, glyph_cache: &mut GlyphCache) {
        clear(color::BLACK, graphics);
        for food in &self.food {
            food.draw(context, graphics);
        }
        self.player.draw(context, graphics);

        text(
            color::WHITE,
            26,
            format!("Score: {}", self.player.score).as_str(),
            glyph_cache,
            context.transform.trans(20.0, 40.0),
            graphics,
        ).unwrap();

        text(
            color::WHITE,
            26,
            format!("Food: {}", self.player.food).as_str(),
            glyph_cache,
            context.transform.trans(20.0, 66.0),
            graphics,
        ).unwrap();
    }
}

impl Updateable for Simulation {
    fn update(&mut self, args: UpdateArgs, tick: u32) {
        self.player.update(args, tick);

        {
            let player = &mut self.player;
            let food = &mut self.food;

            if let Some(index) = food
                .iter()
                .position(|food| food.collides_with(player))
            {
                food.remove(index);
                player.eat();
            }
        }

        if tick % (30.0 * 1.2) as u32 == 0 {
            if self.food.len() < 25 {
                self.food.push(food::Food::new());
            }
        }
    }
}
