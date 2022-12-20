use geo::Geometry;
use geo::MultiLineString;
use geo_types::Coord;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use rust_d3_geo::graticule::generate as generate_graticule;
use rust_d3_geo::path::builder::Builder as PathBuilder;
use rust_d3_geo::path::context::Context;
use rust_d3_geo::projection::equirectangular::Equirectangular;
use rust_d3_geo::projection::Build;
use rust_d3_geo::projection::CenterSet;
use rust_d3_geo::projection::RawBase;
use rust_d3_geo::projection::RotateSet;
use rust_d3_geo::projection::ScaleSet;
use rust_d3_geo::projection::TranslateSet;

use crate::document;

pub async fn draw_equirectangular(land: &Geometry<f64>) -> Result<(), JsValue> {
    let document = document()?;
    // Grab canvas.
    let canvas = document
        .get_element_by_id("equirectangular-rust")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context_raw = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let context = context_raw.clone();

    let width: f64 = canvas.width().into();
    let height: f64 = canvas.height().into();

    let context = Context::new(context);
    let pb = PathBuilder::new(context);

    let equirectangular = Equirectangular::builder()
        .scale_set(width / 1.5_f64 / std::f64::consts::PI)
        .rotate2_set(&[0_f64, 0_f64])
        .center_set(&Coord { x: 0_f64, y: 0_f64 })
        .translate_set(&Coord {
            x: width / 2_f64,
            y: height / 2_f64,
        })
        .build();

    let mut path = pb.build(equirectangular);
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
