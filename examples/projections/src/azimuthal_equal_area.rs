use geo::Coordinate;
use geo::Geometry;
use geo::MultiLineString;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use rust_d3_geo::graticule::generate as generate_graticule;
use rust_d3_geo::path::builder::Builder as PathBuilder;
use rust_d3_geo::path::context::Context;
use rust_d3_geo::projection::azimuthal_equal_area::AzimuthalEqualArea;
use rust_d3_geo::projection::Build;
use rust_d3_geo::projection::PrecisionAdjust;
use rust_d3_geo::projection::ProjectionRawBase;
use rust_d3_geo::projection::ScaleSet;
use rust_d3_geo::projection::TranslateSet;

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

	let context = Context::new(context_raw.clone());
	let pb = PathBuilder::new(context);

	let azimuthal_equal_area = AzimuthalEqualArea::builder()
		.scale_set(width as f64 / 3_f64)
		.translate_set(&Coordinate {
			x: width / 2_f64,
			y: height / 2_f64,
		})
		.precision_set(&0.1_f64)
		.build();

	let mut path = pb.build(azimuthal_equal_area);
	context_raw.set_stroke_style(&"#69b3a2".into());
	path.object(land);
	context_raw.stroke();

	let lines = generate_graticule().lines();
	let mls = Geometry::MultiLineString(MultiLineString(lines.collect()));
	context_raw.begin_path();
	context_raw.set_fill_style(&"#999".into());
	context_raw.set_stroke_style(&"#69b3a2".into());
	path.object(&mls);
	context_raw.stroke();

	Ok(())
}
