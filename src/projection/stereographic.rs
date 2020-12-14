use std::rc::Rc;

use geo::Point;
use num_traits::Float;
use num_traits::float::FloatConst;

use super::projection::Projection;
use super::projection::StreamProcessorValueMaybe;
use super::projection_mutator::ProjectionMutator;
use crate::projection::azimuthal::azimuthal_invert;
use crate::Transform;

#[derive(Clone, Debug)]
pub struct StereographicRaw {}

impl StereographicRaw {
    fn new<T: Float + 'static>() -> Box<dyn Transform<T>> {
        return Box::new(Self {});
    }

    pub fn gen_projection_mutator<'a, T: Float + FloatConst + 'static>() -> ProjectionMutator<T> {
        let s = Rc::new(StereographicRaw::new());
        let mut projection = ProjectionMutator::from_projection_raw(s);
        projection.scale(Some(&T::from(250f64).unwrap()));
        projection.clip_angle(StreamProcessorValueMaybe::Value(T::from(142f64).unwrap()));
        return projection;
    }
}

impl<T: Float + 'static> Transform<T> for StereographicRaw {
    fn transform(&self, p: &Point<T>) -> Point<T> {
        let cy = p.y().cos();
        let k = T::one() + p.x().cos() * cy;
        return Point::new(cy * p.x().sin() / k, p.y().sin() / k);
    }

    fn invert(&self, p: &Point<T>) -> Point<T> {
        let f = Box::new(|z: T| T::from(2).unwrap() * z.atan());
        let g = azimuthal_invert(f);
        return g(p.x(), p.y());
    }
}
