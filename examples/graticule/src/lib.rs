#![allow(clippy::pedantic)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
//! # rust d3 geo
//!
//! See the README.md.
extern crate js_sys;
extern crate rand;
extern crate web_sys;

use geo::Coordinate;
use geo::Geometry;
use geo::LineString;
use geo::MultiLineString;
use geo::MultiPolygon;
use geo::Polygon;
use rust_d3_geo::projection::ClipAngleAdjust;
use rust_d3_geo::projection::ProjectionRawBase;
use rust_d3_geo::projection::Scale;
use rust_d3_geo::projection::Translate;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Document;
use web_sys::Element;

// use rust_d3_geo::clip::circle::line::Line;
// use rust_d3_geo::clip::circle::pv::PV;
use rust_d3_geo::graticule::generate as generate_graticule;
use rust_d3_geo::path::builder::Builder as PathBuilder;
use rust_d3_geo::path::context::Context;
// use rust_d3_geo::projection::builder::Builder as ProjectionBuilder;
use rust_d3_geo::projection::orthographic::Orthographic;
// use rust_d3_geo::projection::ClipAngleSet;
use rust_d3_geo::projection::Rotate;
use web_sys::SvgsvgElement;

mod dom_macros;

type Result<T> = std::result::Result<T, JsValue>;

#[wasm_bindgen]
extern "C" {
	// Use `js_namespace` here to bind `console.log(..)` instead of just
	// `log(..)`
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);

	// The `console.log` is quite polymorphic, so we can bind it with multiple
	// signatures. Note that we need to use `js_name` to ensure we always call
	// `log` in JS.
	#[wasm_bindgen(js_namespace = console, js_name = log)]
	fn log_u32(a: u32);

	// Multiple arguments too!
	#[wasm_bindgen(js_namespace = console, js_name = log)]
	fn log_many(a: &str, b: &str);

	fn alert(s: &str);
}

// Next let's define a macro that's like `println!`, only it works for
// `console.log`. Note that `println!` doesn't actually work on the wasm target
// because the standard library currently just eats all output. To get
// `println!`-like behavior in your app you'll likely want a macro like this.

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

fn get_document() -> Result<Document> {
	let window = web_sys::window().unwrap();
	Ok(window.document().unwrap())
}

/// Entry point.
#[wasm_bindgen]
pub fn run() -> Result<()> {
	let document = get_document()?;
	update_canvas(&document)?;
	update_svg_mls(&document)?;
	update_svg_polygon(&document)?;
	update_svg_multipolygon(&document)?;
	Ok(())
}

// draw dot
fn update_canvas(document: &Document) -> Result<()> {
	// grab canvas
	let canvas = document
		.get_element_by_id("c")
		.unwrap()
		.dyn_into::<web_sys::HtmlCanvasElement>()?;

	let context_raw = canvas
		.get_context("2d")?
		.unwrap()
		.dyn_into::<web_sys::CanvasRenderingContext2d>()?;

	let width = canvas.width().into();
	let height = canvas.height().into();
	context_raw.set_fill_style(&"black".into());
	context_raw.set_stroke_style(&"black".into());
	context_raw.fill_rect(0.0, 0.0, width, height);

	let context: Context<f64> = Context::new(&context_raw);
	let pb = PathBuilder::new(context);

	let ortho_builder = Orthographic::builder()
		.scale(240_f64)
		.rotate(&[0_f64, -20_f64, 0_f64])
		.translate(&Coordinate {
			x: width / 2_f64,
			y: height / 2_f64,
		}).clip_angle_adjust(90_f64 + 1e-6);

	let ortho = ortho_builder.build();

	let mut path = pb.build(ortho);

	let lines = generate_graticule().lines();

	let mls = Geometry::MultiLineString(MultiLineString(lines.collect()));
	context_raw.begin_path();
	context_raw.set_fill_style(&"#999".into());
	context_raw.set_stroke_style(&"#69b3a2".into());
	path.object(&mls);

	context_raw.stroke();

	Ok(())
}

#[cfg(not(tarpaulin_include))]
fn get_path_node(class_name: &str) -> Result<Element> {
	let document = get_document()?;
	// let class_name = format!("id-{}", i);
	let class_list = document.get_elements_by_class_name(class_name);

	assert!(class_list.length() < 2);
	// console_log!("assert passed.");
	let ret = match class_list.item(0) {
		Some(element) => element,
		None => {
			// keep.
			match document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path") {
				Ok(element) => element,
				Err(_) => {
					console_log!("failed to create node.");
					panic!("failed");
				}
			}
		}
	};
	Ok(ret)
}

fn update_svg_mls(document: &Document) -> Result<()> {
	// Grab canvas.
	let svg: SvgsvgElement = document
		.get_element_by_id("s_mls")
		.unwrap()
		.dyn_into::<web_sys::SvgsvgElement>()?;

	let width = svg.width().base_val().value()? as f64;
	let height = svg.height().base_val().value()? as f64;

	let ortho_builder = Orthographic::builder();

	let stroke: [&str; 7] = [
		"stroke: red",
		"stroke: orange",
		"stroke: yellow",
		"stroke: green",
		"stroke: blue",
		"stroke: indigo",
		"stroke: black",
	];
	console_log!("Have builder");
	// for angle in 0 {
	// TODO Code small ortho_builder.clone() can reuse this object as expected.
	let ortho = ortho_builder
		.scale(240_f64)
		.translate(&Coordinate {
			x: width / 2_f64,
			y: height / 2_f64,
		})
		.rotate(&[0_f64, -20_f64, 0_f64])
		.build();
	let mut pb = PathBuilder::context_pathstring().build(ortho);

	let lines: Vec<LineString<f64>> = generate_graticule::<f64>().lines().collect();

	let mls = Geometry::MultiLineString(MultiLineString(lines));

	let s = pb.object(&mls);
	let i = 1;
	let class_name = format!("id-{}", i);
	let path = get_path_node(&class_name)?;
	path.set_attribute_ns(None, "d", &s)?;
	path.set_attribute_ns(None, "style", stroke[i])?;
	svg.append_child(&path)?;

	Ok(())
}

fn update_svg_polygon(document: &Document) -> Result<()> {
	// Grab canvas.
	let svg: SvgsvgElement = document
		.get_element_by_id("s_polygon")
		.unwrap()
		.dyn_into::<web_sys::SvgsvgElement>()?;

	let width = svg.width().base_val().value()? as f64;
	let height = svg.height().base_val().value()? as f64;

	let stroke: [&str; 7] = [
		"stroke: red",
		"stroke: orange",
		"stroke: yellow",
		"stroke: green",
		"stroke: blue",
		"stroke: indigo",
		"stroke: black",
	];
	console_log!("Have builder");
	// for angle in 0 {
	// TODO Code small ortho_builder.clone() can reuse this object as expected.
	let ortho = Orthographic::builder()
		.scale(240_f64)
		.translate(&Coordinate {
			x: width / 2_f64,
			y: height / 2_f64,
		})
		.rotate(&[0_f64, -20_f64, 0_f64])
		.build();
	let mut pb = PathBuilder::context_pathstring().build(ortho);

	let lines = generate_graticule::<f64>().lines();
	for (i, l) in lines.enumerate() {
		let pdo = Geometry::Polygon(Polygon::new(l.clone(), vec![]));

		let s = pb.object(&pdo);

		if s == "EMPTY" {
			// TODO differs from MultiLineString?
			// console_log!("polygon dropping {}", i);
		} else {
			let class_name = format!("s2-id-{}", i);
			let path = get_path_node(&class_name)?;
			path.set_attribute_ns(None, "d", &s)?;
			path.set_attribute_ns(None, "style", stroke[i % 7])?;
			svg.append_child(&path)?;
		}
	}

	Ok(())
}

fn update_svg_multipolygon(document: &Document) -> Result<()> {
	// Grab canvas.
	console_log!("looking");
	let svg: SvgsvgElement = document
		.get_element_by_id("s_multipolygon")
		.unwrap()
		.dyn_into::<web_sys::SvgsvgElement>()?;
	console_log!("got mp");
	let width = svg.width().base_val().value()? as f64;
	let height = svg.height().base_val().value()? as f64;

	let stroke: [&str; 7] = [
		"stroke: red",
		"stroke: orange",
		"stroke: yellow",
		"stroke: green",
		"stroke: blue",
		"stroke: indigo",
		"stroke: black",
	];
	console_log!("Have builder");
	// for angle in 0 {
	// TODO Code small ortho_builder.clone() can reuse this object as expected.
	let ortho = Orthographic::<_, f64>::builder()
		.scale(240_f64)
		.translate(&Coordinate {
			x: width / 2_f64,
			y: height / 2_f64,
		})
		.rotate(&[0_f64, -20_f64, 0_f64])
		.build();
	let mut pb = PathBuilder::context_pathstring().build(ortho);

	let lines = generate_graticule::<f64>().lines();
	let mut polygon_vec = Vec::new();
	for l in lines {
		polygon_vec.push(Polygon::new(l.clone(), vec![]))
	}

	let pdo = Geometry::MultiPolygon(MultiPolygon(polygon_vec));

	let s = pb.object(&pdo);
	if s == "EMPTY" {
		// TODO differs from MultiLineString?
		console_log!("polygon dropping {}", 0);
	} else {
		console_log!("pass");
		let class_name = format!("s2-id-{}", 0);
		let path = get_path_node(&class_name)?;
		path.set_attribute_ns(None, "d", &s)?;
		path.set_attribute_ns(None, "style", stroke[0 % 7])?;
		svg.append_child(&path)?;
	}

	Ok(())
}
