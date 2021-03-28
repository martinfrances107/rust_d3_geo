use geo::CoordFloat;
use geo::Coordinate;
use num_traits::float::FloatConst;
use std::ops::AddAssign;

use super::projection::Projection;
use super::projection_mutator::ProjectionMutator;
use super::ProjectionRawEnum;
use crate::Transform;

#[derive(Clone, Debug, Default)]
pub struct EquirectangularRaw<T> {
    lambda: T,
    phi: T,
}

impl<T> EquirectangularRaw<T>
where
    T: AddAssign + CoordFloat + FloatConst + Default,
{
    pub fn gen_projection_mutator() -> ProjectionMutator<T> {
        let e = ProjectionRawEnum::E(EquirectangularRaw::default());
        let projection = ProjectionMutator::from_projection_raw(e, None);
        projection.scale(T::from(152.63f64).unwrap())
    }
}

// impl<'a, T: CoordFloat + FloatConst> TransformClone<'a> for EquirectangularRaw<T> {
//     fn box_clone(&'a self) -> Box<dyn TransformClone<'a, TcC = Self::TcC>> {
//         Box::new(self.clone())
//     }
// }

impl<T: CoordFloat + FloatConst> Transform for EquirectangularRaw<T> {
    type TcC = Coordinate<T>;
    fn transform(&self, p: &Self::TcC) -> Self::TcC {
        *p
    }
    fn invert(&self, p: &Self::TcC) -> Self::TcC {
        *p
    }
}
