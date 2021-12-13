#![allow(clippy::pedantic)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
//! # rust d3 geo voronoi
//!
//! Know bugs.
//!
//! When I convert this benchmark to run on f32's
//! The polygons are mis-shaped
//!
//! See the README.md.
extern crate js_sys;
extern crate rand;
extern crate rust_topojson_client;
extern crate topojson;
extern crate web_sys;

use std::rc::Rc;

use geo::Coordinate;
use rust_topojson_client::feature::Builder as FeatureBuilder;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::Document;
use web_sys::*;

use rust_d3_geo::clip::circle::line::Line;
use rust_d3_geo::clip::circle::pv::PV;
use rust_d3_geo::data_object::DataObject;
use rust_d3_geo::path::builder::Builder as PathBuilder;
use rust_d3_geo::path::context::Context;
use rust_d3_geo::path::context_stream::ContextStream;
use rust_d3_geo::projection::orthographic::Orthographic;
use rust_d3_geo::projection::Raw;
use rust_d3_geo::projection::Scale;
use rust_d3_geo::projection::Translate;

use rust_d3_geo::projection::Rotate;

use topojson::Topology;

mod dom_macros;

#[wasm_bindgen]
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

// Next let's define a macro that's like `println!`, only it works for
// `console.log`. Note that `println!` doesn't actually work on the wasm target
// because the standard library currently just eats all output. To get
// `println!`-like behavior in your app you'll likely want a macro like this.

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

fn get_document() -> Result<Document, JsValue> {
    let window = web_sys::window().unwrap();
    Ok(window.document().unwrap())
}

/// Entry point
#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    console_log!("run() - wasm entry point");
    let document = get_document()?;

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    // let request = Request::new_with_str_and_init(&"/world-atlas/world/50m.json", &opts)?;
    let request = Request::new_with_str_and_init("/world-atlas/africa.json", &opts)?;

    request.headers().set("Accept", "application/json")?;

    let window = web_sys::window().expect("Failed to get window");
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    let topology: Topology = json.into_serde().expect("could not parse as Topology");

    // Grab canvas.
    let canvas = document
        .get_element_by_id("c")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context_raw = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let context = Rc::new(context_raw);

    let width: f64 = canvas.width().into();
    let height: f64 = canvas.height().into();
    // context.set_fill_style(&"black".into());
    // context.set_stroke_style(&"black".into());
    // context.fill_rect(0.0, 0.0, width, height);

    // let land = FeatureBuilder::<f64>::generate_from_name(&topology, &"land")
    let land = FeatureBuilder::<f64>::generate_from_name(&topology, "countries")
        .expect("Did not extract geometry");

    // match FeatureBuilder::<f64>::generate_from_name(&topology, &"land") {
    //     Some(Geometry::GeometryCollection(GeometryCollection(v_geometry))) => {
    //         assert_eq!(v_geometry.len(), 1);
    //         match &v_geometry[0] {
    //             Geometry::MultiPolygon(mp) => {
    //                 assert_eq!(mp.0.len(), 1428_usize);
    //                 let v_polygon = mp.0;
    //             }
    //             _ => {
    //                 assert!(false, "Failed to decode Multipoloygon")
    //             }
    //         }
    //     }
    //     _ => {
    //         assert!(false, "failed to extract a vector of geometries");
    //     }
    // };

    let cs: ContextStream<f64> = ContextStream::Context(Context::new(context.clone()));
    let pb: PathBuilder<Line<f64>, Orthographic<ContextStream<f64>, f64>, PV<f64>, f64> =
        PathBuilder::new(cs);

    let ortho_builder = Orthographic::<ContextStream<f64>, f64>::builder();

    // var projection = d3.geoOrthographic()
    // .scale(width / 1.3 / Math.PI)
    // .translate([width / 2, height / 2])

    // ortho_builder.scale();
    let ortho = ortho_builder
        .scale(width as f64 / 1.3_f64 / std::f64::consts::PI)
        .translate(&Coordinate {
            x: width / 2_f64,
            y: height / 2_f64,
        })
        .rotate(&[0_f64, 0_f64, 0_f64])
        .build();

    // let pb_cps: PathBuilder<Orthographic<ContextStream<f64>, f64>, PV<f64>, f64> =
    //     PathBuilder::context_pathstring();
    // match pb_cps.build(ortho_projection).object(&land_object) {
    //     Some(r) => match r {
    //         ResultEnum::String(s) => {
    //             log_1(&JsValue::from_str(&s));
    //         }
    //         _ => todo!("must handle "),
    //     },
    //     None => {
    //         panic!("Expecting an area.");
    //     }
    // }

    let mut path = pb.build(ortho);
    context.begin_path();
    context.set_fill_style(&"#999".into());
    context.set_stroke_style(&"#69b3a2".into());
    path.object(&DataObject::Geometry(land));

    context.fill();
    context.stroke();

    Ok(())
}
