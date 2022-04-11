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

use geo::Coordinate;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::Document;
use web_sys::*;

use rust_d3_geo::path::builder::Builder as PathBuilder;
use rust_d3_geo::path::context::Context;
use rust_d3_geo::projection::orthographic::Orthographic;
use rust_d3_geo::projection::ProjectionRawBase;

use rust_d3_geo::projection::RotateSet;
use rust_d3_geo::projection::ScaleAdjust;

use rust_d3_geo::projection::TranslateSet;
use rust_topojson_client::feature::Builder as FeatureBuilder;

use topojson::Topology;

mod dom_macros;

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

// macro_rules! console_log {
//     // Note that this is using the `log` function imported above during
//     // `bare_bones`
//     ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
// }

fn get_document() -> Result<Document, JsValue> {
    let window = web_sys::window().unwrap();
    Ok(window.document().unwrap())
}

/// Entry point
#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    let document = get_document()?;
    let window = web_sys::window().expect("Failed to get window");

    // Get data from world map.
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init("/world-atlas/world/50m.json", &opts)?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json()?).await?;

    let topology: Topology = json.into_serde().expect("Could not parse as Topology");

    // Grab canvas.
    let canvas = document
        .get_element_by_id("c")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context_raw = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let width: f64 = canvas.width().into();
    let height: f64 = canvas.height().into();

    let countries = FeatureBuilder::generate_from_name(&topology, "countries")
        .expect("Did not extract geometry");

    let context = Context::new(context_raw.clone());
    let pb = PathBuilder::new(context);

    let ortho_builder = Orthographic::<Context<f64>, f64>::builder();

    let ortho = ortho_builder
        .scale(width as f64 / 1.3_f64 / std::f64::consts::PI)
        .translate(&Coordinate {
            x: width / 2_f64,
            y: height / 2_f64,
        })
        .rotate(&[270_f64, 00_f64, 0_f64])
        .build();

    let mut path = pb.build(ortho);
    context_raw.set_stroke_style(&"#69b3a2".into());
    path.object(&countries);
    context_raw.stroke();

    Ok(())
}
