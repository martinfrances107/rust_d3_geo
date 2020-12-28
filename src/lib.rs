// #![allow(clippy::needless_return)]
#![allow(clippy::all)]
// #![allow(unused_variables)]
// #![allow(dead_code)]
// #![allow(unused_imports)]

use geo::Coordinate;
use num_traits::Float;

pub mod cartesian;
pub mod centroid;
pub mod circle;
pub mod data_object;
pub mod distance;
pub mod in_delta;
pub mod length;
pub mod math;
pub mod polygon_contains;
pub mod projection;
pub mod rotation;

mod clip;
mod compose;
mod point_equal;
mod resample;
mod stream;
mod transform_stream;

#[derive(Copy, Clone, Debug)]
pub struct TransformIdentity {}
impl TransformIdentity {
    fn new() -> Self {
        return TransformIdentity {};
    }
}

impl<T: Float> Transform<T> for TransformIdentity {}

/// Common to Projection, Rotation.
pub trait Transform<T>
where
    T: Float,
{
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        return p.clone();
    }
    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        return p.clone();
    }
}
