#![allow(clippy::pedantic)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
//! # rust d3 geo voronoi
//!
//! See the README.md.
extern crate js_sys;
extern crate rand;
extern crate rust_topojson_client;
extern crate topojson;
extern crate web_sys;

use rust_topojson_client::feature::Builder as FeatureBuilder;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::Document;
use web_sys::*;

use topojson::Topology;

mod dom_macros;
mod azimuthal_equal_area;
mod azimuthal_equidistant;
mod equirectangular;
mod orthographic;
mod mercator;
mod gnomic;
mod stereographic;

use azimuthal_equidistant::draw_azimuthal_equidistant;
use azimuthal_equal_area::draw_azimuthal_equal_area;
use orthographic::draw_orthographic;
use mercator::draw_mercator;
use gnomic::draw_gnomic;
use stereographic::draw_sterographic;

#[wasm_bindgen]
#[cfg(not(tarpaulin_include))]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);

    fn alert(s: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[cfg(not(tarpaulin_include))]
fn get_document() -> Result<Document, JsValue> {
    let window = web_sys::window().unwrap();
    Ok(window.document().unwrap())
}

/// Entry point
#[wasm_bindgen(start)]
#[cfg(not(tarpaulin_include))]
pub async fn start() -> Result<(), JsValue> {
    use crate::equirectangular::draw_equirectangular;

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

    let topology: Topology = json.into_serde().expect("Could not parse as Topology");

    let land = FeatureBuilder::generate_from_name(&topology, "countries")
        .expect("Did not extract geometry");

        draw_azimuthal_equal_area(&land)?;
        draw_azimuthal_equidistant(&land)?;
        draw_orthographic(&land)?;
        draw_mercator(&land)?;
        draw_sterographic(&land)?;
        draw_equirectangular(&land)?;
        draw_gnomic(&land)?;
    Ok(())
}
