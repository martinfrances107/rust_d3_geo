// use std::fmt::Display;
// use std::ops::AddAssign;
use std::rc::Rc;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::float::FloatConst;
// use num_traits::AsPrimitive;

// use super::projection::Projection;
// use super::scale::Scale;
// use super::ProjectionRawTrait;
use crate::Transform;

#[derive(Clone, Debug, Default)]
pub struct EquirectangularRaw<T> {
    lambda: T,
    phi: T,
}

// impl<T> EquirectangularRaw<T>
// where
//     T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
// {
//     #[inline]
//     pub fn gen_projection_mutator<'a>() -> Projection<'a, EquirectangularRaw<T>, T> {
//         Projection::new(EquirectangularRaw::default(), None).scale(T::from(152.63f64).unwrap())
//     }
// }

// impl<T> ProjectionRawTrait for Rc<EquirectangularRaw<T>>
// // where
// //     T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
// {
// }

impl<T: CoordFloat + FloatConst> Transform for Rc<EquirectangularRaw<T>> {
    type C = Coordinate<T>;
    fn transform(&self, p: &Coordinate<T>) -> Self::C {
        *p
    }
    fn invert(&self, p: &Coordinate<T>) -> Self::C {
        *p
    }
}
