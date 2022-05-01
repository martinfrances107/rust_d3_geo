#![allow(clippy::pedantic)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
//! # rust d3 geo
//!
//! See the README.md.

extern crate js_sys;
extern crate rand;
extern crate rust_topojson_client;
extern crate topojson;
extern crate web_sys;

use geo::Coordinate;
use geo::Geometry;
use geo::GeometryCollection;
use rust_d3_geo::projection::RotateSet;
use rust_d3_geo::projection::TranslateAdjust;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::Document;
use web_sys::SvgsvgElement;
use web_sys::*;
mod dom_macros;

use rust_d3_geo::path::builder::Builder as PathBuilder;
use rust_d3_geo::projection::orthographic::Orthographic;
use rust_d3_geo::projection::ProjectionRawBase;
use rust_d3_geo::projection::ScaleAdjust;
use rust_topojson_client::feature::Builder as FeatureBuilder;

use topojson::Topology;

#[wasm_bindgen]
#[cfg(not(tarpaulin_include))]
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

#[cfg(not(tarpaulin_include))]
fn get_document() -> Result<Document, JsValue> {
    let window = web_sys::window().unwrap();
    Ok(window.document().unwrap())
}

#[cfg(not(tarpaulin_include))]
fn get_path_node(class_name: &str) -> Result<Element, JsValue> {
    let document = get_document()?;
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
    let document = get_document()?;
    let window = web_sys::window().expect("Failed to get window");

    // Get data from world map.
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init("/world-atlas/world/50m.json", &opts)?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json()?).await?;

    let topology: Topology = json.into_serde().expect("could not parse as Topology");

    // Grab canvas.
    let svg: SvgsvgElement = document
        .get_element_by_id("s")
        .unwrap()
        .dyn_into::<web_sys::SvgsvgElement>()?;

    let width = svg.width().base_val().value()? as f64;
    let height = svg.height().base_val().value()? as f64;

    let countries = FeatureBuilder::generate_from_name(&topology, "countries")
        .expect("Did not extract geometry");

    let ortho = Orthographic::builder()
        .scale(width as f64 / 1.3_f64 / std::f64::consts::PI)
        .translate(&Coordinate {
            x: width / 2_f64,
            y: height / 2_f64,
        })
        .rotate(&[270_f64, 0_f64, 0_f64])
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

    let mut builder = PathBuilder::context_pathstring().build(ortho);
    let mut i = 0;

    match &countries {
        Geometry::GeometryCollection(GeometryCollection(g_vec)) => {
            console_log!("{}", g_vec.len());
            for g in g_vec {
                match &g {
                    Geometry::MultiPolygon(mp) => {
                        i += 1;
                        for p in &mp.0 {
                            let s = builder.object(&Geometry::Polygon(p.clone()));
                            let class_name = format!("id-{}", i);
                            let path = get_path_node(&class_name)?;
                            path.set_attribute_ns(None, "d", &s)?;
                            path.set_attribute_ns(None, "class", &class_name)?;
                            path.set_attribute_ns(None, "style", fill[i % 7])?;
                            svg.append_child(&path)?;
                            i += 1
                        }
                    }
                    Geometry::Polygon(p) => {
                        let s = builder.object(&Geometry::Polygon(p.clone()));

                        let class_name = format!("id-{}", i);
                        let path = get_path_node(&class_name)?;
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

    Ok(())
}
