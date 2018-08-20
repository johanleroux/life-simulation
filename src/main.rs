extern crate opengl_graphics;
extern crate piston_window;
extern crate rand;

use piston_window::{OpenGL, PistonWindow, Size, WindowSettings};
use opengl_graphics::GlGraphics;

mod config;
mod menu;
mod settings;

fn main() {
    let title = "Life Simulation";
    let window_size = Size {
        width: 1280,
        height: 720,
    };

    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new(
        title,
        [window_size.width, window_size.height],
    ).opengl(opengl)
        .samples(4)
        .fullscreen(false)
        .resizable(false)
        .exit_on_esc(false)
        .build()
        .unwrap_or_else(|error| panic!("Failed to build PistonWindow: {}", error));

    let mut gl = GlGraphics::new(opengl);

    menu::run(&mut window, &mut gl, window_size);
}
