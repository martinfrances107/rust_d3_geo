use geo::Coordinate;
use geo::Geometry;
use geo::MultiLineString;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use rust_d3_geo::graticule::generate as generate_graticule;
use rust_d3_geo::path::builder::Builder as PathBuilder;
use rust_d3_geo::path::context::Context;
use rust_d3_geo::projection::mercator::Mercator;
use rust_d3_geo::projection::Build;
use rust_d3_geo::projection::ClipAngleSet;
use rust_d3_geo::projection::PrecisionAdjust;
use rust_d3_geo::projection::RawBase;
use rust_d3_geo::projection::ScaleSet;
use rust_d3_geo::projection::TranslateSet;

use crate::document;

pub async fn draw_mercator(land: &Geometry<f64>) -> Result<(), JsValue> {
    let document = document()?;
    // Grab canvas.
    let canvas = document
        .get_element_by_id("mercator-rust")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context_raw = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let width: f64 = canvas.width().into();
    let height: f64 = canvas.height().into();

    let context = Context::new(context_raw.clone());
    let pb = PathBuilder::new(context);

    let mut mercator = Mercator::builder();
    let mercator = mercator.scale_set(width as f64 / 1.3_f64 / std::f64::consts::PI);
    mercator.translate_set(&Coordinate {
        x: width / 2_f64,
        y: height / 2_f64,
    });
    let mercator = mercator
        .clip_angle_set(90_f64)
        .precision_set(&10_f64)
        .build();

    let mut path = pb.build(mercator);
    context_raw.set_stroke_style(&"#69b3a2".into());
    path.object(land);
    context_raw.stroke();

    let graticule = generate_graticule();
    let lines = graticule.lines();
    let mls = Geometry::MultiLineString(MultiLineString(lines.collect()));
    context_raw.begin_path();
    context_raw.set_fill_style(&"#999".into());
    context_raw.set_stroke_style(&"#69b3a2".into());
    path.object(&mls);
    context_raw.stroke();

    Ok(())
}
