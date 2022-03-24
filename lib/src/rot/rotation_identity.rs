use std::marker::PhantomData;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::Transform;

/// One of the 3-axis rotation transforms.
#[derive(Clone, Copy, Debug)]
pub struct RotationIdentity<T>
// where
//     T: CoordFloat,
{
    phantom: PhantomData<T>,
}

impl<T> Default for RotationIdentity<T>
where
    T: CoordFloat + FloatConst,
{
    #[inline]
    fn default() -> Self {
        Self {
            phantom: PhantomData::<T>,
        }
    }
}

impl<T> Transform for RotationIdentity<T>
where
    T: CoordFloat + FloatConst,
{
    /// f64 or f32.
    type T = T;

    #[inline]
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        normalise(p)
    }

    #[inline]
    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        normalise(p)
    }
}

#[inline]
fn normalise<T: CoordFloat + FloatConst>(p: &Coordinate<T>) -> Coordinate<T> {
    if p.x.abs() > T::PI() {
        Coordinate {
            x: p.x - (p.x / T::TAU()).round() * T::TAU(),
            y: p.y,
        }
    } else {
        *p
    }
}
