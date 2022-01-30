use std::rc::Rc;

use geo::Coordinate;
use geo::Geometry;
use geo::MultiLineString;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use rust_d3_geo::graticule::generate as generate_graticule;
use rust_d3_geo::path::builder::Builder as PathBuilder;
use rust_d3_geo::path::context::Context;
use rust_d3_geo::projection::azimuthal_equidistant::AzimuthalEquiDistant;
use rust_d3_geo::projection::ClipAngle;
use rust_d3_geo::projection::Precision;
use rust_d3_geo::projection::Raw;
use rust_d3_geo::projection::Scale;
use rust_d3_geo::projection::Translate;

use crate::get_document;

pub fn draw_azimuthal_equidistant(land: &Geometry<f64>) -> Result<(), JsValue> {
    let document = get_document()?;
    // Grab canvas.
    let canvas = document
        .get_element_by_id("azimuthal-equidistant-rust")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context_raw = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let context = Rc::new(context_raw);

    let width: f64 = canvas.width().into();
    let height: f64 = canvas.height().into();

    let cs: Context<f64> = Context::new(context.clone());
    let pb = PathBuilder::new(cs);

    let ortho_builder = AzimuthalEquiDistant::<Context<f64>, f64>::builder();

    let azimuthal_equidistant = ortho_builder
        .scale(width as f64 / 3_f64)
        .translate(&Coordinate {
            x: width / 2_f64,
            y: height / 2_f64,
        })
        .clip_angle(90_f64)
        .precision(&10_f64)
        .build();

    let mut path = pb.build(azimuthal_equidistant);
    context.set_stroke_style(&"#69b3a2".into());
    path.object(land);
    context.stroke();

    let lines = generate_graticule().lines();
    let mls = Geometry::MultiLineString(MultiLineString(lines.collect()));
    context.begin_path();
    context.set_fill_style(&"#999".into());
    context.set_stroke_style(&"#69b3a2".into());
    path.object(&mls);
    context.stroke();

    Ok(())
}
