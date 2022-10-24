#![allow(clippy::pedantic)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
//! # rust d3 geo voronoi
//!
//! See the README.md.
extern crate js_sys;
extern crate rust_topojson_client;
extern crate topojson;
extern crate web_sys;

use futures::prelude::*;
use geo::Coordinate;
use geo::Geometry;
use geo::MultiLineString;
use gloo_timers::future::IntervalStream;
use gloo_utils::format::JsValueSerdeExt;
use topojson::Topology;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::console_log;
use web_sys::Document;
use web_sys::Request;
use web_sys::RequestInit;
use web_sys::RequestMode;
use web_sys::Response;

use rust_d3_geo::graticule::generate as generate_graticule;
use rust_d3_geo::path::builder::Builder as PathBuilder;
use rust_d3_geo::path::context::Context;
use rust_d3_geo::projection::orthographic::Orthographic;
use rust_d3_geo::projection::Build;
use rust_d3_geo::projection::ProjectionRawBase;
use rust_d3_geo::projection::RotateSet;
use rust_d3_geo::projection::ScaleSet;
use rust_d3_geo::projection::TranslateSet;
use rust_topojson_client::feature::feature_from_name;

fn document() -> Result<Document, JsValue> {
    let window = web_sys::window().unwrap();
    Ok(window.document().unwrap())
}

/// Entry point
#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    let document = document()?;
    let window = web_sys::window().expect("Failed to get window");

    // Get data from world map.
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init("/world-atlas/world/50m.json", &opts)?;

    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    let json = JsFuture::from(resp.json()?).await?;

    let topology =
        JsValueSerdeExt::into_serde::<Topology>(&json).expect("Did not get a valid Topology");

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

    let countries = feature_from_name(&topology, "countries").expect("Did not extract geometry");

    let mut yaw = 300_f64;

    let mut ob = Orthographic::builder();
    ob.scale_set(width as f64 / 1.3_f64 / std::f64::consts::PI)
        .translate_set(&Coordinate {
            x: width / 2_f64,
            y: height / 2_f64,
        });

    // Graticule
    let graticule =
        Geometry::MultiLineString(MultiLineString(generate_graticule().lines().collect()));

    spawn_local(async move {
        let mut count = 0;

        IntervalStream::new(500)
            .take_while(|_| {
                count += 1;
                future::ready(count < 100)
            })
            .for_each(|_| {
                let context = Context::new(context_raw.clone());
                let pb = PathBuilder::new(context);
                ob.rotate_set(&[yaw, -45f64, 0f64]);

                let ortho = ob.build();
                context_raw.clear_rect(0f64, 0f64, width, height);

                let mut path = pb.build(ortho);
                context_raw.set_stroke_style(&"#333".into());
                path.object(&countries);
                context_raw.stroke();

                context_raw.begin_path();
                context_raw.set_stroke_style(&"#ccc".into());
                path.object(&graticule);
                context_raw.stroke();
                yaw -= 0.2f64;

                console_log!("yaw :{}", yaw);

                future::ready(())
            })
            .await;
    });

    Ok(())
}
