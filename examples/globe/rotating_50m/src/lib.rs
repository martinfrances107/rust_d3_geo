#![allow(clippy::pedantic)]
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

mod utils;

use geo::Coordinate;
use geo::Geometry;
use geo::MultiLineString;
use gloo_utils::format::JsValueSerdeExt;
use topojson::Topology;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::CanvasRenderingContext2d;
use web_sys::Document;
use web_sys::Request;
use web_sys::RequestInit;
use web_sys::RequestMode;
use web_sys::Response;

use rust_d3_geo::graticule::generate as generate_graticule;
use rust_d3_geo::path::builder::Builder as PathBuilder;
use rust_d3_geo::path::context::Context;
use rust_d3_geo::projection::builder::types::BuilderCircleResampleNoClip;
use rust_d3_geo::projection::orthographic::Orthographic;
use rust_d3_geo::projection::Build;
use rust_d3_geo::projection::ProjectionRawBase;
use rust_d3_geo::projection::RotateSet;
use rust_d3_geo::projection::ScaleSet;
use rust_d3_geo::projection::TranslateSet;
use rust_topojson_client::feature::feature_from_name;
use web_sys::window;
use web_sys::Window;

fn document() -> Result<Document, JsValue> {
    let window = match js_sys::global().dyn_into::<Window>() {
        Ok(w) => w,
        Err(_) => return Err(JsValue::from_str("document() Could not get the window")),
    };

    match window.document() {
        Some(d) => Ok(d),
        None => Err(JsValue::from_str(
            "document() Could not get the docuement from the window.",
        )),
    }
}

// type PB = PathBuilder<
//     ClipCircleC<ResampleNoPCNC<Context, Orthographic<Context, f64>, f64>, f64>,
//     ClipCircleU<ResampleNoPCNC<Context, Orthographic<Context, f64>, f64>, f64>,
//     Context,
//     NoPCNU<Context>,
//     NoPCNC<Context>,
//     Orthographic<Context, f64>,
//     ResampleNoPCNC<Context, Orthographic<Context, f64>, f64>,
//     ResampleNoPCNU<Context, Orthographic<Context, f64>, f64>,
//     f64,
// >;

#[wasm_bindgen]
#[derive(Debug)]
/// State associated with render call.
pub struct Renderer {
    context2d: CanvasRenderingContext2d,
    countries: Geometry<f64>,
    height: f64,
    graticule: Geometry<f64>,
    ob: BuilderCircleResampleNoClip<Context, Orthographic<Context, f64>, f64>,
    width: f64,
    yaw: f64,
}

#[wasm_bindgen]
impl Renderer {
    /// yaw initial rotation.
    /// "/world-atlas/world/50m.json"
    pub async fn new(filename: &str, yaw: f64) -> Result<Renderer, JsValue> {
        utils::set_panic_hook();

        let document = document()?;
        // let window = web_sys::window().expect("Failed to get window");

        let w = match window() {
            Some(w) => w,
            None => {
                return Err(JsValue::from_str("new() Could not get window."));
            }
        };

        // Get data from world map.
        let mut opts = RequestInit::new();
        opts.method("GET");
        opts.mode(RequestMode::Cors);
        let request = Request::new_with_str_and_init(filename, &opts)?;

        let resp_value = JsFuture::from(w.fetch_with_request(&request)).await?;
        let resp: Response = resp_value.dyn_into().unwrap();

        let json = JsFuture::from(resp.json()?).await?;

        let topology =
            JsValueSerdeExt::into_serde::<Topology>(&json).expect("Did not get a valid Topology");

        // Grab canvas.
        let canvas = document
            .get_element_by_id("c")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()?;

        let context2d = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

        // let context: Context = Context::new(context2d.clone());
        // let pb = PathBuilder::new(context);

        let width: f64 = canvas.width().into();
        let height: f64 = canvas.height().into();

        let countries =
            feature_from_name(&topology, "countries").expect("Did not extract geometry");

        // let performance = match w.performance() {
        //     Some(p) => p,
        //     None => {
        //         return Err(JsValue::from_str("new() Could not get performance."));
        //     }
        // };

        let mut ob = Orthographic::builder();
        ob.scale_set(width as f64 / 1.3_f64 / std::f64::consts::PI)
            .translate_set(&Coordinate {
                x: width / 2_f64,
                y: height / 2_f64,
            });

        // Graticule
        let graticule =
            Geometry::MultiLineString(MultiLineString(generate_graticule().lines().collect()));

        Ok(Self {
            context2d,
            countries,
            graticule,
            // pb,
            height,
            ob,
            width,
            yaw,
        })
    }

    /// Render the next frame.
    pub fn render(&mut self) {
        self.ob.rotate_set(&[self.yaw, -45f64, 0f64]);

        let ortho = self.ob.build();
        self.context2d
            .clear_rect(0f64, 0f64, self.width, self.height);

        let context: Context = Context::new(self.context2d.clone());
        let pb = PathBuilder::new(context);

        let mut path = pb.build(ortho);
        self.context2d.set_stroke_style(&"#333".into());
        self.context2d.begin_path();
        path.object(&self.countries);
        self.context2d.stroke();

        self.context2d.begin_path();
        self.context2d.set_stroke_style(&"#ccc".into());
        path.object(&self.graticule);
        self.context2d.stroke();
        self.yaw -= 0.2f64;
    }
}

// /// Entry point
// #[wasm_bindgen(start)]
// pub async fn start() -> Result<(), JsValue> {
//     let document = document()?;
//     let window = web_sys::window().expect("Failed to get window");

//     // Get data from world map.
//     let mut opts = RequestInit::new();
//     opts.method("GET");
//     opts.mode(RequestMode::Cors);
//     let request = Request::new_with_str_and_init("/world-atlas/world/50m.json", &opts)?;

//     let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
//     let resp: Response = resp_value.dyn_into().unwrap();

//     let json = JsFuture::from(resp.json()?).await?;

//     let topology =
//         JsValueSerdeExt::into_serde::<Topology>(&json).expect("Did not get a valid Topology");

//     // Grab canvas.
//     let canvas = document
//         .get_element_by_id("c")
//         .unwrap()
//         .dyn_into::<web_sys::HtmlCanvasElement>()?;

//     let context_raw = canvas
//         .get_context("2d")?
//         .unwrap()
//         .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

//     let width: f64 = canvas.width().into();
//     let height: f64 = canvas.height().into();

//     let countries = feature_from_name(&topology, "countries").expect("Did not extract geometry");

//     let mut yaw = 300_f64;

//     let mut ob = Orthographic::builder();
//     ob.scale_set(width as f64 / 1.3_f64 / std::f64::consts::PI)
//         .translate_set(&Coordinate {
//             x: width / 2_f64,
//             y: height / 2_f64,
//         });

//     // Graticule
//     let graticule =
//         Geometry::MultiLineString(MultiLineString(generate_graticule().lines().collect()));

//     spawn_local(async move {
//         let mut count = 0;

//         IntervalStream::new(20)
//             .take_while(|_| {
//                 count += 1;
//                 future::ready(count < 100)
//             })
//             .for_each(|_| {
//                 let context = Context::new(context_raw.clone());
//                 let pb = PathBuilder::new(context);
//                 ob.rotate_set(&[yaw, -45f64, 0f64]);

//                 let ortho = ob.build();
//                 context_raw.clear_rect(0f64, 0f64, width, height);

//                 let mut path = pb.build(ortho);
//                 context_raw.set_stroke_style(&"#333".into());
//                 context_raw.begin_path();
//                 path.object(&countries);
//                 context_raw.stroke();

//                 context_raw.begin_path();
//                 context_raw.set_stroke_style(&"#ccc".into());
//                 path.object(&graticule);
//                 context_raw.stroke();
//                 yaw -= 0.2f64;

//                 console_log!("yaw :{}", yaw);

//                 future::ready(())
//             })
//             .await;
//     });

//     Ok(())
// }
