/*----------------------------------------------------------------------------------------------------------
 *  Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/fixed32-math-rs
 *  Licensed under the MIT License. See LICENSE in the project root for license information.
 *--------------------------------------------------------------------------------------------------------*/
use std::ops::{Add, AddAssign, Div, Mul, Sub};

use fixed32::Fp;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Vector {
    pub x: Fp,
    pub y: Fp,
}

impl Vector {
    pub fn new(x: Fp, y: Fp) -> Self {
        Self { x, y }
    }

    pub fn new_from_int(x: i16, y: i16) -> Self {
        Self {
            x: Fp::from_int(x),
            y: Fp::from_int(y),
        }
    }

    pub fn from_float(x: f32, y: f32) -> Self {
        Self {
            x: Fp::from_float(x),
            y: Fp::from_float(y),
        }
    }

    pub fn sqr_len(&self) -> Fp {
        Fp(self.x.0 / 1000 * self.x.0 + self.y.0 / 1000 * self.y.0)
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

impl Div<i16> for Vector {
    type Output = Vector;

    fn div(self, rhs: i16) -> Self::Output {
        Vector {
            x: self.x / Fp::from_int(rhs),
            y: self.y / Fp::from_int(rhs),
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
            x: self.x * Fp::from_int(rhs),
            y: self.y * Fp::from_int(rhs),
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



