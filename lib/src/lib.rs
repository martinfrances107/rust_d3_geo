#![allow(clippy::pedantic)]
// #![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![allow(clippy::many_single_char_names)]
//! A port of [d3/d3-geo](<https://github.com/d3/d3-geo>).
//!
//! This rust repository [rust_d3_geo](<https://github.com/martinfrances107/rust_d3_geo>)
//!
//! Geographic projections, spherical shapes and spherical trigonometry.
//!
//! Features :-
//!  - complex transformations - scaling, rotating and translation give fine
//!    grained control the desired map view.
//!
//!  - Large datasets can be resampled to reduce compute.
//!
//!  - Computation of paramters such as [Area](path::area::Area), [Centroids](path::centroid::Centroid),
//!     and [Bounds](path::bounds::Bounds) on polygons and lines.
//!
//! TODO add note about stream pipelines and endpoints.
//!
//! # Available projections
//!
//! - [AzimuthalEqualArea](projection::azimuthal_equal_area::AzimuthalEqualArea)
//! - [AzimuthalEquiDistant](projection::azimuthal_equidistant::AzimuthalEquiDistant)
//! - [ConicEqualAreaRaw](projection::conic_equal_area::ConicEqualAreaRaw)
//! - [Equirectangular](projection::equirectangular::Equirectangular)
//! - [Gnomic](projection::gnomic::Gnomic)
//! - [Orthographic](projection::orthographic::Orthographic)
//! - [Mercator](projection::mercator::Mercator)
//! - [MercatorTransverse](projection::mercator_transverse::MercatorTransverse)
//! - [Stereographic](projection::stereographic::Stereographic)
//!
//! Each projection has default builder
//!
//! Stereographic for example
//!
//! ```rust
//! let stereographic_builder = Stereographic::builder()
//! ```
//!
//! # Examples
//!
//! The examples directory contains a large selection of applications demmonstration web applications
//! rendering to a CANVAS or SVG elemments.
//!
//! A a migration guide
//! examples/projection shows each projction in turn, with the javascript and rust version drawn side by side.
//!
//! examples/globe - demonstrates that this library can process larger datasets than is possible which javascript
//!   The javascript version operate on a 110m dataset of the globe while, the RUST version use a denser 50m dataset.
//!
//! Here is code snippet from example/projection/globe/ showing the rendering of a globe.
//!
//! ```rust
//!   let context = Context::new(context_raw.clone());
//!   let pb = PathBuilder::new(context);
//!
//!   // The default orthographic builder is adjusted with calls like scale_set()
//!   // calling .build() constructs the projector from  builder.
//!   let ortho = Orthographic::builder()
//!        .scale_set(width as f64 / 1.3_f64 / std::f64::consts::PI)
//!        .translate_set(&Coordinate {
//!            x: width / 2_f64,
//!            y: height / 2_f64,
//!        })
//!        .rotate_set(&[270_f64, 00_f64, 0_f64])
//!        .build();
//!
//!   // Output countries on the screen.
//!   // 'countries' is a large dataset of the perimeters of all the countries on the globe.
//!   let mut path = pb.build(ortho);
//!   context_raw.set_stroke_style(&"#333".into());
//!   context_raw.set_line_width(0.5);
//!   path.object(&countries);
//!   context_raw.stroke();
//!
//!   // Output the default Graticule, a set of longitude and latitude lines.
//!   let graticule =
//!       Geometry::MultiLineString(MultiLineString(generate_graticule().lines().collect()));
//!   context_raw.begin_path();
//!   context_raw.set_stroke_style(&"#ccc".into());
//!   path.object(&graticule);
//!   context_raw.stroke();
//! ```
//!
//! <hr>

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
/// A stream pipeline stage
pub mod identity;
/// Testing and Debug helpers.
#[cfg(not(tarpaulin_include))]
pub mod in_delta;
/// Stream end point: Calculation of object lengths.
pub mod length;
/// Mathematical constants.
pub mod math;
/// Stream end point: Calculation of paths to a string or rendering context.
pub mod path;
pub mod path_identity;
/// Determines when points are located inside data objects.
pub mod polygon_contains;
/// Holds proctions and associated builders.
pub mod projection;
/// Rotation transforms.
pub mod rot;
/// Stream related helper functions.
pub mod stream;

#[cfg(not(tarpaulin_include))]
pub mod path_test_context;

/// 2-D Transform common to projections and, rotations.
///
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
