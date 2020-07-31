// #![allow(unused_variables)]
// #![allow(dead_code)]
// #![allow(unused_imports)]

// mod adder;
pub mod cartesian;
/// Circle is used in intgration tests.
pub mod circle;
/// polygon_contains has a integration test.
pub mod polygon_contains;

mod compose;
mod rotation;
mod stream;

/// Common to Projection, Rotation.
pub trait Transform<F> {
    fn transform(&self, p: &[F; 2]) -> [F; 2];
    fn invert(&self, p: &[F; 2]) -> [F; 2];
}
