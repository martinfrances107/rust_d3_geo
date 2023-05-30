use core::marker::PhantomData;

use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::Transform;

/// One of the 3-axis rotation transforms.
#[derive(Clone, Debug)]
pub struct RotationIdentity<T> {
    phantom: PhantomData<T>,
}

impl<T> Default for RotationIdentity<T> {
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
    fn transform(&self, p: &Coord<T>) -> Coord<T> {
        normalise(p)
    }

    #[inline]
    fn invert(&self, p: &Coord<T>) -> Coord<T> {
        normalise(p)
    }
}

#[inline]
fn normalise<T: CoordFloat + FloatConst>(p: &Coord<T>) -> Coord<T> {
    if p.x.abs() > T::PI() {
        Coord {
            x: p.x - (p.x / T::TAU()).round() * T::TAU(),
            y: p.y,
        }
    } else {
        *p
    }
}
