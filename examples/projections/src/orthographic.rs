use geo::Coordinate;
use geo::Geometry;
use geo::MultiLineString;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use rust_d3_geo::graticule::generate as generate_graticule;
use rust_d3_geo::path::builder::Builder as PathBuilder;
use rust_d3_geo::path::context::Context;
use rust_d3_geo::projection::orthographic::Orthographic;
use rust_d3_geo::projection::ProjectionRawBase;
use rust_d3_geo::projection::ScaleAdjust;
use rust_d3_geo::projection::TranslateAdjust;

use crate::get_document;

pub async fn draw_orthographic(land: &Geometry<f64>) -> Result<(), JsValue> {
	let document = get_document()?;
	// Grab canvas.
	let canvas = document
		.get_element_by_id("orthographic-rust")
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

	let ortho_builder = Orthographic::<Context<f64>, f64>::builder();

	let ortho = ortho_builder
		.scale(width as f64 / 1.3_f64 / std::f64::consts::PI)
		.translate_adjust(&Coordinate {
			x: width / 2_f64,
			y: height / 2_f64,
		})
		.build();

	let mut path = pb.build(ortho);
	context_raw.set_stroke_style(&"#69b3a2".into());
	context_raw.set_fill_style(&"#2a2a2a".into());
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
