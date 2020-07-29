// #![allow(unused_variables)]
// #![allow(dead_code)]
// #![allow(unused_imports)]

// mod adder;
mod cartesian;
mod circle;
mod compose;
mod polygon_contains;
mod stream;

/// Common to Projection, Rotation.
pub trait Transform<F> {
    fn transform(&self, p: &[F; 2]) -> [F; 2];
    fn invert(&self, p: &[F; 2]) -> [F; 2];
}
