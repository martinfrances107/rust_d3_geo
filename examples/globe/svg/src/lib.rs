#![allow(clippy::pedantic)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![cfg(not(tarpaulin_include))]

//! # rust d3 geo
//!
//! See the README.md.

extern crate js_sys;
extern crate rust_topojson_client;
extern crate topojson;
extern crate web_sys;

use d3_geo_rs::graticule::generate_mls;
use d3_geo_rs::projection::builder::template::NoPCNC;
use geo::Geometry;
use geo::GeometryCollection;
use geo_types::Coord;
use gloo_utils::format::JsValueSerdeExt;
use topojson::Topology;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::console_log;
use web_sys::Document;
use web_sys::SvgsvgElement;
use web_sys::*;

use d3_geo_rs::path::builder::Builder as PathBuilder;
use d3_geo_rs::path::string::String;
use d3_geo_rs::projection::orthographic::Orthographic;
use d3_geo_rs::projection::Build;
use d3_geo_rs::projection::RawBase;
use d3_geo_rs::projection::RotateSet;
use d3_geo_rs::projection::ScaleSet;
use d3_geo_rs::projection::TranslateSet;
use rust_topojson_client::feature::feature_from_name;

#[cfg(not(tarpaulin_include))]
fn document() -> Result<Document, JsValue> {
    let window = web_sys::window().unwrap();
    Ok(window.document().unwrap())
}

#[cfg(not(tarpaulin_include))]
fn path_node(class_name: &str) -> Result<Element, JsValue> {
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

/// Entry point.
#[wasm_bindgen]
pub async fn start() -> Result<(), JsValue> {
    let document = document()?;
    let window = web_sys::window().expect("Failed to get window");

    // Get data from world map.
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init("/world-atlas/world/50m.json", &opts)?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json()?).await?;

    let topology =
        JsValueSerdeExt::into_serde::<Topology>(&json).expect("Did not get a valid Topology");

    // Grab canvas.
    let svg: SvgsvgElement = document
        .get_element_by_id("s")
        .unwrap()
        .dyn_into::<web_sys::SvgsvgElement>()?;

    let width = svg.width().base_val().value()? as f64;
    let height = svg.height().base_val().value()? as f64;

    let countries = feature_from_name(&topology, "countries").expect("Did not extract geometry");

    let ortho = Orthographic::builder()
        .scale_set(width / 1.3_f64 / std::f64::consts::PI)
        .translate_set(&Coord {
            x: width / 2_f64,
            y: height / 2_f64,
        })
        .rotate2_set(&[270_f64, 0_f64])
        .build();

    let fill: [&str; 7] = [
        "fill: red",
        "fill: orange",
        "fill: olive",
        "fill: blue",
        "fill: indigo",
        "fill: brown",
        "fill: silver",
    ];

    let pb: PathBuilder<_, _, _, NoPCNC<String<f64>>, _, _, _, _, _> =
        PathBuilder::context_pathstring();

    let mut builder = pb.build(ortho);
    let mut i = 0;

    match &countries {
        Geometry::GeometryCollection(GeometryCollection(g_vec)) => {
            for g in g_vec {
                match &g {
                    Geometry::MultiPolygon(mp) => {
                        i += 1;
                        for p in &mp.0 {
                            let s = builder.object(&Geometry::Polygon(p.clone()));
                            let class_name = format!("id-{i}");
                            let path = path_node(&class_name)?;
                            path.set_attribute_ns(None, "d", &s)?;
                            path.set_attribute_ns(None, "class", &class_name)?;
                            path.set_attribute_ns(None, "style", fill[i % 7])?;
                            svg.append_child(&path)?;
                            i += 1
                        }
                    }
                    Geometry::Polygon(p) => {
                        let s = builder.object(&Geometry::Polygon(p.clone()));

                        let class_name = format!("id-{i}");
                        let path = path_node(&class_name)?;
                        path.set_attribute_ns(None, "d", &s)?;
                        path.set_attribute_ns(None, "style", fill[i % 7])?;
                        svg.append_child(&path)?;
                        i += 1
                    }

                    _ => {
                        console_log!("Not polygon, Not Multipolygon.");
                    }
                }
            }
        }
        _ => {
            console_log!("Not a geometry collection.")
        }
    }

    // Graticule
    let graticule = generate_mls();

    let graticule_d = builder.object(&graticule);
    let class_name = "graticule";
    let path = path_node(class_name)?;
    path.set_attribute_ns(None, "d", &graticule_d)?;
    path.set_attribute_ns(None, "style", "#ccc")?;
    svg.append_child(&path)?;

    Ok(())
}
