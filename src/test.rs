/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/fixed32-math-rs
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

#[cfg(test)]
mod tests {
    use fixed32::Fp;

    use crate::{Rect, Vector};

    #[test]
    fn multiply_fp_vector() {
        let result = Fp::from(2) * Vector::from((10, 33));
        assert_eq!(result.x, Fp::from(20));
        assert_eq!(result.y, Fp::from(66));
    }

    #[test]
    fn multiply_vector_fp() {
        let result = Vector::from((10, 33)) * Fp::from(2);
        assert_eq!(result.x, Fp::from(20));
        assert_eq!(result.y, Fp::from(66));
    }

    #[test]
    fn rect_from() {
        let result = Rect::from((10, 33, 20, 30));
        assert_eq!(result.pos, Vector::from((10, 33)));
        assert_eq!(result.size, Vector::from((20, 30)));
    }

    #[test]
    fn rect_move() {
        let result = Rect::from((10, 33, 20, 30)).move_by(Vector::from((18, -2)));
        assert_eq!(result.pos, Vector::from((28, 31)));
        assert_eq!(result.size, Vector::from((20, 30)));
    }

    #[test]
    fn test_rotate_0_degrees() {
        let vector = Vector {
            x: Fp::from(1.0),
            y: Fp::from(0.0),
        };
        let angle = Fp::from(0);
        let rotated = vector.rotate(angle);
        assert_eq!(
            rotated,
            Vector {
                x: Fp::from(1.0),
                y: Fp::from(0.0)
            }
        );
    }

    #[test]
    fn test_rotate_90_degrees() {
        let vector = Vector {
            x: Fp::from(1.0),
            y: Fp::from(0.0),
        };
        let angle = Fp::FRAC_PI_2; // 90 degrees (π/2)
        let rotated = vector.rotate(angle);
        let expected_vector = Vector {
            x: Fp::from(0.0),
            y: Fp::from(1.0),
        };
        let len = (expected_vector - rotated).sqr_len();

        assert!(len < Fp::from(0.01));
    }

    #[test]
    fn test_rotate_270_degrees() {
        let vector = Vector::new(Fp::from(1.0), Fp::from(0.0));
        let angle = Fp::from(3.0 * std::f32::consts::FRAC_PI_2); // 270 degrees (3π/2)
        let rotated = vector.rotate(angle);
        let expected_vector = Vector::new(Fp::from(0.0), Fp::from(-1.0));
        let len = (expected_vector - rotated).sqr_len();
        assert!(len < Fp::from(0.01));
    }
    #[test]
    fn test_contains_point_inside() {
        let rect = Rect::new(Vector::from((0, 0)), Vector::from((10, 10)));
        let point_inside = Vector::from((5, 5));
        assert!(rect.contains_point(&point_inside));
    }

    #[test]
    fn test_contains_point_near_edge() {
        let rect = Rect::new(Vector::from((0, 0)), Vector::from((10, 10)));
        let point_on_edge = Vector::from((9, 5));
        assert!(rect.contains_point(&point_on_edge));
    }

    #[test]
    fn test_contains_point_outside() {
        let rect = Rect::new(Vector::from((0, 0)), Vector::from((10, 10)));
        let point_outside = Vector::from((10, 5));
        assert!(!rect.contains_point(&point_outside));
    }

    #[test]
    fn test_intersection_no_overlap() {
        let rect1 = Rect::new(Vector::from((0, 0)), Vector::from((10, 10)));
        let rect2 = Rect::new(Vector::from((15, 15)), Vector::from((10, 10)));
        assert_eq!(rect1.intersection(&rect2), None);
    }

    #[test]
    fn test_intersection_partial_overlap() {
        let rect1 = Rect::new(Vector::from((0, 0)), Vector::from((10, 10)));
        let rect2 = Rect::new(Vector::from((5, 5)), Vector::from((10, 10)));
        let expected = Rect::new(Vector::from((5, 5)), Vector::from((5, 5)));
        assert_eq!(rect1.intersection(&rect2), Some(expected));
    }

    #[test]
    fn test_intersection_full_overlap() {
        let rect1 = Rect::new(Vector::from((0, 0)), Vector::from((10, 10)));
        let rect2 = Rect::new(Vector::from((2, 2)), Vector::from((5, 5)));
        let expected = Rect::new(Vector::from((2, 2)), Vector::from((5, 5)));
        assert_eq!(rect1.intersection(&rect2), Some(expected));
    }
}
