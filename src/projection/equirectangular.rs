use std::fmt::Display;
use std::ops::AddAssign;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::float::FloatConst;
use num_traits::AsPrimitive;

use crate::Transform;

use super::projection::Projection;
use super::projection_mutator::ProjectionMutator;
use super::ProjectionRawEnum;

#[derive(Clone, Debug, Default)]
pub struct EquirectangularRaw<T> {
    lambda: T,
    phi: T,
}

impl<T> EquirectangularRaw<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    pub fn gen_projection_mutator() -> ProjectionMutator<T> {
        let e = ProjectionRawEnum::E(EquirectangularRaw::default());
        let projection = ProjectionMutator::from_projection_raw(e, None);
        projection.scale(T::from(152.63f64).unwrap())
    }
}

impl<T: CoordFloat + FloatConst> Transform for EquirectangularRaw<T> {
    type TcC = Coordinate<T>;
    fn transform(&self, p: &Self::TcC) -> Self::TcC {
        *p
    }
    fn invert(&self, p: &Self::TcC) -> Self::TcC {
        *p
    }
}
