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
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::console_log;
use web_sys::Document;
use web_sys::Element;
use web_sys::SvgsvgElement;

use rust_d3_geo::graticule::generate as generate_graticule;
use rust_d3_geo::path::builder::Builder as PathBuilder;
use rust_d3_geo::path::context::Context;
use rust_d3_geo::projection::orthographic::Orthographic;
use rust_d3_geo::projection::Build;
use rust_d3_geo::projection::ClipAngleAdjust;
use rust_d3_geo::projection::ProjectionRawBase;
use rust_d3_geo::projection::RotateSet;
use rust_d3_geo::projection::ScaleSet;
use rust_d3_geo::projection::TranslateSet;

type Result<T> = std::result::Result<T, JsValue>;

fn document() -> Result<Document> {
    let window = web_sys::window().unwrap();
    Ok(window.document().unwrap())
}

/// Entry point.
#[wasm_bindgen]
pub fn run() -> Result<()> {
    let document = document()?;
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

    let context: Context = Context::new(context_raw.clone());
    let pb = PathBuilder::new(context);

    let ortho = Orthographic::builder()
        .scale_set(240_f64)
        .rotate_set(&[0_f64, -20_f64, 0_f64])
        .translate_set(&Coordinate {
            x: width / 2_f64,
            y: height / 2_f64,
        })
        .clip_angle(90_f64 + 1e-6)
        .build();

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
fn path_node(class_name: &str) -> Result<Element> {
    let document = document()?;
    let class_list = document.get_elements_by_class_name(class_name);

    assert!(class_list.length() < 2);
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
    let ortho = Orthographic::builder()
        .scale_set(240_f64)
        .translate_set(&Coordinate {
            x: width / 2_f64,
            y: height / 2_f64,
        })
        .rotate_set(&[0_f64, -20_f64, 0_f64])
        .build();

    let mut pb = PathBuilder::context_pathstring().build(ortho);

    let lines: Vec<LineString<f64>> = generate_graticule::<f64>().lines().collect();

    let mls = Geometry::MultiLineString(MultiLineString(lines));

    let s = pb.object(&mls);
    let i = 1;
    let class_name = format!("id-{}", i);
    let path = path_node(&class_name)?;
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
    let ortho = Orthographic::builder()
        .scale_set(240_f64)
        .translate_set(&Coordinate {
            x: width / 2_f64,
            y: height / 2_f64,
        })
        .rotate_set(&[0_f64, -20_f64, 0_f64])
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
            let path = path_node(&class_name)?;
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
    let ortho = Orthographic::builder()
        .scale_set(240_f64)
        .translate_set(&Coordinate {
            x: width / 2_f64,
            y: height / 2_f64,
        })
        .rotate_set(&[0_f64, -20_f64, 0_f64])
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
        let path = path_node(&class_name)?;
        path.set_attribute_ns(None, "d", &s)?;
        path.set_attribute_ns(None, "style", stroke[0 % 7])?;
        svg.append_child(&path)?;
    }

    Ok(())
}
