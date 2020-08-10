// #![allow(unused_variables)]
// #![allow(dead_code)]
// #![allow(unused_imports)]
use num_traits::Float;

// mod adder;
pub mod cartesian;
/// Circle is used in intgration tests.
pub mod circle;
/// polygon_contains has a integration test.
pub mod polygon_contains;
pub mod rotation;

mod clip;
mod compose;
mod math;
mod point_equal;
mod projection;
mod resample;
mod stream;
mod transform_stream;

/// Common to Projection, Rotation.
/// Defaults to the identity transform.
///
#[derive(Clone, Debug)]
struct TransformIdentity {}
impl TransformIdentity {
    fn new() -> Self
    {
        return TransformIdentity {};
    }
}

impl<F> Transform<F> for TransformIdentity where F: Float {}

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
