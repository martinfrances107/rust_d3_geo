#![allow(clippy::pedantic)]
#![warn(missing_debug_implementations)]
#![allow(clippy::many_single_char_names)]
// #![allow(dead_code)]
// #![allow(unused_imports)]

extern crate derivative;
extern crate rust_d3_array;
extern crate web_sys;

use std::fmt::Debug;
use std::fmt::Display;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

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
mod point_equal;
pub mod stream;

// Common to Projection, Rotation.
pub trait Transform: Clone
where
    <Self as Transform>::T:
        AsPrimitive<<Self as Transform>::T> + Debug + Display + CoordFloat + FloatConst,
{
    type T;
    fn transform(&self, p: &Coordinate<Self::T>) -> Coordinate<Self::T>;
    fn invert(&self, p: &Coordinate<Self::T>) -> Coordinate<Self::T>;
}
