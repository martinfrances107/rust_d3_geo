// #![allow(clippy::needless_return)]
#![allow(clippy::all)]
// #![allow(unused_variables)]
// #![allow(dead_code)]
// #![allow(unused_imports)]

extern crate derivative;
extern crate rust_d3_array;
extern crate web_sys;

pub mod cartesian;
pub mod centroid;
pub mod circle;
pub mod clip;
pub mod data_object;
pub mod distance;
pub mod identity;
pub mod in_delta;
pub mod length;
pub mod path;
pub mod polygon_contains;
pub mod projection;
pub mod rotation;

mod compose;
mod constant;
mod point_equal;
pub mod stream;

// pub trait TransformClone<'a>: Transform {
//     fn box_clone(&'a self) -> Box<dyn TransformClone<'a, TcC = Self::TcC>>;
// }

// Common to Projection, Rotation.
pub trait Transform {
    type C;
    fn transform(&self, p: &Self::C) -> Self::C;
    fn invert(&self, p: &Self::C) -> Self::C;
}
