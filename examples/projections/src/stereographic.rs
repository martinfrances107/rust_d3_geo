use geo::Geometry;
use geo::MultiLineString;
use geo_types::Coord;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Path2d;

use crate::document;

use d3_geo_rs::graticule::generate as generate_graticule;
use d3_geo_rs::path::builder::Builder as PathBuilder;
use d3_geo_rs::path::context::Context;
use d3_geo_rs::projection::stereographic::Stereographic;
use d3_geo_rs::projection::Build;
use d3_geo_rs::projection::ClipAngleAdjust;
use d3_geo_rs::projection::PrecisionAdjust;
use d3_geo_rs::projection::RawBase;
use d3_geo_rs::projection::ScaleSet;
use d3_geo_rs::projection::TranslateSet;

pub async fn draw_sterographic(land: &Geometry<f64>) -> Result<(), JsValue> {
    let document = document()?;
    // Grab canvas.
    let canvas = document
        .get_element_by_id("stereographic-rust")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context_raw = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let width: f64 = canvas.width().into();
    let height: f64 = canvas.height().into();

    let path2d = Path2d::new().unwrap();

    let context: Context = Context::new(path2d);
    let pb = PathBuilder::new(context.clone());

    let stereographic = Stereographic::builder()
        .scale_set(width / 1.3_f64 / std::f64::consts::PI)
        .translate_set(&Coord {
            x: width / 2_f64,
            y: height / 2_f64,
        })
        .clip_angle(90_f64)
        .precision_set(&10_f64)
        .build();

    let mut path = pb.build(stereographic);
    context_raw.set_stroke_style(&"#69b3a2".into());
    path.object(land);
    let path2d = context.path2d.as_ref().unwrap();
    context_raw.stroke_with_path(path2d);

    let graticule = generate_graticule();
    let lines = graticule.lines();
    let mls = Geometry::MultiLineString(MultiLineString(lines.collect()));
    context_raw.begin_path();
    context_raw.set_fill_style(&"#999".into());
    context_raw.set_stroke_style(&"#69b3a2".into());
    path.object(&mls);
    let path2d = context.path2d.unwrap();
    context_raw.stroke_with_path(&path2d);

    Ok(())
}
