use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

use piston_window::Size;
use rand;

/// Models an (x, y) coordinate value (such as position or velocity).
#[derive(Copy, Clone, Default)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn new_rand(x_min: f64, y_min: f64, x_max: f64, y_max: f64) -> Self {
        Vector {
            x: rand::random::<f64>() * (x_max - x_min) + x_min,
            y: rand::random::<f64>() * (y_max - y_min) + y_min,
        }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Rem for Vector {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        Vector {
            x: self.x % other.x,
            y: self.y % other.y,
        }
    }
}

impl RemAssign for Vector {
    fn rem_assign(&mut self, other: Self) {
        *self = *self % other;
    }
}

impl Div<Vector> for Vector {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        if other.x == 0.0 || other.y == 0.0 {
            Vector { x: 0.0, y: 0.0 }
        } else {
            Vector {
                x: self.x / other.x,
                y: self.y / other.y,
            }
        }
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        self / Vector { x: other, y: other }
    }
}

impl Mul<Vector> for Vector {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Vector {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Vector {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl MulAssign<Vector> for Vector {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl MulAssign<f64> for Vector {
    fn mul_assign(&mut self, other: f64) {
        *self = *self * other;
    }
}
impl DivAssign<Vector> for Vector {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

impl DivAssign<f64> for Vector {
    fn div_assign(&mut self, other: f64) {
        *self = *self / other;
    }
}

/// Define how a two dimensional `Size` can be converted to a two dimensional `Vector`.
/// Width is defined as the x unit and height is defined as the y unit.
impl From<Size> for Vector {
    fn from(size: Size) -> Self {
        Vector {
            x: f64::from(size.width),
            y: f64::from(size.height),
        }
    }
}

impl From<[f64; 2]> for Vector {
    fn from(list: [f64; 2]) -> Self {
        Vector {
            x: list[0],
            y: list[1],
        }
    }
}
