use std::rc::Rc;

use geo::Coordinate;
use geo::Geometry;

use rust_d3_geo::projection::Center;
use rust_d3_geo::projection::Rotate;
use rust_d3_geo::projection::equirectangular::Equirectangular;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use rust_d3_geo::path::builder::Builder as PathBuilder;
use rust_d3_geo::path::context::Context;

use rust_d3_geo::projection::Raw;
use rust_d3_geo::projection::Scale;
use rust_d3_geo::projection::Translate;

use crate::get_document;

pub fn draw_equirectangular(land: &Geometry<f64>)-> Result<(), JsValue>{

	let document = get_document()?;
    // Grab canvas.
    let canvas = document
        .get_element_by_id("equirectangular-rust")
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
    let pb: PathBuilder<Context<f64>, _, Equirectangular<Context<f64>, f64>, _, f64> =
        PathBuilder::new(cs);

    let equirectangular_builder = Equirectangular::<Context<f64>, f64>::builder();

    let equirectangular = equirectangular_builder
        .scale(width as f64 / 1.5_f64 / std::f64::consts::PI)
        .rotate(&[0_f64, 0_f64, 0_f64])
        .center(&Coordinate{x: 0_f64, y: 0_f64})
        .translate(&Coordinate {
            x: width / 2_f64,
            y: height / 2_f64,
        })
        .build();

    let mut path = pb.build(equirectangular);
    context.set_stroke_style(&"#69b3a2".into());
    path.object(land);
    context.stroke();

	// let graticule10 = Graticule10::new();

	Ok(())
}
