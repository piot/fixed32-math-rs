/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/fixed32-math-rs
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

/*!
# Vector and Rect Library

This Rust crate that provides efficient 2D vector and rectangle operations using fixed-point arithmetic.
Designed for applications where fixed precision is preferred, this crate is ideal for scenarios such as graphics programming,
game development, and embedded systems where deterministic results are crucial.
*/

use core::fmt;
use core::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

use fixed32::Fp;

/// Represents a vector in a 2D space.
///
#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub struct Vector {
    pub x: Fp,
    pub y: Fp,
}

impl Vector {
    /// Creates a new `Vector` with the specified `x` and `y` components.
    ///
    /// # Parameters
    /// - `x`: The x-coordinate of the vector.
    /// - `y`: The y-coordinate of the vector.
    ///
    /// # Returns
    /// A `Vector` instance with the given `x` and `y` components.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed32::Fp;
    /// use fixed32_math::Vector;
    ///
    /// let v = Vector::new(Fp::from(2), Fp::from(3));
    /// assert_eq!(v.x, Fp::from(2));
    /// assert_eq!(v.y, Fp::from(3));
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(x: Fp, y: Fp) -> Self {
        Self { x, y }
    }

    /// Returns a `Vector` pointing to the left (negative x-axis direction).
    ///
    /// This is a convenience method to create a vector that represents a direction
    /// to the left in 2D space.
    ///
    /// # Returns
    /// A `Vector` instance with `x` set to `-1` and `y` set to `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed32::Fp;
    /// use fixed32_math::Vector;
    ///
    /// let left = Vector::left();
    /// assert_eq!(left.x, Fp::neg_one());
    /// assert_eq!(left.y, Fp::zero());
    /// ```
    #[inline]
    #[must_use]
    pub const fn left() -> Self {
        Self {
            x: Fp::neg_one(),
            y: Fp::zero(),
        }
    }

    /// Returns a `Vector` pointing to the right (positive x-axis direction).
    ///
    /// This is a convenience method to create a vector that represents a direction
    /// to the right in 2D space.
    ///
    /// # Returns
    /// A `Vector` instance with `x` set to `1` and `y` set to `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed32::Fp;
    /// use fixed32_math::Vector;
    ///
    /// let right = Vector::right();
    /// assert_eq!(right.x, Fp::one());
    /// assert_eq!(right.y, Fp::zero());
    /// ```
    #[inline]
    #[must_use]
    pub const fn right() -> Self {
        Self {
            x: Fp::one(),
            y: Fp::zero(),
        }
    }

    /// Returns a `Vector` pointing upwards (positive y-axis direction).
    ///
    /// This is a convenience method to create a vector that represents a direction
    /// upwards in 2D space.
    ///
    /// # Returns
    /// A `Vector` instance with `x` set to `0` and `y` set to `1`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed32::Fp;
    /// use fixed32_math::Vector;
    ///
    /// let up = Vector::up();
    /// assert_eq!(up.x, Fp::zero());
    /// assert_eq!(up.y, Fp::one());
    /// ```
    #[inline]
    #[must_use]
    pub const fn up() -> Self {
        Self {
            x: Fp::zero(),
            y: Fp::one(),
        }
    }

    /// Returns a `Vector` pointing downwards (negative y-axis direction).
    ///
    /// This is a convenience method to create a vector that represents a direction
    /// downwards in 2D space.
    ///
    /// # Returns
    /// A `Vector` instance with `x` set to `0` and `y` set to `-1`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed32::Fp;
    /// use fixed32_math::Vector;
    ///
    /// let down = Vector::down();
    /// assert_eq!(down.x, Fp::zero());
    /// assert_eq!(down.y, Fp::neg_one());
    /// ```
    #[inline]
    #[must_use]
    pub const fn down() -> Self {
        Self {
            x: Fp::zero(),
            y: Fp::neg_one(),
        }
    }

    /// Computes the squared length (magnitude) of the vector.
    ///
    /// This method calculates the squared length of the vector, which is the sum of the
    /// squares of its `x` and `y` components. It is often used for performance reasons when
    /// the actual length is not needed, as computing the square root can be costly.
    ///
    /// # Returns
    /// The squared length of the vector as an [`Fp`] value.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed32::Fp;
    /// use fixed32_math::Vector;
    ///
    /// let v = Vector::new(Fp::from(3), Fp::from(4));
    /// let sqr_len = v.sqr_len();
    /// assert_eq!(sqr_len, Fp::from(25));
    /// ```
    #[must_use]
    pub fn sqr_len(&self) -> Fp {
        self.x * self.x + self.y * self.y
    }

    /// Returns the length of the vector.
    #[must_use]
    pub fn len(&self) -> Fp {
        self.sqr_len().sqrt()
    }

    /// Returns a normalized vector with length 1. Returns `None` if the vector is zero-length.
    #[must_use]
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
    #[must_use]
    pub fn dot(&self, other: &Self) -> Fp {
        self.x * other.x + self.y * other.y
    }

    /// Computes the magnitude of the cross product in 2D (which is a scalar value).
    #[must_use]
    pub fn cross(&self, other: &Self) -> Fp {
        self.x * other.y - self.y * other.x
    }

    /// Scales the vector by another vector component-wise.
    #[must_use]
    pub fn scale(&self, factor: &Self) -> Self {
        Self {
            x: self.x * factor.x,
            y: self.y * factor.y,
        }
    }

    /// Rotates the vector by the given angle in radians.
    #[must_use]
    pub fn rotate(&self, angle: Fp) -> Self {
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();
        Self {
            x: self.x * cos_angle - self.y * sin_angle,
            y: self.x * sin_angle + self.y * cos_angle,
        }
    }

    /// Returns the absolute value of each component of the vector.
    #[must_use]
    pub const fn abs(&self) -> Self {
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
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for Vector {
    type Output = Self;

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

impl Mul<Self> for Vector {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
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
    type Output = Self;

    fn mul(self, rhs: Fp) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<Self> for Vector {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
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
    type Output = Self;

    fn div(self, rhs: Fp) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Div<i16> for Vector {
    type Output = Self;

    fn div(self, rhs: i16) -> Self::Output {
        Self {
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
    type Output = Self;

    fn mul(self, rhs: i16) -> Self::Output {
        Self {
            x: self.x * Fp::from(rhs),
            y: self.y * Fp::from(rhs),
        }
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

/// Represents a rectangle in a 2D space.
///
/// The `Rect` struct is defined by its position (`pos`) and size (`size`), both of which are
/// represented as [`Vector`] instances. The position indicates the coordinates of the rectangle's
/// bottom-left corner, and the size indicates the width and height of the rectangle.
///
/// # Examples
///
/// Creating a new rectangle:
/// ```
/// use fixed32::Fp;
/// use fixed32_math::{Vector, Rect};
///
/// let pos = Vector::new(Fp::from(1), Fp::from(2));
/// let size = Vector::new(Fp::from(3), Fp::from(4));
/// let rect = Rect::new(pos, size);
/// ```
///
/// Accessing the position and size:
/// ```
/// use fixed32::Fp;
/// use fixed32_math::{Vector, Rect};
///
/// let pos = Vector::new(Fp::from(1), Fp::from(2));
/// let size = Vector::new(Fp::from(3), Fp::from(4));
/// let rect = Rect::new(pos, size);
/// assert_eq!(rect.pos.x, Fp::from(1));
/// assert_eq!(rect.size.y, Fp::from(4));
/// ```
#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub struct Rect {
    pub pos: Vector,
    pub size: Vector,
}

impl Rect {
    /// Creates a new `Rect` with the specified position and size.
    ///
    /// # Parameters
    /// - `pos`: The position of the rectangle's top-left corner as a [`Vector`].
    /// - `size`: The size of the rectangle, including its width and height, as a [`Vector`].
    ///
    /// # Returns
    /// A `Rect` instance with the given position and size.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed32::Fp;
    /// use fixed32_math::{Vector, Rect};
    ///
    /// let pos = Vector::new(Fp::from(1), Fp::from(2));
    /// let size = Vector::new(Fp::from(3), Fp::from(4));
    /// let rect = Rect::new(pos, size);
    /// assert_eq!(rect.pos, pos);
    /// assert_eq!(rect.size, size);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(pos: Vector, size: Vector) -> Self {
        Self { pos, size }
    }

    #[must_use]
    #[inline(always)]
    pub fn top(self) -> Fp {
        self.pos.y + self.size.y
    }

    #[inline(always)]
    #[must_use]
    pub const fn bottom(self) -> Fp {
        self.pos.y
    }

    #[inline(always)]
    #[must_use]
    pub const fn left(self) -> Fp {
        self.pos.x
    }

    #[inline(always)]
    #[must_use]
    pub fn right(self) -> Fp {
        self.pos.x + self.size.x
    }

    /// Returns a new `Rect` with its position translated by the given vector.
    ///
    /// This method is useful for moving the rectangle while keeping its size unchanged.
    ///
    /// # Parameters
    /// - `offset`: A vector indicating how much to translate the rectangle's position.
    ///
    /// # Returns
    /// A new `Rect` with the position translated by the given vector. The size remains unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed32::Fp;
    /// use fixed32_math::{Rect, Vector};
    ///
    /// let pos = Vector::new(Fp::from(1), Fp::from(2));
    /// let size = Vector::new(Fp::from(3), Fp::from(4));
    /// let rect = Rect::new(pos, size);
    /// let offset = Vector::new(Fp::from(1), Fp::from(-1));
    /// let translated_rect = rect.move_by(offset);
    /// assert_eq!(translated_rect.pos.x, Fp::from(2));
    /// assert_eq!(translated_rect.pos.y, Fp::from(1));
    /// assert_eq!(translated_rect.size, size);
    /// ```
    #[must_use]
    pub fn move_by(self, offset: Vector) -> Self {
        Self {
            pos: self.pos + offset,
            size: self.size,
        }
    }

    /// Calculates the area of the rectangle.
    #[must_use]
    pub fn area(&self) -> Fp {
        self.size.x * self.size.y
    }

    /// Calculates the perimeter of the rectangle.
    #[must_use]
    pub fn perimeter(&self) -> Fp {
        2 * (self.size.x + self.size.y)
    }

    /// Calculates the intersection of two rectangles.
    #[must_use]
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
    #[must_use]
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
    #[must_use]
    pub fn contains_point(&self, point: &Vector) -> bool {
        point.x >= self.pos.x
            && point.x < self.pos.x + self.size.x
            && point.y >= self.pos.y
            && point.y < self.pos.y + self.size.y
    }

    /// Checks if another rectangle is completely inside this rectangle.
    #[must_use]
    pub fn contains_rect(&self, other: &Self) -> bool {
        self.contains_point(&other.pos) && self.contains_point(&(other.pos + other.size))
    }

    #[inline]
    #[must_use]
    pub fn is_overlapping(self, other: Self) -> bool {
        !(self.right() < other.left()
            || self.left() > other.right()
            || self.bottom() > other.top()
            || self.top() < other.bottom())
    }

    /// Expands the rectangle by a given offset.
    #[must_use]
    pub fn expanded(&self, offset: Vector) -> Self {
        Self {
            pos: self.pos - offset,
            size: self.size + offset * Fp::from(2.0),
        }
    }

    /// Contracts the rectangle by a given offset.
    #[must_use]
    pub fn contracted(&self, offset: Vector) -> Self {
        Self {
            pos: self.pos + offset,
            size: self.size - offset * Fp::from(2.0),
        }
    }

    /// Calculates the aspect ratio of the rectangle.
    #[must_use]
    pub fn aspect_ratio(&self) -> Fp {
        self.size.x / self.size.y
    }
}

impl fmt::Debug for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "rect:({:?},{:?},{:?},{:?})",
            self.pos.x, self.pos.y, self.size.x, self.size.y
        )
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}, {}, {})",
            self.pos.x, self.pos.y, self.size.x, self.size.y
        )
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

impl From<(f32, f32, f32, f32)> for Rect {
    fn from(values: (f32, f32, f32, f32)) -> Self {
        Self {
            pos: Vector::from((values.0, values.1)),
            size: Vector::from((values.2, values.3)),
        }
    }
}
