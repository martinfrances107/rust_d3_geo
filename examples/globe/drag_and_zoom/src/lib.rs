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

mod exported_point;
mod utils;

use geo::Coord;
use geo::Geometry;
use geo::LineString;
use geo::MultiPolygon;
use geo::Polygon;
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

use d3_geo_rs::circle::generator::Generator as CircleGenerator;
use d3_geo_rs::graticule::generate_mls;
use d3_geo_rs::path::builder::Builder as PathBuilder;
use d3_geo_rs::path::endpoint::Endpoint;
use d3_geo_rs::path::Result as PathResult;
use d3_geo_rs::projection::builder::types::BuilderCircleResampleNoClip;
use d3_geo_rs::projection::orthographic::Orthographic;
use d3_geo_rs::projection::Build;
use d3_geo_rs::projection::RawBase;
use d3_geo_rs::projection::Reflect;
use d3_geo_rs::projection::ReflectSet;
use d3_geo_rs::projection::RotateGet;
use d3_geo_rs::projection::RotateSet;
use d3_geo_rs::projection::ScaleGet;
use d3_geo_rs::projection::ScaleSet;
use d3_geo_rs::projection::TranslateSet;
use d3_geo_rs::Transform;

use crate::exported_point::ExportedPoint;

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

/// Which pattern to display.
#[derive(Debug)]
#[wasm_bindgen]
pub enum SelectedPattern {
    /// Bar is a rectangle on the equator which cross the antimeridian.
    Bar,
    /// Hi res maps of the earth.
    Globe,
    /// Repeated pattern of a disc with a central hole.
    Rings,
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
    graticule: Geometry<f64>,
    pattern: Geometry<f64>,
    projector_builder: BuilderCircleResampleNoClip<Endpoint, Orthographic<f64>, f64>,
}

async fn countries() -> Result<Geometry, JsValue> {
    let Some(w) = window() else {
        return Err(JsValue::from_str("new() Could not get window."));
    };

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let request = match Request::new_with_str_and_init("./world-atlas/world/50m.json", &opts) {
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

    let countries = feature_from_name(&topology, "countries").expect("Did not extract geometry");

    Ok(countries)
}

// Returns the ring pattern
//
// Makes the globe look like a beach ball.
fn rings() -> Geometry {
    let mut cg_outer = CircleGenerator::default();
    cg_outer.radius_set(10_f64).precision_set(10_f64);

    let mut cg_inner = CircleGenerator::default();
    cg_inner.radius_set(5_f64).precision_set(5_f64);

    let mut p_vec: Vec<Polygon<f64>> = vec![];
    for lat in (-30..=30).step_by(30) {
        for long in (-180..=180).step_by(40) {
            let mut inner = cg_inner
                .center_set(&Coord {
                    x: f64::from(long),
                    y: f64::from(lat),
                })
                .circle()
                .exterior()
                .0
                .clone();
            inner.reverse();
            let inner_ring: LineString<f64> = inner.into();

            let poly = Polygon::new(
                cg_outer
                    .center_set(&Coord {
                        x: f64::from(long),
                        y: f64::from(lat),
                    })
                    .circle()
                    .exterior()
                    .clone(),
                vec![inner_ring],
            );

            p_vec.push(poly);
        }
    }

    let object = MultiPolygon(p_vec);

    Geometry::MultiPolygon(object)
}

/// This test pattern is a rectangle that crosses the antimeridian.
fn bar() -> Geometry {
    Geometry::Polygon(Polygon::new(
        LineString(vec![
            Coord {
                x: 170_f64,
                y: 10_f64,
            },
            Coord {
                x: -170_f64,
                y: 10_f64,
            },
            Coord {
                x: -170_f64,
                y: -10_f64,
            },
            Coord {
                x: 170_f64,
                y: -10_f64,
            },
            Coord {
                x: 170_f64,
                y: 10_f64,
            },
        ]),
        vec![],
    ))
}

#[wasm_bindgen]
impl Renderer {
    /// Construct from an initial pattern.
    ///
    /// # Errors
    ///
    /// if the css selector '#c' fails.
    /// if the context type is not available. [ '2d' is not the only one see "webgl" ]
    /// if an inconsisten state associated with the selected pattern was detected.
    pub async fn new(selected_pattern: SelectedPattern) -> Result<Renderer, JsValue> {
        utils::set_panic_hook();

        // If required, Start async file loading process.
        let countries_loading = match selected_pattern {
            SelectedPattern::Globe => Some(countries()),
            SelectedPattern::Bar | SelectedPattern::Rings => None,
        };

        let document = document()?;

        // Grab canvas.
        let canvas: web_sys::HtmlCanvasElement;
        if let Some(element) = document.get_element_by_id("c") {
          canvas = element.dyn_into::<web_sys::HtmlCanvasElement>()?;
        } else {
          return Err(JsValue::from("Did not find the canvas element on the page."))
        }

        let context2d :web_sys::CanvasRenderingContext2d;
        if let Some(context) = canvas.get_context("2d")?{
          context2d = context.dyn_into::<web_sys::CanvasRenderingContext2d>()?;
        } else {
          return Err(JsValue::from("did not get the 2d context"));
        }

        let width: f64 = canvas.width().into();
        let height: f64 = canvas.height().into();

        let mut projector_builder = Orthographic::builder();
        projector_builder
            .scale_set(width / 1.3_f64 / std::f64::consts::PI)
            .translate_set(&Coord {
                x: width / 2_f64,
                y: height / 2_f64,
            });

        // Graticule.
        let graticule = generate_mls();

        let pattern = match countries_loading {
            Some(f) => f.await?,
            None => match selected_pattern {
                SelectedPattern::Bar => bar(),
                SelectedPattern::Rings => rings(),
                SelectedPattern::Globe => {
                    return Err(JsValue::from("Invalid state: Was not loading but the selected pattern was globe."));
                }
            },
        };

        Ok(Self {
            color_inner_stroke: "#777".into(),
            color_inner_fill: "#888".into(),
            color_outer_stroke: "#000".into(),
            color_outer_fill: "#111".into(),
            color_graticule: "#ccc".into(),
            context2d,
            graticule,
            pattern,
            projector_builder,
        })
    }

    /// Change the pattern rendered
    ///
    /// # Errors
    ///
    /// If the data file associated with the country cannot be loaded.
    pub async fn pattern_change(&mut self, p: SelectedPattern) -> Result<(), JsValue> {
        match p {
            SelectedPattern::Bar => {
                self.pattern = bar();
            }
            SelectedPattern::Globe => {
                self.pattern = countries().await?;
            }
            SelectedPattern::Rings => {
                self.pattern = rings();
            }
        }
        Ok(())
    }

    /// Transform a point base in the renderer's transform.
    pub fn transform_in_place(&self, p: &mut ExportedPoint) {
        let p_out = self.projector_builder.transform(&Coord { x: p.x, y: p.y });

        p.x = p_out.x;
        p.y = p_out.y;
    }

    /// Returns a coordinate based on the renderer's transform.
    #[must_use]
    pub fn transform(&self, p: &ExportedPoint) -> ExportedPoint {
        let p_out = self.projector_builder.transform(&Coord { x: p.x, y: p.y });
        ExportedPoint::new(p_out.x, p_out.y)
    }

    /// Set the builder's scale.
    pub fn scale_set(&mut self, scale: f64) {
        self.projector_builder.scale_set(scale);
    }

    /// Get the builder's scale.
    #[must_use]
    pub fn scale(&self) -> f64 {
        self.projector_builder.scale()
    }

    /// Returns a coordinate based on the renderer's invert transform.
    #[must_use]
    pub fn invert(&self, p: &ExportedPoint) -> ExportedPoint {
        let p_out = self.projector_builder.invert(&Coord { x: p.x, y: p.y });
        ExportedPoint::new(p_out.x, p_out.y)
    }

    #[must_use]
    /// Returns the builders angle settings.
    pub fn rotate(&self) -> Array {
        self.projector_builder
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
        self.projector_builder
            .rotate3_set(&[angles[0], angles[1], angles[2]]);
    }

    /// Render the next frame.
    ///
    /// # Panics
    ///
    /// If a Path2d object cannot be created.
    pub fn render(&mut self, solid: bool) {
        if !solid {
            let r = self.projector_builder.rotate();
            self.projector_builder.reflect_x_set(Reflect::Flipped);
            self.projector_builder
                .rotate3_set(&[r[0] + 180_f64, -r[1], -r[2]]);

            let projector = self.projector_builder.build();
            let path2d = Path2d::new().unwrap();
            let ep = Endpoint::new(path2d);
            let path_builder = PathBuilder::new(ep);

            let mut path = path_builder.build(projector);
            self.context2d.set_stroke_style(&self.color_inner_stroke);
            self.context2d.set_fill_style(&self.color_inner_fill);
            path.object(&self.pattern);
            let path2d = path.context.result();
            self.context2d.stroke_with_path(&path2d);
            self.context2d.fill_with_path_2d(&path2d);

            self.projector_builder.reflect_x_set(Reflect::Unflipped);
            self.projector_builder.rotate3_set(&r);
        }

        let projector = self.projector_builder.build();
        let path2d = Path2d::new().unwrap();
        let ep = Endpoint::new(path2d);
        let path_builder = PathBuilder::new(ep);

        let mut path = path_builder.build(projector);
        self.context2d.set_fill_style(&self.color_outer_fill);
        self.context2d.set_stroke_style(&self.color_outer_stroke);
        path.object(&self.pattern);
        let path2d = path.context.result();
        self.context2d.stroke_with_path(&path2d);
        self.context2d.fill_with_path_2d(&path2d);

        self.context2d.set_stroke_style(&self.color_graticule);
        path.object(&self.graticule);
        let path2d = path.context.result();
        self.context2d.stroke_with_path(&path2d);
    }
}
