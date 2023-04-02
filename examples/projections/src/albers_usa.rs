use geo::Geometry;
use geo_types::Coord;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use d3_geo_rs::multidrain::Multidrain;
use d3_geo_rs::path::builder::Builder as PathBuilder;
use d3_geo_rs::path::context::Context;
use d3_geo_rs::projection::albers_usa::AlbersUsa;
use d3_geo_rs::projection::RawBase;
use d3_geo_rs::projection::ScaleSet;
use d3_geo_rs::projection::TranslateSet;

use crate::document;

pub async fn draw_albers_usa(land: &Geometry<f64>) -> Result<(), JsValue> {
    let document = document()?;
    // Grab canvas.
    let canvas = document
        .get_element_by_id("albers-usa-rust")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context_raw = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let width: f64 = canvas.width().into();
    let height: f64 = canvas.height().into();
    // let context: Context::new(context_raw.clone();
    let context = Multidrain::<Context, f64>::default();

    let pb = PathBuilder::new(context);

    let mut builder = AlbersUsa::builder();
    let builder = builder.scale_set(width).translate_set(&Coord {
        x: width / 2_f64,
        y: height / 2_f64,
    });

    let projector = builder.build();

    let mut path = pb.build(projector);

    context_raw.set_stroke_style(&"#69b3a2".into());
    path.object(land);
    context_raw.stroke();

    Ok(())
}
