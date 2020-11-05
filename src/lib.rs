// #![allow(clippy::needless_return)]
#![allow(clippy::all)]
// #![allow(unused_variables)]
// #![allow(dead_code)]
// #![allow(unused_imports)]

use delaunator::Point;

pub mod cartesian;
pub mod centroid;
pub mod circle;
pub mod data_object;
pub mod distance;
pub mod in_delta;
pub mod length;
pub mod polygon_contains;
pub mod projection;
pub mod rotation;

mod clip;
mod compose;
mod math;
mod point_equal;
mod resample;
mod stream;
mod transform_stream;

#[derive(Copy, Clone, Debug)]
struct TransformIdentity {}
impl TransformIdentity {
    fn new() -> Self {
        return TransformIdentity {};
    }
}

impl Transform for TransformIdentity {}

/// Common to Projection, Rotation.
pub trait Transform {
    fn transform(&self, p: &Point) -> Point {
        return p.clone();
    }
    fn invert(&self, p: &Point) -> Point {
        return p.clone();
    }
}
