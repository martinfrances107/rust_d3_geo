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

use geo::Geometry;
use geo_types::Coord;
use gloo_utils::format::JsValueSerdeExt;
use topojson::Topology;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;
use web_sys::CanvasRenderingContext2d;
use web_sys::Document;
use web_sys::Request;
use web_sys::RequestInit;
use web_sys::RequestMode;
use web_sys::Response;
use web_sys::Window;

use d3_geo_rs::graticule::generate_mls;
use d3_geo_rs::path::builder::Builder as PathBuilder;
use d3_geo_rs::path::context::Context;
use d3_geo_rs::projection::builder::types::BuilderCircleResampleNoClip;
use d3_geo_rs::projection::orthographic::Orthographic;
use d3_geo_rs::projection::Build;
use d3_geo_rs::projection::RawBase;
use d3_geo_rs::projection::RotateSet;
use d3_geo_rs::projection::ScaleSet;
use d3_geo_rs::projection::TranslateSet;
use rust_topojson_client::feature::feature_from_name;

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
    /// filename: of atlas - "/world-atlas/world/50m.json"
    /// yaw initial rotation.
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

        let width: f64 = canvas.width().into();
        let height: f64 = canvas.height().into();

        let countries =
            feature_from_name(&topology, "countries").expect("Did not extract geometry");

        let mut ob = Orthographic::builder();
        ob.scale_set(width / 1.3_f64 / std::f64::consts::PI)
            .translate_set(&Coord {
                x: width / 2_f64,
                y: height / 2_f64,
            });

        // Graticule
        let graticule = generate_mls::<f64>();

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
        self.ob.rotate2_set(&[self.yaw, -45f64]);

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
