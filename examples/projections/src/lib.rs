#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![cfg(not(tarpaulin_include))]
//! # rust d3 geo voronoi
//!
//! See the README.md.
extern crate js_sys;
extern crate rust_topojson_client;
extern crate topojson;
extern crate web_sys;

use futures::try_join;
use geo::Geometry;
use gloo_utils::format::JsValueSerdeExt;
use rust_topojson_client::feature::feature_from_name;
use topojson::Topology;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::console_log;
use web_sys::Document;
use web_sys::Request;
use web_sys::RequestInit;
use web_sys::RequestMode;
use web_sys::Response;

mod albers;
mod azimuthal_equal_area;
mod azimuthal_equidistant;
mod conformal;
mod conic_equal_area;
mod equal_earth;
mod equidistant;
mod equirectangular;
mod gnomic;
mod mercator;
mod mercator_transverse;
mod orthographic;
mod stereographic;

use albers::draw as draw_albers;
use azimuthal_equal_area::draw as draw_azimuthal_equal_area;
use azimuthal_equidistant::draw as draw_azimuthal_equidistant;
use conformal::draw as draw_conformal;
use conic_equal_area::draw as draw_conic_equal_area;
use equal_earth::draw as draw_equal_earth;
use equidistant::draw as draw_equidistant;
use equirectangular::draw as draw_equirectangular;
use gnomic::draw as draw_gnomic;
use mercator::draw as draw_mercator;
use mercator_transverse::draw as draw_mercator_transverse;
use orthographic::draw as draw_orthographic;
use stereographic::draw as draw_stereographic;

#[cfg(not(tarpaulin_include))]
fn document() -> Result<Document, JsValue> {
    let window = web_sys::window().ok_or("no window")?;
    Ok(window.document().ok_or("no document")?)
}

/// Entry point
///
/// # Panics
/// # Errors
///
/// When the window could not be obtained.
/// When geoJson file cannot be obtained/fetched
/// When the Topology deserialization failed.
/// When the canvas element could not be obtained.
#[wasm_bindgen(start)]
#[cfg(not(tarpaulin_include))]
pub async fn start() -> Result<(), JsValue> {
    console_log!("run() - wasm entry point");

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request =
        Request::new_with_str_and_init("/world-atlas/world/50m.json", &opts)?;

    request.headers().set("Accept", "application/json")?;

    let window = web_sys::window().expect("Failed to get window");
    let resp_value =
        JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    let topology = JsValueSerdeExt::into_serde::<Topology>(&json)
        .expect("Did not get a valid Topology");

    let land: Geometry<f64> = feature_from_name(&topology, "countries")
        .expect("Did not extract geometry");

    try_join!(
        draw_albers(&land),
        draw_azimuthal_equal_area(&land),
        draw_azimuthal_equidistant(&land),
        draw_conformal(&land),
        draw_conic_equal_area(&land),
        draw_equirectangular(&land),
        draw_equidistant(&land),
        draw_equal_earth(&land),
        draw_gnomic(&land),
        draw_mercator(&land),
        draw_mercator_transverse(&land),
        draw_orthographic(&land),
        draw_stereographic(&land),
    )?;

    Ok(())
}
