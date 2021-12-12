#![allow(clippy::pedantic)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
//! # rust d3 geo
//!
//! See the README.md.
extern crate js_sys;
extern crate rand;
extern crate web_sys;

use std::iter::repeat_with;
use std::rc::Rc;

use geo::Coordinate;
use geo::Geometry;
use geo::MultiLineString;
use rust_d3_geo::projection::ClipAngle;
use rust_d3_geo::projection::Scale;
use rust_d3_geo::projection::Translate;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Document;

use rust_d3_geo::clip::circle::line::Line;
use rust_d3_geo::clip::circle::pv::PV;
use rust_d3_geo::data_object::DataObject;
use rust_d3_geo::graticule::generate as generate_graticule;
use rust_d3_geo::path::builder::Builder as PathBuilder;
use rust_d3_geo::path::context::Context;
use rust_d3_geo::path::context_stream::ContextStream;
use rust_d3_geo::projection::builder::Builder as ProjectionBuilder;
use rust_d3_geo::projection::orthographic::Orthographic;
use rust_d3_geo::projection::Raw;

mod dom_macros;

type Result<T> = std::result::Result<T, JsValue>;

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

fn get_document() -> Result<Document> {
    let window = web_sys::window().unwrap();
    Ok(window.document().unwrap())
}

/// Entry point.
#[wasm_bindgen]
pub fn run() -> Result<()> {
    let document = get_document()?;
    update_canvas(&document)?;

    Ok(())
}

// draw dot
fn update_canvas(document: &Document) -> Result<()> {
    // grab canvas
    let canvas = document
        .get_element_by_id("c")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context_raw = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let context = Rc::new(context_raw);

    let width = canvas.width().into();
    let height = canvas.height().into();
    context.set_fill_style(&"black".into());
    context.set_stroke_style(&"black".into());
    context.fill_rect(0.0, 0.0, width, height);

    let cs: ContextStream<f64> = ContextStream::Context(Context::new(context.clone()));
    let pb: PathBuilder<Line<f64>, Orthographic<ContextStream<f64>, f64>, PV<f64>, f64> =
        PathBuilder::new(cs);

    let ortho_builder: ProjectionBuilder<
        ContextStream<f64>,
        Line<f64>,
        Orthographic<ContextStream<f64>, f64>,
        PV<f64>,
        f64,
    > = Orthographic::builder();

    let ortho = ortho_builder
        .scale(240_f64)
        .translate(&Coordinate {
            x: width / 2_f64,
            y: height / 2_f64,
        })
        .clip_angle(90_f64 + 1e-6)
        .build();

    let mut path = pb.build(ortho.clone());

    let lines = generate_graticule().lines();

    let mls = DataObject::Geometry(Geometry::MultiLineString(MultiLineString(lines)));
    context.begin_path();
    context.set_fill_style(&"#999".into());
    context.set_stroke_style(&"#69b3a2".into());
    path.object(&mls);

    context.stroke();

    Ok(())
}
