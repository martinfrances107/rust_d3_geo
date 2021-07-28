use std::fmt::Display;
use std::ops::AddAssign;
// use std::rc::Rc;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::float::FloatConst;
use num_traits::AsPrimitive;
// use num_traits::AsPrimitive;

// use super::projection::Projection;
use super::scale::Scale;
// use super::ProjectionRawTrait;
use super::projection::Projection;
// use crate::clip::Clip;
// use crate::projection::resample::ResampleTrait;
// use crate::stream::Stream;
use crate::Transform;
#[derive(Clone, Debug)]
pub struct EquirectangularRaw<T>
where
    // T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
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
    pub fn gen_projection_mutator<'a>() -> Projection<'a, EquirectangularRaw<T>, T>
// where
    //     SD: 'a + Stream<SC = Coordinate<T>> + Default,
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
