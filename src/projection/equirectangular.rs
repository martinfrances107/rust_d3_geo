use geo::CoordFloat;
use geo::Coordinate;
use num_traits::float::FloatConst;

use super::projection::Projection;
use super::projection_mutator::ProjectionMutator;
use crate::Transform;
use crate::TransformClone;
use std::rc::Rc;

#[derive(Clone, Debug, Default)]
pub struct EquirectangularRaw<T> {
    lambda: T,
    phi: T,
}

impl<T> EquirectangularRaw<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    #[inline]
    fn new() -> Box<dyn Transform<C = Coordinate<T>, TcC = Coordinate<T>>> {
        Box::new(Self {
            lambda: T::zero(),
            phi: T::zero(),
        })
    }

    pub fn gen_projection_mutator<'a>() -> ProjectionMutator<T> {
        let s = Rc::new(EquirectangularRaw::new());
        let mut projection = ProjectionMutator::from_projection_raw(s, None);
        projection.scale(T::from(152.63f64).unwrap());
        return projection;
    }
}

impl<T: CoordFloat + FloatConst + 'static> TransformClone for EquirectangularRaw<T> {
    type TcC = Coordinate<T>;
    fn clone_box(&self) -> Box<dyn Transform<C = Coordinate<T>, TcC = Self::TcC>> {
        Box::new(self.clone())
    }
}

impl<T: CoordFloat + FloatConst + 'static> Transform for EquirectangularRaw<T> {
    type C = Coordinate<T>;
}
