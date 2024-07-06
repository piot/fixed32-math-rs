/*----------------------------------------------------------------------------------------------------------
 *  Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/fixed32-math-rs
 *  Licensed under the MIT License. See LICENSE in the project root for license information.
 *--------------------------------------------------------------------------------------------------------*/
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
}
