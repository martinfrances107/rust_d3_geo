#![allow(clippy::pedantic)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![allow(clippy::many_single_char_names)]
//! # Rust port of d3-geo
//!
//! (see README.md)

/// Allows the ommission of complex fields from debug output.
extern crate derivative;
extern crate rust_d3_array;
extern crate web_sys;

use std::fmt::Debug;

use geo::CoordFloat;
use geo::Coordinate;

/// Vector arithmatic operations on 3-D vectors.
pub mod cartesian;
/// Stream end point: calculation of centroid point.
pub mod centroid;
/// Related to the injection of circles into a stream.
pub mod circle;
/// Projectors can clip, remove point computed to be outside the projection.
pub mod clip;
/// Combines transforms into one.
mod compose;
/// Streamable Data obejcts. ( D3 objects )
pub mod data_object;
/// Stream end point: calculation of distances on a surface.
pub mod distance;
/// Testing and Debug helpers.
pub mod in_delta;
/// Stream end point: calculation of object lengths.
pub mod length;
/// Stream end point: calculation of paths to a string or rendering context.
pub mod path;
/// Determins when points are located inside data objects.
pub mod polygon_contains;
/// Holds proctions and associated builders.
pub mod projection;
/// Rotation transforms
pub mod rotation;
/// Stream related helper functions.
pub mod stream;

mod identity;
mod point_equal;
/// 2-D Transform common to projections and, rotations.
///
/// FloatConst is required by forward_rotation_lambda()
pub trait Transform: Clone + Debug
where
    <Self as Transform>::T: CoordFloat,
{
    /// f64 or f43
    type T;

    /// Transform a 2-D point to another.
    fn transform(&self, p: &Coordinate<Self::T>) -> Coordinate<Self::T>;
    /// Reversed the transform.
    fn invert(&self, p: &Coordinate<Self::T>) -> Coordinate<Self::T>;
}
