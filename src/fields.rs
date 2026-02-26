use num_complex::Complex32;

use crate::vectorspace::Field;

impl Field for f32 {
    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }

    fn inv(&self) -> Option<Self> {
        if self.is_zero() {
            None
        } else {
            Some(1.0 / *self)
        }
    }

    fn is_zero(&self) -> bool {
        *self == 0.0
    }

    fn conjugate(self) -> Self {
        self
    }
}

impl Field for Complex32 {
    fn zero() -> Self {
        Complex32::new(0.0, 0.0)
    }

    fn one() -> Self {
        Complex32::new(1.0, 0.0)
    }

    fn inv(&self) -> Option<Self> {
        if self.is_zero() {
            None
        } else {
            Some(self.inv())
        }
    }

    fn is_zero(&self) -> bool {
        *self == Complex32::ZERO
    }

    fn conjugate(self) -> Self {
        self.conj()
    }
}
