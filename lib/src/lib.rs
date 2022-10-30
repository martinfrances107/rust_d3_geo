#![allow(clippy::pedantic)]
// #![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![allow(clippy::many_single_char_names)]
#![doc(html_logo_url = "https://github.com/martinfrances107/rust_d3_geo/images/logo.png")]
//! A port of [d3/d3-geo](<https://github.com/d3/d3-geo>) into [rust_d3_geo](<https://github.com/martinfrances107/rust_d3_geo>).
//!
//! A library with a wide range of geographic projections, spherical shapes and spherical trigonometry.
//!
//! Features :-
//!  - Each projection builder supports - scaling, rotating and translation to yield the desired map view.
//!
//!  - Large datasets can be resampled to reduce compute.
//!
//!  - As well as displaying to a CANVAS element or SVG, various metric can be computed on the geometry
//!    such as [Area](path::area::Area), [Centroids](path::centroid::Centroid),
//!     and [Bounds](path::bounds::Bounds) on polygons and lines.
//!
//! TODO add note about stream pipelines and endpoints.
//!
//! # Available projections
//!
//! - [AzimuthalEqualArea](projection::azimuthal_equal_area::AzimuthalEqualArea)
//! - [AzimuthalEquiDistant](projection::azimuthal_equidistant::AzimuthalEquiDistant)
//! - [ConicEqualArea](projection::conic_equal_area::ConicEqualArea)
//! - [Equirectangular](projection::equirectangular::Equirectangular)
//! - [Gnomic](projection::gnomic::Gnomic)
//! - [Orthographic](projection::orthographic::Orthographic)
//! - [Mercator](projection::mercator::Mercator)
//! - [MercatorTransverse](projection::mercator_transverse::MercatorTransverse)
//! - [Stereographic](projection::stereographic::Stereographic)
//!
//! Each projection has default builder, Stereographic for example
//!
//! ```rust
//! use crate::rust_d3_geo::projection::ProjectionRawBase;
//! use rust_d3_geo::projection::stereographic::Stereographic;
//!
//! let stereographic_builder = Stereographic::builder()
//! ```
//!
//! # Examples
//!
//! The examples directory contains a large selection of applications demmonstration web applications
//! rendering to a CANVAS or SVG elemments. It serves as a migration guide
//! examples/projection shows each projction in turn, with the javascript and rust version drawn side by side.
//!
//! examples/globe - demonstrates that this library can process larger datasets than is possible which javascript
//!   The javascript version operate on a 110m dataset of the globe while, the RUST version use a denser 50m dataset.
//!
//! Here is code snippet from example/projection/globe/ showing the rendering of a globe.
//!
//!
//!
//! ```no_run rust
//!
//! extern crate js_sys;
//! extern crate rust_topojson_client;
//! extern crate topojson;
//! extern crate web_sys;
//!
//! use geo::Coordinate;
//! use geo::Geometry;
//! use geo::MultiLineString;
//! use topojson::Topology;
//! use wasm_bindgen::prelude::*;
//! use wasm_bindgen::JsCast;
//! use gloo_utils::format::JsValueSerdeExt;
//! use wasm_bindgen_futures::JsFuture;
//! use web_sys::Document;
//! use web_sys::*;
//!
//! use rust_d3_geo::graticule::generate as generate_graticule;
//! use rust_d3_geo::path::builder::Builder as PathBuilder;
//! use rust_d3_geo::path::context::Context;
//! use rust_d3_geo::projection::orthographic::Orthographic;
//! use rust_d3_geo::projection::Build;
//! use rust_d3_geo::projection::ProjectionRawBase;
//! use rust_d3_geo::projection::RotateSet;
//! use rust_d3_geo::projection::ScaleSet;
//! use rust_d3_geo::projection::TranslateSet;
//! use rust_topojson_client::feature::feature_from_name;
//!
//! fn document() -> Result<Document, JsValue> {
//!     let window = web_sys::window().unwrap();
//!     Ok(window.document().unwrap())
//! }
//!
//! /// Entry point
//! #[wasm_bindgen(start)]
//! pub async fn start() -> Result<(), JsValue> {
//!     let document = document()?;
//!     let window = web_sys::window().expect("Failed to get window");
//!
//!     // Get data from world map.
//!     let mut opts = RequestInit::new();
//!     opts.method("GET");
//!     opts.mode(RequestMode::Cors);
//!     let request = Request::new_with_str_and_init("/world-atlas/world/50m.json", &opts)?;
//!
//!     let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
//!     let resp: Response = resp_value.dyn_into().unwrap();
//!
//!     let json = JsFuture::from(resp.json()?).await?;
//!
//!     let topology =
//!         JsValueSerdeExt::into_serde::<Topology>(&json).expect("Did not get a valid Topology");
//!
//!     // Grab canvas.
//!     let canvas = document
//!         .get_element_by_id("c")
//!         .unwrap()
//!         .dyn_into::<web_sys::HtmlCanvasElement>()?;
//!
//!     let context_raw = canvas
//!         .get_context("2d")?
//!         .unwrap()
//!         .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
//!
//!     let width: f64 = canvas.width().into();
//!     let height: f64 = canvas.height().into();
//!
//!     let countries = feature_from_name(&topology, "countries").expect("Did not extract geometry");
//!
//!     let context = Context::new(context_raw.clone());
//!     let pb = PathBuilder::new(context);
//!
//!     let ortho = Orthographic::builder()
//!         .scale_set(width as f64 / 1.3_f64 / std::f64::consts::PI)
//!         .translate_set(&Coordinate {
//!             x: width / 2_f64,
//!             y: height / 2_f64,
//!         })
//!         .rotate_set(&[270_f64, 00_f64, 0_f64])
//!         .build();
//!
//!     let mut path = pb.build(ortho);
//!     context_raw.set_stroke_style(&"#333".into());
//!     context_raw.set_line_width(0.5);
//!     path.object(&countries);
//!     context_raw.stroke();
//!
//!     // Graticule
//!     let graticule =
//!         Geometry::MultiLineString(MultiLineString(generate_graticule().lines().collect()));
//!     context_raw.begin_path();
//!     context_raw.set_stroke_style(&"#ccc".into());
//!     path.object(&graticule);
//!     context_raw.stroke();
//!
//!     Ok(())
//! }
//!
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
/// Use to calculate the centroid point for a given object.
pub mod centroid;
/// Related to the injection of circles into a stream.
pub mod circle;
/// Projectors can clip, remove point computed to be outside the projection.
pub mod clip;
/// Combines two transforms into one.
mod compose;
/// Streamable Data obejcts.
pub mod data_object;
/// Used to calculate distances on a sphereical surface.
pub mod distance;
/// A graticule is a network of lines used for plotting, scaling.
pub mod graticule;
/// A stream pipeline stage.
pub mod identity;
/// Testing and debug helpers.
#[cfg(not(tarpaulin_include))]
pub mod in_delta;
/// Used to calculate of object lengths.
pub mod length;
/// Mathematical constants.
pub mod math;
/// Stream end point: Calculation of paths to a string or rendering context.
pub mod path;
/// A stripped down version of [path](crate::path)
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
mod path_test_context;

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
