use geo::Geometry;
use geo::MultiLineString;
use geo_types::Coord;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use d3_geo_rs::graticule::generate as generate_graticule;
use d3_geo_rs::path::builder::Builder as PathBuilder;
use d3_geo_rs::path::endpoint::Endpoint;
use d3_geo_rs::path::Result as PathResult;
use d3_geo_rs::projection::azimuthal_equal_area::AzimuthalEqualArea;
use d3_geo_rs::projection::Build;
use d3_geo_rs::projection::PrecisionAdjust;
use d3_geo_rs::projection::RawBase;
use d3_geo_rs::projection::ScaleSet;
use d3_geo_rs::projection::TranslateSet;
use web_sys::Path2d;

use crate::document;

pub async fn draw_azimuthal_equal_area(land: &Geometry<f64>) -> Result<(), JsValue> {
    let document = document()?;
    // Grab canvas.
    let canvas = document
        .get_element_by_id("azimuthal-equal-area-rust")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context_raw = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let width: f64 = canvas.width().into();
    let height: f64 = canvas.height().into();

    let path2d = Path2d::new()?;

    let context: Endpoint = Endpoint::new(path2d);
    let pb = PathBuilder::new(context);

    let azimuthal_equal_area = AzimuthalEqualArea::builder()
        .scale_set(width / 3_f64)
        .translate_set(&Coord {
            x: width / 2_f64,
            y: height / 2_f64,
        })
        .precision_set(&0.1_f64)
        .build();

    let mut path = pb.build(azimuthal_equal_area);
    context_raw.set_stroke_style(&"#69b3a2".into());
    path.object(land);
    let path2d = path.context_stream.result();
    context_raw.stroke_with_path(&path2d);

    let graticule = generate_graticule();
    let lines = graticule.lines();
    let mls = Geometry::MultiLineString(MultiLineString(lines.collect()));
    context_raw.set_fill_style(&"#999".into());
    context_raw.set_stroke_style(&"#69b3a2".into());
    path.object(&mls);
    let path2d = path.context_stream.result();
    context_raw.stroke_with_path(&path2d);

    Ok(())
}
