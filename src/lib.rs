// #![allow(unused_variables)]
// #![allow(dead_code)]
// #![allow(unused_imports)]
use num_traits::Float;

pub mod cartesian;
pub mod circle;
pub mod projection;
pub mod polygon_contains;
pub mod rotation;

mod clip;
mod compose;
mod math;
mod in_delta;
mod point_equal;
mod resample;
mod stream;
mod transform_stream;

#[derive(Copy, Clone, Debug)]
struct TransformIdentity {}
impl TransformIdentity {
    fn new() -> Self
    {
        return TransformIdentity {};
    }
}

impl<F> Transform<F> for TransformIdentity where F: Float {}


/// Common to Projection, Rotation.
pub trait Transform<F>
where
    F: Float,
{
    fn transform(&self, p: &[F; 2]) -> [F; 2] {
        return [p[0], p[1]];
    }
    fn invert(&self, p: &[F; 2]) -> [F; 2] {
        return *p;
    }
}
