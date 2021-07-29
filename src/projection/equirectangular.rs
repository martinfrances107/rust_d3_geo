use std::fmt::Display;
use std::ops::AddAssign;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::float::FloatConst;
use num_traits::AsPrimitive;

use crate::stream::Stream;
use crate::Transform;

use super::projection::Projection;
use super::scale::Scale;

#[derive(Clone, Debug)]
pub struct EquirectangularRaw<T>
where
    T: CoordFloat,
{
    lambda: T,
    phi: T,
}

impl<T> Default for EquirectangularRaw<T>
where
    T: CoordFloat,
{
    fn default() -> Self {
        Self {
            lambda: T::zero(),
            phi: T::zero(),
        }
    }
}

impl<T> EquirectangularRaw<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    #[inline]
    pub fn gen_projection_mutator<'a, DRAIN>() -> Projection<'a, DRAIN, EquirectangularRaw<T>, T>
    where
        DRAIN: 'a + Default + Stream<SC = Coordinate<T>>,
    {
        Projection::new(EquirectangularRaw::default(), None).scale(T::from(152.63f64).unwrap())
    }
}

impl<T> Transform for EquirectangularRaw<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type C = Coordinate<T>;
    fn transform(&self, p: &Coordinate<T>) -> Self::C {
        *p
    }
    fn invert(&self, p: &Coordinate<T>) -> Self::C {
        *p
    }
}
