use piston_window::types;

pub const BLACK: types::Color = [0.0, 0.0, 0.0, 1.0];
pub const WHITE: types::Color = [1.0, 1.0, 1.0, 1.0];
pub const CYAN: types::Color = [0.0, 1.0, 1.0, 1.0];

pub struct ColoredText {
    pub color: types::Color,
    pub text: &'static str,
}