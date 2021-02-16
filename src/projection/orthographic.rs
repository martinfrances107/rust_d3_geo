use geo::{CoordFloat, Coordinate};
use num_traits::float::FloatConst;

use super::projection::Projection;
use super::projection::StreamOrValueMaybe;
use super::projection_mutator::ProjectionMutator;
use crate::projection::azimuthal::azimuthal_invert;
use crate::Transform;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct OrthographicRaw {}

impl OrthographicRaw {
    pub fn gen_projection_mutator<'a, T: CoordFloat + FloatConst + 'static>(
    ) -> ProjectionMutator<'a, T> {
        let s: Rc<Box<dyn Transform<T>>> = Rc::new(Box::new(OrthographicRaw {}));
        let mut projection = ProjectionMutator::from_projection_raw(s, None);
        projection.scale(T::from(249.5f64).unwrap());
        let angle = T::from(249.5f64).unwrap();
        projection.clip_angle(StreamOrValueMaybe::Value(angle));
        return projection;
    }
}

impl<T: CoordFloat + 'static> Transform<T> for OrthographicRaw {
    #[inline]
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        Coordinate {
            x: p.y.cos() * p.x.sin(),
            y: p.y.sin(),
        }
    }

    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let f = Box::new(|z: T| z.asin());
        let g = azimuthal_invert(f);
        return g(p.x, p.y);
    }
}
