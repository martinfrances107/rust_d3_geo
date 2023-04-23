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

use d3_geo_rs::projection::projector_albers_usa::AlbersUsaMultiplex;
use geo::Geometry;
use geo::GeometryCollection;
use gloo_utils::format::JsValueSerdeExt;
use rust_topojson_client::feature::feature_from_name;
use topojson::Topology;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::console_log;
use web_sys::Document;
use web_sys::SvgsvgElement;
use web_sys::*;

use d3_geo_rs::graticule::generate_mls;
use d3_geo_rs::path::builder::Builder as PathBuilder;
use d3_geo_rs::path::string::String as PathString;
use d3_geo_rs::projection::albers_usa::AlbersUsa;
use d3_geo_rs::projection::projector_albers_usa::AlbersUsaMultiTransformer;
use d3_geo_rs::projection::projector_albers_usa::Projector as ProjectorAlbersUsa;
use d3_geo_rs::projection::RawBase;

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

type P_A_USA<const N: usize> =
    ProjectorAlbersUsa<PathString<f64>, AlbersUsaMultiTransformer<PathString<f64>, f64>>;

type MyBuilder = PathBuilder<PathString<f64>, P_A_USA<3>, f64>;

use d3_geo_rs::path::builder::StringMultidrian;
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

    let countries: Geometry<f64> =
        feature_from_name(&topology, "countries").expect("Did not extract geometry");

    let projector = AlbersUsa::<PathString<f64>, f64>::builder().build();

    let fill: [&str; 7] = [
        "fill: red",
        "fill: orange",
        "fill: olive",
        "fill: blue",
        "fill: indigo",
        "fill: brown",
        "fill: silver",
    ];

    let pb = PathBuilder::<_, _, f64>::albers_pathstring();
    // let mut path = pb.build(projector);

    let mut i = 0;

    match &countries {
        Geometry::GeometryCollection(GeometryCollection(g_vec)) => {
            for g in g_vec {
                match &g {
                    Geometry::MultiPolygon(mp) => {
                        i += 1;
                        for p in &mp.0 {
                            // let s = path.object(&Geometry::Polygon(p.clone()));
                            // let class_name = format!("id-{i}");
                            // let path = path_node(&class_name)?;
                            // path.set_attribute_ns(None, "d", &s)?;
                            // path.set_attribute_ns(None, "class", &class_name)?;
                            // path.set_attribute_ns(None, "style", fill[i % 7])?;
                            // svg.append_child(&path)?;
                            // i += 1
                        }
                    }
                    Geometry::Polygon(p) => {
                        // let s = path.object(&Geometry::Polygon(p.clone())).result();

                        // let class_name = format!("id-{i}");
                        // let path = path_node(&class_name)?;
                        // path.set_attribute_ns(None, "d", &s)?;
                        // path.set_attribute_ns(None, "style", fill[i % 7])?;
                        // svg.append_child(&path)?;
                        // i += 1
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

    // let graticule_d = builder.object(&graticule).result();
    // let class_name = "graticule";
    // let path = path_node(class_name)?;
    // path.set_attribute_ns(None, "d", &graticule_d)?;
    // path.set_attribute_ns(None, "style", "#ccc")?;
    // svg.append_child(&path)?;

    Ok(())
}
