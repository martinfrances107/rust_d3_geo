#![deny(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![warn(clippy::perf)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![allow(clippy::many_single_char_names)]

//! # rust d3 geo voronoi
//!
//! See the README.md.
extern crate js_sys;
extern crate rust_topojson_client;
extern crate topojson;
extern crate web_sys;

use geo_types::Coord;
use gloo_utils::format::JsValueSerdeExt;
use topojson::Topology;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::Document;
use web_sys::Path2d;
use web_sys::Request;
use web_sys::RequestInit;
use web_sys::RequestMode;
use web_sys::Response;

use d3_geo_rs::graticule::generate_mls;
use d3_geo_rs::path::builder::Builder as PathBuilder;
use d3_geo_rs::path::path2d_endpoint::Path2dEndpoint;
use d3_geo_rs::projection::orthographic::Orthographic;
use d3_geo_rs::projection::Build;
use d3_geo_rs::projection::RawBase as ProjectionRawBase;
use d3_geo_rs::projection::RotateSet;
use d3_geo_rs::projection::ScaleSet;
use d3_geo_rs::projection::TranslateSet;
use rust_topojson_client::feature::feature_from_name;

fn document() -> Result<Document, JsValue> {
    let window = web_sys::window().ok_or("no window")?;
    Ok(window.document().ok_or("no document")?)
}

/// Entry point
///
///
/// # Panics
/// # Errors
///
/// When the window could not be obtained.
/// When geoJson file cannot be obtained/fetched
/// When the Topology deserialization failed.
/// When the canvas element could not be obtained.
#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    let document = document();
    let window = web_sys::window().expect("Failed to get window");

    // Get data from world map.
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);
    let request =
        Request::new_with_str_and_init("/world-atlas/world/50m.json", &opts)?;

    let resp_value =
        JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    let json = JsFuture::from(resp.json()?).await?;

    let topology = JsValueSerdeExt::into_serde::<Topology>(&json)
        .expect("Did not get a valid Topology");

    // Grab canvas.
    let canvas = document?
        .get_element_by_id("c")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context_raw = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    let path2d = Path2d::new().expect("could not construct path2d");

    let width: f64 = canvas.width().into();
    let height: f64 = canvas.height().into();

    let countries = feature_from_name(&topology, "countries")
        .expect("Did not extract geometry");

    let ep = Path2dEndpoint::new(path2d);
    let path_builder = PathBuilder::new(ep);

    let ortho = Orthographic::builder()
        .scale_set(width / 1.3_f64 / std::f64::consts::PI)
        .translate_set(&Coord {
            x: width / 2_f64,
            y: height / 2_f64,
        })
        .rotate2_set(&[270_f64, 0_f64])
        .build();

    let mut path = path_builder.build(ortho);
    context_raw.set_stroke_style(&"#333".into());
    context_raw.set_line_width(0.5);
    let path2d = path.object(&countries);
    context_raw.stroke_with_path(&path2d);

    // Graticule
    let graticule = generate_mls();
    context_raw.set_stroke_style(&"#ccc".into());
    let path2d = path.object(&graticule);
    context_raw.stroke_with_path(&path2d);

    Ok(())
}
