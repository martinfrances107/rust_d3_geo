use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::Transform;

///Projection definition.
#[derive(Clone, Debug)]
pub struct ConicEqualArea<T> {
    c: T,
    n: T,
    r0: T,
    two: T,
}

impl<T> ConicEqualArea<T> {
    #[inline]
    pub(super) const fn new(c: T, n: T, r0: T, two: T) -> Self {
        Self { c, n, r0, two }
    }
}

impl<T> Transform for ConicEqualArea<T>
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
