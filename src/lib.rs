/*----------------------------------------------------------------------------------------------------------
 *  Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/fixed32-math-rs
 *  Licensed under the MIT License. See LICENSE in the project root for license information.
 *--------------------------------------------------------------------------------------------------------*/
use core::fmt;
/*----------------------------------------------------------------------------------------------------------
 *  Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/fixed32-math-rs
 *  Licensed under the MIT License. See LICENSE in the project root for license information.
 *--------------------------------------------------------------------------------------------------------*/
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

use fixed32::Fp;

mod test;

#[derive(Default, PartialEq, Clone, Copy)]
pub struct Vector {
    pub x: Fp,
    pub y: Fp,
}

impl Vector {
    pub fn new(x: Fp, y: Fp) -> Self {
        Self { x, y }
    }

    pub fn left() -> Self {
        Self {
            x: Fp::neg_one(),
            y: Fp::zero(),
        }
    }

    pub fn right() -> Self {
        Self {
            x: Fp::one(),
            y: Fp::zero(),
        }
    }

    pub fn up() -> Self {
        Self {
            x: Fp::zero(),
            y: Fp::one(),
        }
    }

    pub fn down() -> Self {
        Self {
            x: Fp::zero(),
            y: Fp::neg_one(),
        }
    }

    pub fn sqr_len(&self) -> Fp {
        self.x * self.x + self.y * self.y
    }
}

impl fmt::Debug for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "vec:{},{}", self.x, self.y)
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "vec:{},{}", self.x, self.y)
    }
}

impl From<(i16, i16)> for Vector {
    fn from(values: (i16, i16)) -> Self {
        Self {
            x: Fp::from(values.0),
            y: Fp::from(values.1),
        }
    }
}

impl From<(f32, f32)> for Vector {
    fn from(values: (f32, f32)) -> Self {
        Self {
            x: Fp::from(values.0),
            y: Fp::from(values.1),
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul<Vector> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Mul<Vector> for Fp {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl Mul<Fp> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Fp) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<Vector> for Vector {
    type Output = Vector;

    fn div(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl Div<Vector> for i16 {
    type Output = Vector;

    fn div(self, rhs: Vector) -> Self::Output {
        Vector {
            x: (self / rhs.x),
            y: (self / rhs.y),
        }
    }
}

impl Div<Fp> for Vector {
    type Output = Vector;

    fn div(self, rhs: Fp) -> Self::Output {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Div<i16> for Vector {
    type Output = Vector;

    fn div(self, rhs: i16) -> Self::Output {
        Vector {
            x: self.x / Fp::from(rhs),
            y: self.y / Fp::from(rhs),
        }
    }
}

impl Mul<Vector> for i16 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl Mul<i16> for Vector {
    type Output = Vector;

    fn mul(self, rhs: i16) -> Self::Output {
        Vector {
            x: self.x * Fp::from(rhs),
            y: self.y * Fp::from(rhs),
        }
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Rect {
    pub pos: Vector,
    pub size: Vector,
}

impl Rect {
    pub fn new(pos: Vector, size: Vector) -> Self {
        Self { pos, size }
    }
}

impl From<(i16, i16, i16, i16)> for Rect {
    fn from(values: (i16, i16, i16, i16)) -> Self {
        Self {
            pos: Vector::from((values.0, values.1)),
            size: Vector::from((values.2, values.3)),
        }
    }
}
