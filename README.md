# Fixed32 Math

`fixed32_math` is a Rust crate that provides efficient 2D vector and rectangle operations using fixed-point arithmetic.
Designed for applications where fixed precision is preferred, this crate is ideal for scenarios such as graphics programming,
game development, and embedded systems where deterministic results are crucial.

## Overview

### Vector

The `Vector` struct represents a 2D vector and supports various operations including addition, subtraction, scaling,
normalization, and more. The vector components use fixed-point arithmetic via the `fixed32` crate.

#### Example Usage

```rust
use fixed32::Fp;
use your_crate_name::Vector;

// Create vectors using the `From` trait
let v1 = Vector::from((2, 3));  // Automatically converts (2, 3) to Vector with Fp::from
let v2 = Vector::from((1, 4));

// Vector operations
let sum = v1 + v2;
let dot_product = v1.dot(&v2);
let normalized = v1.normalize().unwrap();
let rotated = v1.rotate(Fp::from(90).to_radians());
```

### Rect

The `Rect` struct represents an axis-aligned rectangle defined by its lower left position and size.
It supports operations such as intersection, union, expansion, and contraction, as well as checking
if a point or another rectangle is contained within it.

#### Example Usage

```rust
use fixed32::Fp;
use your_crate_name::{Vector, Rect};

// Create Rect instances using `From` trait
let rect1 = Rect::from((0, 0, 10, 10));
let rect2 = Rect::from((5, 5, 15, 15));

// Rectangle operations
let intersection = rect1.intersection(&rect2);
let union = rect1.union(&rect2);
let contains_point = rect1.contains_point(&Vector::from((2, 2)));
let expanded_rect = rect1.expanded(Vector::from((2, 2)));
```

#### Using `From` Trait

The `From` trait is implemented for convenient creation of `Rect` and `Vector` from tuples of integers:

```rust
// Create a Rect from a tuple of integers
let rect = Rect::from((1, 2, 3, 4));

let rect2 = Rect::from((1.24, 2.34, 3.98, 4.01));

// Create a Vector from a tuple of integers
let vector = Vector::from((2, 2));
```

This feature allows you to create `Rect` and `Vector` instances in a more concise manner without needing to
manually convert integer and float values to fixed-point numbers (`fixed32::Fp`).

## Features

- **Fixed-Point Arithmetic**: Uses `fixed32::Fp` for all calculations, avoiding floating-point inaccuracies.
- **Comprehensive Operations**: Includes basic arithmetic operations, normalization, rotation, and more for vectors.
- **Rectangle Operations**: Includes area, perimeter, intersection, union, and containment checks for rectangles.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
fixed32_math = "0.0.16"
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
