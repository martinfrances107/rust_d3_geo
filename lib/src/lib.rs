#![allow(clippy::pedantic)]
// #![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![allow(clippy::many_single_char_names)]
//! A port of [d3/d3-geo](<https://github.com/d3/d3-geo>).
//!
//! Geographic projections, spherical shapes and spherical trigonometry.
//!
//! <hr>
//!
//! Repository [rust_d3_geo](<https://github.com/martinfrances107/rust_d3_geo>)

/// Allows the ommission of complex fields from debug output.
extern crate derivative;
extern crate rust_d3_array;
extern crate web_sys;

use geo::CoordFloat;
use geo::Coordinate;

use math::EPSILON;

/// GeoArea Stream.
pub mod area;
/// Vector arithmatic operations on 3-D vectors.
pub mod cartesian;
/// Stream end point: Calculates centroid point for a given object.
pub mod centroid;
/// Related to the injection of circles into a stream.
pub mod circle;
/// Projectors can clip, remove point computed to be outside the projection.
pub mod clip;
/// Combines two transforms into one.
mod compose;
/// Streamable Data obejcts.
pub mod data_object;
/// Stream end point: calculation of distances on a surface.
pub mod distance;
/// A graticule is a network of lines used for plotting, scaling.
pub mod graticule;
/// Testing and Debug helpers.
pub mod in_delta;
/// Stream end point: Calculation of object lengths.
pub mod length;
/// Stream end point: Calculation of paths to a string or rendering context.
pub mod path;
/// Determins when points are located inside data objects.
pub mod polygon_contains;
/// Holds proctions and associated builders.
pub mod projection;
/// Rotation transforms.
pub mod rot;
/// Stream related helper functions.
pub mod stream;

/// Mathematical constants.
pub mod math;

pub mod identity;

/// 2-D Transform common to projections and, rotations.
///
/// FloatConst is required by forward_rotation_lambda().
pub trait Transform
where
    <Self as Transform>::T: CoordFloat,
{
    /// f64 or f43.
    type T;

    /// Transform a 2-D point to another.
    fn transform(&self, p: &Coordinate<Self::T>) -> Coordinate<Self::T>;
    /// Reversed the transform.
    fn invert(&self, p: &Coordinate<Self::T>) -> Coordinate<Self::T>;
}

// The implementation used to be available on Coordinate<T>
// but geo has since been refactored into workspaces. where the packages geo and geo-types are separate.
// abs_diff_eq() is private to the geo-types package. And Coordinate is accessed through the geo package
// Sp I have had to transplant this here.
pub(crate) fn abs_diff_eq<T: CoordFloat>(a: &Coordinate<T>, b: &Coordinate<T>) -> bool {
    let e = T::from(EPSILON).unwrap();
    (a.x - b.x).abs() < e && (a.y - b.y).abs() < e
}
