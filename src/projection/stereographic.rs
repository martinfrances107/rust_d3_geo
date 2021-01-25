use super::projection::Projection;
use super::projection::StreamProcessorValueMaybe;
use super::projection_mutator::ProjectionMutator;
use crate::projection::azimuthal::azimuthal_invert;
use crate::Transform;
use geo::{CoordFloat, Coordinate};
use num_traits::float::FloatConst;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct StereographicRaw {}

impl StereographicRaw {
    #[inline]
    fn new<T: CoordFloat + 'static>() -> Box<dyn Transform<T>> {
        Box::new(Self {})
    }

    pub fn gen_projection_mutator<'a, T: CoordFloat + FloatConst + 'static>() -> ProjectionMutator<T>
    {
        let s = Rc::new(StereographicRaw::new());
        let mut projection = ProjectionMutator::from_projection_raw(s);
        projection.scale(Some(&T::from(250f64).unwrap()));
        projection.clip_angle(StreamProcessorValueMaybe::Value(T::from(142f64).unwrap()));
        return projection;
    }
}

impl<T: CoordFloat + 'static> Transform<T> for StereographicRaw {
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let cy = p.y.cos();
        let k = T::one() + p.x.cos() * cy;
        Coordinate {
            x: cy * p.x.sin() / k,
            y: p.y.sin() / k,
        }
    }

    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let f = Box::new(|z: T| T::from(2).unwrap() * z.atan());
        let g = azimuthal_invert(f);
        return g(p.x, p.y);
    }
}
