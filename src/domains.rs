use crate::vectorspace::Domain;

impl Domain for f32 {
    fn first() -> Self {
        f32::NEG_INFINITY
    }

    fn last() -> Self {
        f32::INFINITY
    }

    fn zero() -> Self {
        0.0
    }
}
