use std::rc::Rc;

use geo::Coordinate;
use num_traits::float::FloatConst;
use num_traits::Float;

use super::projection::Projection;
use super::projection::StreamProcessorValueMaybe;
use super::projection_mutator::ProjectionMutator;
use crate::projection::azimuthal::azimuthal_invert;
use crate::Transform;

#[derive(Clone, Debug)]
pub struct OrthographicRaw {}

impl OrthographicRaw {
    #[inline]
    fn new<T: Float + FloatConst + 'static>() -> Box<dyn Transform<T>> {
        Box::new(Self {})
    }

    pub fn gen_projection_mutator<'a, T: Float + FloatConst + 'static>() -> ProjectionMutator<T> {
        let s = Rc::new(OrthographicRaw::new());
        let mut projection = ProjectionMutator::from_projection_raw(s);
        projection.scale(Some(&T::from(249.5f64).unwrap()));
        let angle = T::from(249.5f64).unwrap();
        projection.clip_angle(StreamProcessorValueMaybe::Value(angle));
        return projection;
    }
}

impl<T: Float + 'static> Transform<T> for OrthographicRaw {
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
