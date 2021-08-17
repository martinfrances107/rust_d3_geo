#![allow(clippy::pedantic)]
#![warn(missing_debug_implementations)]
#![allow(clippy::many_single_char_names)]
// #![allow(dead_code)]
// #![allow(unused_imports)]

extern crate derivative;
extern crate rust_d3_array;
extern crate web_sys;

use std::fmt::Debug;

use geo::CoordFloat;
use geo::Coordinate;

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
pub mod stream;

mod compose;
mod point_equal;

/// Common to Projection, Rotation.
pub trait Transform: Clone
where
    <Self as Transform>::T: CoordFloat,
{
    type T;
    fn transform(&self, p: &Coordinate<Self::T>) -> Coordinate<Self::T>;
    fn invert(&self, p: &Coordinate<Self::T>) -> Coordinate<Self::T>;
}
