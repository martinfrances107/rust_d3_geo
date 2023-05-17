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

mod utils;

use geo::Coord;
use geo::Geometry;
use gloo_utils::format::JsValueSerdeExt;
use js_sys::Array;
use rust_topojson_client::feature::feature_from_name;
use topojson::Topology;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;
use web_sys::CanvasRenderingContext2d;
use web_sys::Document;
use web_sys::Path2d;
use web_sys::Request;
use web_sys::RequestInit;
use web_sys::RequestMode;
use web_sys::Response;
use web_sys::Window;

use d3_geo_rs::graticule::generate_mls;
use d3_geo_rs::path::builder::Builder as PathBuilder;
use d3_geo_rs::path::context::Context;
use d3_geo_rs::path::Result as PathResult;
use d3_geo_rs::projection::builder::types::BuilderCircleResampleNoClip;
use d3_geo_rs::projection::orthographic::Orthographic;
use d3_geo_rs::projection::Build;
use d3_geo_rs::projection::RawBase;
use d3_geo_rs::projection::ReflectSet;
use d3_geo_rs::projection::RotateGet;
use d3_geo_rs::projection::RotateSet;
use d3_geo_rs::projection::ScaleGet;
use d3_geo_rs::projection::ScaleSet;
use d3_geo_rs::projection::TranslateSet;
use d3_geo_rs::projection::REFLECT;
use d3_geo_rs::Transform;

fn document() -> Result<Document, JsValue> {
    let Ok(window) = js_sys::global().dyn_into::<Window>() else { return Err(JsValue::from_str("document() Could not get the window")) };

    window.document().map_or_else(
        || {
            Err(JsValue::from_str(
                "document() Could not get the docuement from the window.",
            ))
        },
        Ok,
    )
}

/// Used to export a `Coord<f64>` to javascript.
#[derive(Debug)]
#[wasm_bindgen]
pub struct ExportedPoint {
    /// x coordinate.
    pub x: f64,
    /// y coordinate.
    pub y: f64,
}

#[wasm_bindgen]
#[allow(clippy::missing_const_for_fn)]
impl ExportedPoint {
    /// Constructor.
    #[wasm_bindgen(constructor)]
    #[must_use]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Getter
    #[must_use]
    pub fn get_x(&self) -> f64 {
        self.x
    }

    /// Getter
    #[must_use]
    pub fn get_y(&self) -> f64 {
        self.y
    }
}

#[wasm_bindgen]
#[derive(Debug)]
/// State associated with render call.
pub struct Renderer {
    color_inner_stroke: JsValue,
    color_inner_fill: JsValue,
    color_outer_stroke: JsValue,
    color_outer_fill: JsValue,
    color_graticule: JsValue,
    context2d: CanvasRenderingContext2d,
    countries: Geometry<f64>,
    graticule: Geometry<f64>,
    ob: BuilderCircleResampleNoClip<Context, Orthographic<f64>, f64>,
}

#[wasm_bindgen]
impl Renderer {
    /// yaw initial rotation.
    /// "/world-atlas/world/50m.json"
    pub async fn new(filename: &str) -> Result<Renderer, JsValue> {
        utils::set_panic_hook();

        let document = document()?;

        let Some(w) = window() else {
                         return Err(JsValue::from_str("new() Could not get window."));
                     };

        // Get data from world map.
        let mut opts = RequestInit::new();
        opts.method("GET");
        opts.mode(RequestMode::Cors);
        let request = match Request::new_with_str_and_init(filename, &opts) {
            Ok(r) => r,
            Err(e) => {
                return Err(e);
            }
        };

        let resp_fetch = JsFuture::from(w.fetch_with_request(&request));
        let resp_value = match resp_fetch.await {
            Ok(r) => r,
            Err(e) => {
                return Err(e);
            }
        };
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
        let graticule = generate_mls();

        Ok(Self {
            color_inner_stroke: "#777".into(),
            color_inner_fill: "#888".into(),
            color_outer_stroke: "#000".into(),
            color_outer_fill: "#111".into(),
            color_graticule: "#ccc".into(),
            context2d,
            countries,
            graticule,
            ob,
        })
    }

    /// Transform a point base in the renderer's transform.
    pub fn transform_in_place(&self, p: &mut ExportedPoint) {
        let p_out = self.ob.transform(&Coord { x: p.x, y: p.y });

        p.x = p_out.x;
        p.y = p_out.y;
    }

    /// Returns a coordinate based on the renderer's transform.
    #[must_use]
    pub fn transform(&self, p: &ExportedPoint) -> ExportedPoint {
        let p_out = self.ob.transform(&Coord { x: p.x, y: p.y });
        ExportedPoint::new(p_out.x, p_out.y)
    }

    /// Set the builder scale.
    pub fn scale_set(&mut self, scale: f64) {
        self.ob.scale_set(scale);
    }

    /// Set the builder scale.
    #[must_use]
    pub fn scale(&self) -> f64 {
        self.ob.scale()
    }

    /// Returns a coordinate based on the renderer's invert transform.
    #[must_use]
    pub fn invert(&self, p: &ExportedPoint) -> ExportedPoint {
        let p_out = self.ob.invert(&Coord { x: p.x, y: p.y });
        ExportedPoint::new(p_out.x, p_out.y)
    }

    #[must_use]
    /// Returns the builders angle settings.
    pub fn rotate(&self) -> Array {
        self.ob
            .rotate()
            .into_iter()
            .map(JsValue::from_f64)
            .collect::<Array>()
    }

    /// Sets the rotation
    pub fn rotate_set(&mut self, angles_js: &Array) {
        let angles = angles_js
            .iter()
            .map(&mut |x: JsValue| x.as_f64().unwrap_or_default())
            .collect::<Vec<f64>>();
        self.ob.rotate3_set(&[angles[0], angles[1], angles[2]]);
    }

    /// Render the next frame.
    pub fn render(&mut self, solid: bool) {
        let path2d = Path2d::new().unwrap();
        let context: Context = Context::new(path2d);

        if !solid {
            let r = self.ob.rotate();
            self.ob.reflect_x_set(REFLECT::Flipped);
            self.ob.rotate3_set(&[r[0] + 180_f64, -r[1], -r[2]]);

            let ortho = self.ob.build();
            let pb = PathBuilder::new(context.clone());

            let mut path = pb.build(ortho);
            self.context2d.set_stroke_style(&self.color_inner_stroke);
            self.context2d.set_fill_style(&self.color_inner_fill);
            path.object(&self.countries);
            let path2d = path.context_stream.result();

            self.context2d.stroke_with_path(&path2d);
            self.context2d.fill_with_path_2d(&path2d);

            self.ob.reflect_x_set(REFLECT::Unflipped);
            self.ob.rotate3_set(&r);
        }

        let ortho = self.ob.build();

        let pb = PathBuilder::new(context.clone());

        let mut path = pb.build(ortho);
        self.context2d.set_fill_style(&self.color_outer_fill);
        self.context2d.set_stroke_style(&self.color_outer_stroke);
        path.object(&self.countries);
        let path2d = path.context_stream.result();
        self.context2d.stroke_with_path(&path2d);
        self.context2d.fill_with_path_2d(&path2d);

        self.context2d.set_stroke_style(&self.color_graticule);
        path.object(&self.graticule);
        let path2d = path.context_stream.result();
        self.context2d.stroke_with_path(&path2d);
    }
}
