#![deny(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::perf)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

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
use d3_geo_rs::path::path2d_endpoint::Path2dEndpoint;
use d3_geo_rs::projection::builder::types::BuilderCircleResampleNoClip;
use d3_geo_rs::projection::orthographic::Orthographic;
use d3_geo_rs::projection::Build;
use d3_geo_rs::projection::RawBase;
use d3_geo_rs::projection::RotateSet;
use d3_geo_rs::projection::ScaleSet;
use d3_geo_rs::projection::TranslateSet;

fn document() -> Result<Document, JsValue> {
    let Ok(window) = js_sys::global().dyn_into::<Window>() else {
        return Err(JsValue::from_str("document() Could not get the window"));
    };

    window.document().map_or_else(
        || {
            Err(JsValue::from_str(
                "document() Could not get the document from the window.",
            ))
        },
        Ok,
    )
}

#[wasm_bindgen]
#[derive(Debug)]
/// State associated with render call.
pub struct Renderer {
    // A string owned by javascript representing the HTML color used in the graticule.
    color_graticule: String,
    // A string owned by javascript represent the HTML color used to draw the countries.
    color_land: String,
    context2d: CanvasRenderingContext2d,
    countries: Geometry<f64>,
    height: f64,
    graticule: Geometry<f64>,
    builder:
        BuilderCircleResampleNoClip<Path2dEndpoint, Orthographic<f64>, f64>,
    width: f64,
    yaw: f64,
}

#[wasm_bindgen]
impl Renderer {
    /// filename: of atlas - "/world-atlas/world/50m.json"
    /// yaw initial rotation.
    ///
    /// # Panics
    /// # Errors
    /// When the window could not be obtained.
    /// When geoJson file cannot be obtained/fetched
    /// When the Topology deserialization failed.
    /// When the canvas element could not be obtained.
    pub async fn new(filename: &str, yaw: f64) -> Result<Self, JsValue> {
        utils::set_panic_hook();

        let document = document()?;

        let Some(w) = window() else {
            return Err(JsValue::from_str("new() Could not get window."));
        };

        // Get data from world map.
        let opts = RequestInit::new();
        opts.set_method("GET");
        opts.set_mode(RequestMode::Cors);
        let request = Request::new_with_str_and_init(filename, &opts)?;

        let resp_value = JsFuture::from(w.fetch_with_request(&request)).await?;
        let resp: Response = resp_value.dyn_into().unwrap();

        let json = JsFuture::from(resp.json()?).await?;

        let topology = JsValueSerdeExt::into_serde::<Topology>(&json)
            .expect("Did not get a valid Topology");

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

        let mut builder = Orthographic::builder();
        builder
            .scale_set(width / 1.3_f64 / std::f64::consts::PI)
            .translate_set(&Coord {
                x: width / 2_f64,
                y: height / 2_f64,
            });

        // Graticule
        let graticule = generate_mls();

        Ok(Self {
            color_graticule: "#ccc".into(),
            color_land: "#333".into(),
            context2d,
            countries: feature_from_name(&topology, "countries")
                .expect("Did not extract geometry"),
            graticule,
            // pb,
            height,
            builder,
            width,
            yaw,
        })
    }

    /// Render the next frame.
    ///
    /// # Panics
    ///
    /// When the Path2d Object cannot be obtained from the system.
    ///
    pub fn render(&mut self) {
        self.builder.rotate2_set(&[self.yaw, -45f64]);

        let projector = self.builder.build();

        self.context2d
            .clear_rect(0f64, 0f64, self.width, self.height);

        let path2d = Path2d::new().unwrap();
        let ep = Path2dEndpoint::new(path2d);
        let path_builder = PathBuilder::new(ep);

        let mut path = path_builder.build(projector);
        self.context2d.set_stroke_style_str(&self.color_land);
        let path2d = path.object(&self.countries);
        self.context2d.stroke_with_path(&path2d);

        self.context2d.set_stroke_style_str(&self.color_graticule);
        let path2d = path.object(&self.graticule);
        self.context2d.stroke_with_path(&path2d);
        self.yaw -= 0.2f64;
    }
}
