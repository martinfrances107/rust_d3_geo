use geo::CoordFloat;
use num_traits::float::FloatConst;

use super::projection::Projection;
use super::projection_mutator::ProjectionMutator;
use crate::Transform;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct EquirectangularRaw<T> {
    lambda: T,
    phi: T,
}

impl<T> EquirectangularRaw<T>
where
    T: CoordFloat + FloatConst + 'static,
{
    #[inline]
    fn new() -> Box<dyn Transform<T>> {
        Box::new(Self {
            lambda: T::zero(),
            phi: T::zero(),
        })
    }

    pub fn gen_projection_mutator<'a>() -> ProjectionMutator<'a, T> {
        let s = Rc::new(EquirectangularRaw::new());
        let mut projection = ProjectionMutator::from_projection_raw(s, None);
        projection.scale(T::from(152.63f64).unwrap());
        return projection;
    }
}

impl<T: CoordFloat + FloatConst + 'static> Transform<T> for EquirectangularRaw<T> {}
