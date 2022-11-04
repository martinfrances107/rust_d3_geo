#![allow(clippy::pedantic)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![cfg(not(tarpaulin_include))]

//! # rust d3 geo voronoi
//!
//! See the README.md.
extern crate js_sys;
extern crate rand;
extern crate rust_topojson_client;
extern crate topojson;
extern crate web_sys;

use futures::try_join;
use geo::Geometry;
use gloo_utils::format::JsValueSerdeExt;
use rust_topojson_client::feature::feature_from_name;
use topojson::Topology;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::console_log;

use web_sys::Document;
use web_sys::*;

mod azimuthal_equal_area;
mod azimuthal_equidistant;
mod conic_equal_area;
mod equirectangular;
mod gnomic;
mod mercator;
mod mercator_transverse;
mod orthographic;
mod stereographic;

use azimuthal_equal_area::draw_azimuthal_equal_area;
use azimuthal_equidistant::draw_azimuthal_equidistant;
use conic_equal_area::draw_conic_equal_area;
use equirectangular::draw_equirectangular;
use gnomic::draw_gnomic;
use mercator::draw_mercator;
use mercator_transverse::draw_mercator_transverse;
use orthographic::draw_orthographic;
use stereographic::draw_sterographic;

#[cfg(not(tarpaulin_include))]
fn document() -> Result<Document, JsValue> {
    let window = web_sys::window().unwrap();
    Ok(window.document().unwrap())
}

/// Entry point
#[wasm_bindgen(start)]
#[cfg(not(tarpaulin_include))]
pub async fn start() -> Result<(), JsValue> {
    console_log!("run() - wasm entry point");

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init("/world-atlas/world/50m.json", &opts)?;

    request.headers().set("Accept", "application/json")?;

    let window = web_sys::window().expect("Failed to get window");
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    let topology =
        JsValueSerdeExt::into_serde::<Topology>(&json).expect("Did not get a valid Topology");

    let land: Geometry<f64> =
        feature_from_name(&topology, "countries").expect("Did not extract geometry");

    try_join!(
        draw_azimuthal_equal_area(&land),
        draw_azimuthal_equidistant(&land),
        draw_conic_equal_area(&land),
        draw_orthographic(&land),
        draw_mercator(&land),
        draw_mercator_transverse(&land),
        draw_sterographic(&land),
        draw_equirectangular(&land),
        draw_gnomic(&land),
    )?;

    Ok(())
}
