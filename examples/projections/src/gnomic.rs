use geo::Geometry;
use geo::MultiLineString;
use geo_types::Coord;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use d3_geo_rs::graticule::generate as generate_graticule;
use d3_geo_rs::path::builder::Builder as PathBuilder;
use d3_geo_rs::path::context::Context;
use d3_geo_rs::projection::builder::template::NoPCNC;
use d3_geo_rs::projection::gnomic::Gnomic;
use d3_geo_rs::projection::Build;
use d3_geo_rs::projection::ClipAngleAdjust;
use d3_geo_rs::projection::PrecisionAdjust;
use d3_geo_rs::projection::RawBase;
use d3_geo_rs::projection::ScaleSet;
use d3_geo_rs::projection::TranslateSet;

use crate::document;

pub async fn draw_gnomic(land: &Geometry<f64>) -> Result<(), JsValue> {
    let document = document()?;
    // Grab canvas.
    let canvas = document
        .get_element_by_id("gnomic-rust")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context_raw = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let width: f64 = canvas.width().into();
    let height: f64 = canvas.height().into();

    let context: Context = Context::new(context_raw.clone());
    let pb: PathBuilder<_, _, _, NoPCNC<Context>, _, _, _, _, _> = PathBuilder::new(context);
    // let pb = PathBuilder::new(context);

    let gnomic = Gnomic::builder()
        .scale_set(width / 6_f64)
        .translate_set(&Coord {
            x: width / 2_f64,
            y: height / 2_f64,
        })
        .precision_set(&0.3_f64)
        .clip_angle(90_f64 - 1e-3)
        .build();

    let mut path = pb.build(gnomic);
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
