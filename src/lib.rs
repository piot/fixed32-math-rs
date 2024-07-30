/*----------------------------------------------------------------------------------------------------------
 *  Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/fixed32-math-rs
 *  Licensed under the MIT License. See LICENSE in the project root for license information.
 *--------------------------------------------------------------------------------------------------------*/
use core::fmt;
use core::ops::{Add, AddAssign, Div, Mul, Neg, Sub};
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

    #[inline]
    pub fn left() -> Self {
        Self {
            x: Fp::neg_one(),
            y: Fp::zero(),
        }
    }

    #[inline]
    pub fn right() -> Self {
        Self {
            x: Fp::one(),
            y: Fp::zero(),
        }
    }

    #[inline]
    pub fn up() -> Self {
        Self {
            x: Fp::zero(),
            y: Fp::one(),
        }
    }

    #[inline]
    pub fn down() -> Self {
        Self {
            x: Fp::zero(),
            y: Fp::neg_one(),
        }
    }

    pub fn sqr_len(&self) -> Fp {
        self.x * self.x + self.y * self.y
    }

    /// Returns the length of the vector.
    pub fn len(&self) -> Fp {
        self.sqr_len().sqrt()
    }

    /// Returns a normalized vector with length 1. Returns `None` if the vector is zero-length.
    pub fn normalize(&self) -> Option<Self> {
        let length = self.len();
        if length.is_zero() {
            None
        } else {
            Some(Self {
                x: self.x / length,
                y: self.y / length,
            })
        }
    }

    /// Computes the dot product of this vector with another.
    pub fn dot(&self, other: &Self) -> Fp {
        self.x * other.x + self.y * other.y
    }

    /// Computes the magnitude of the cross product in 2D (which is a scalar value).
    pub fn cross(&self, other: &Self) -> Fp {
        self.x * other.y - self.y * other.x
    }

    /// Scales the vector by another vector component-wise.
    pub fn scale(&self, factor: &Self) -> Self {
        Self {
            x: self.x * factor.x,
            y: self.y * factor.y,
        }
    }

    /// Rotates the vector by the given angle in radians.
    pub fn rotate(&self, angle: Fp) -> Self {
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();
        Self {
            x: self.x * cos_angle - self.y * sin_angle,
            y: self.x * sin_angle + self.y * cos_angle,
        }
    }

    /// Returns the absolute value of each component of the vector.
    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
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

    pub fn with_offset(self, offset: Vector) -> Self {
        Self {
            pos: self.pos + offset,
            size: self.size,
        }
    }

    /// Calculates the area of the rectangle.
    pub fn area(&self) -> Fp {
        self.size.x * self.size.y
    }

    /// Calculates the perimeter of the rectangle.
    pub fn perimeter(&self) -> Fp {
        2 * (self.size.x + self.size.y)
    }

    /// Calculates the intersection of two rectangles.
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let x_overlap = Fp::max(self.pos.x, other.pos.x)
            ..Fp::min(self.pos.x + self.size.x, other.pos.x + other.size.x);
        let y_overlap = Fp::max(self.pos.y, other.pos.y)
            ..Fp::min(self.pos.y + self.size.y, other.pos.y + other.size.y);

        if x_overlap.is_empty() || y_overlap.is_empty() {
            None
        } else {
            Some(Self {
                pos: Vector {
                    x: x_overlap.start,
                    y: y_overlap.start,
                },
                size: Vector {
                    x: x_overlap.end - x_overlap.start,
                    y: y_overlap.end - y_overlap.start,
                },
            })
        }
    }

    /// Calculates the union of two rectangles.
    pub fn union(&self, other: &Self) -> Self {
        let x_min = Fp::min(self.pos.x, other.pos.x);
        let y_min = Fp::min(self.pos.y, other.pos.y);
        let x_max = Fp::max(self.pos.x + self.size.x, other.pos.x + other.size.x);
        let y_max = Fp::max(self.pos.y + self.size.y, other.pos.y + other.size.y);

        Self {
            pos: Vector { x: x_min, y: y_min },
            size: Vector {
                x: x_max - x_min,
                y: y_max - y_min,
            },
        }
    }

    /// Checks if a point is inside the rectangle.
    pub fn contains_point(&self, point: &Vector) -> bool {
        point.x >= self.pos.x
            && point.x < self.pos.x + self.size.x
            && point.y >= self.pos.y
            && point.y < self.pos.y + self.size.y
    }

    /// Checks if another rectangle is completely inside this rectangle.
    pub fn contains_rect(&self, other: &Self) -> bool {
        self.contains_point(&other.pos) && self.contains_point(&(other.pos + other.size))
    }

    /// Expands the rectangle by a given offset.
    pub fn expanded(&self, offset: Vector) -> Self {
        Self {
            pos: self.pos - offset,
            size: self.size + offset * Fp::from(2.0),
        }
    }

    /// Contracts the rectangle by a given offset.
    pub fn contracted(&self, offset: Vector) -> Self {
        Self {
            pos: self.pos + offset,
            size: self.size - offset * Fp::from(2.0),
        }
    }

    /// Calculates the aspect ratio of the rectangle.
    pub fn aspect_ratio(&self) -> Fp {
        self.size.x / self.size.y
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
