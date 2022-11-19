use std::marker::PhantomData;

use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::Transform;

///Projection definition.
///
/// Why is the Phantom Data is required here...
///
/// The Raw trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Clone, Debug)]
pub struct ConicEqualArea<DRAIN, T> {
    c: T,
    p_drain: PhantomData<DRAIN>,
    n: T,
    r0: T,
    two: T,
}

impl<DRAIN, T> ConicEqualArea<DRAIN, T> {
    pub(super) const fn new(c: T, n: T, r0: T, two: T) -> Self {
        Self {
            c,
            p_drain: PhantomData::<DRAIN>,
            n,
            r0,
            two,
        }
    }
}

impl<DRAIN, T> Transform for ConicEqualArea<DRAIN, T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn transform(&self, p: &Coord<T>) -> Coord<T> {
        let r = (self.c - self.two * self.n * p.y.sin()).sqrt() / self.n;
        let x = p.x * self.n;
        Coord {
            x: r * x.sin(),
            y: self.r0 - r * x.cos(),
        }
    }

    #[inline]
    fn invert(&self, p: &Coord<T>) -> Coord<T> {
        let r0y = self.r0 - p.y;
        let mut l = p.x.atan2(r0y.abs()) * r0y.signum();
        if r0y * self.n < T::zero() {
            l = l - T::PI() * p.x.signum() * r0y.signum();
        }

        Coord {
            x: l / self.n,
            y: ((self.c - (p.x * p.x + r0y * r0y) * self.n * self.n) / (self.two * self.n)).asin(),
        }
    }
}
