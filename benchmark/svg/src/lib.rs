#![allow(clippy::pedantic)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
//! # rust d3 geo voronoi
//!
//! Know bugs.
//!
//! When I convert this benchmark to run on f32's
//! The polygons are mis-shaped
//!
//! See the README.md.
extern crate js_sys;
extern crate rand;
extern crate rust_topojson_client;
extern crate topojson;
extern crate web_sys;

use geo::polygon;
use wasm_bindgen::prelude::*;

use geo::Coordinate;
use geo::Geometry;
use geo::GeometryCollection;
use geo::MultiPolygon;
use geo::Polygon;
use rust_topojson_client::feature::Builder as FeatureBuilder;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::Document;
use web_sys::SvgsvgElement;
use web_sys::*;

use rust_d3_geo::path::builder::Builder as PathBuilder;

use rust_d3_geo::path::string::String as PathString;

use rust_d3_geo::projection::orthographic::Orthographic;
use rust_d3_geo::projection::Raw;
use rust_d3_geo::projection::Scale;
use rust_d3_geo::projection::Translate;

use rust_d3_geo::projection::Rotate;

use topojson::Topology;

mod dom_macros;

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
    // let class_name = format!("id-{}", i);
    let class_list = document.get_elements_by_class_name(&class_name);

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

/// Entry point.
#[wasm_bindgen]
pub async fn start(p_vec_in: JsValue, mp_vec_in: JsValue) -> Result<(), JsValue> {
    let p_vec: Vec<Polygon<f64>>;
    match p_vec_in.into_serde::<Vec<Polygon<f64>>>() {
        Ok(v) => {
            console_log!("polygon - ok {}", v.len());
            // console_log!("{:?}", v);
            p_vec = v;
        }
        Err(e) => {
            console_log!("fail to decode {}", e);
            // console_log!("{:?}", p_vec_in);
            return Ok(());
        }
    }

    let mp_vec: Vec<Polygon<f64>>;
    match mp_vec_in.into_serde::<Vec<Polygon<f64>>>() {
        Ok(v) => {
            console_log!("polygon - ok {}", v.len());
            // console_log!("{:?}", v);
            mp_vec = v;
        }
        Err(e) => {
            console_log!("fail to decode {}", e);
            console_log!("{:?}", mp_vec_in);
            return Ok(());
        }
    }

    // console_log!("run() - wasm entry point");
    let document = get_document()?;

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init("/world-atlas/world/50m.json", &opts)?;
    // let request = Request::new_with_str_and_init("/world-atlas/africa.json", &opts)?;

    // request.headers().set("Accept", "application/json")?;

    let window = web_sys::window().expect("Failed to get window");
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    // assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let json = JsFuture::from(resp.json()?).await?;

    // for f in data.values("features").iter() {}

    let topology: Topology = json.into_serde().expect("could not parse as Topology");
    console_log!("Have topology");
    // Grab canvas.
    let svg: SvgsvgElement = document
        .get_element_by_id("s")
        .unwrap()
        .dyn_into::<web_sys::SvgsvgElement>()?;

    let width = svg.width().base_val().value()? as f64;
    let height = svg.height().base_val().value()? as f64;

    let countries = FeatureBuilder::generate_from_name(&topology, "countries")
        .expect("Did not extract geometry");

    console_log!("Have countries");
    let ortho_builder = Orthographic::<PathString<f64>, f64>::builder()
        .scale(width as f64 / 1.3_f64 / std::f64::consts::PI)
        .translate(&Coordinate {
            x: width / 2_f64,
            y: height / 2_f64,
        })
        .rotate(&[0_f64, 00_f64, 0_f64]);

    let ortho = ortho_builder.build();

    // let builder = PathBuilder::context_pathstring();

    // let path_d = builder.build(ortho.clone()).object(&countries);
    // TODO Code small ortho_builder.clone() can reuse this object as expected.
    // console_log!("{:?}", &JsValue::from(path_d.clone()));
    // if let Ok(path) = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path") {
    //     path.set_attribute_ns(None, "d", &path_d)?;
    //     svg.append_child(&path)?;
    // };

    let stroke: [&str; 6] = [
        "stroke: red",
        "stroke: orange",
        "stroke: green",
        "stroke: blue",
        "stroke: indigo",
        "stroke: black",
    ];
    console_log!("Have builder");

    let mut builder = PathBuilder::context_pathstring().build(ortho);
    let mut i = 0;
    // for p in p_vec {
    //     // console_log!("p {:?}", p);
    //     let s = builder.object(&p);
    //     let class_name = format!("id-{}", i);
    //     let path = get_path_node(&class_name)?;
    //     path.set_attribute_ns(None, "d", &s)?;
    //     // console_log!("setting  (p) attr1");
    //     path.set_attribute_ns(None, "style", stroke[i % 5])?;
    //     // console_log!("setting (p) attr2");
    //     svg.append_child(&path)?;
    //     // console_log!("setting (p) attr3");
    //     i = i + 1;
    // }

    // for mp in mp_vec {
    //     // console_log!("p {:?}", p);
    //     let s = builder.object(&mp);
    //     let class_name = format!("id-{}", i);
    //     let path = get_path_node(&class_name)?;
    //     path.set_attribute_ns(None, "d", &s)?;
    //     // console_log!("setting  (p) attr1");
    //     path.set_attribute_ns(None, "style", stroke[i % 5])?;
    //     // console_log!("setting (p) attr2");
    //     svg.append_child(&path)?;
    //     // console_log!("setting (p) attr3");
    //     i = i + 1
    // }

    match &countries {
        Geometry::GeometryCollection(GeometryCollection(g_vec)) => {
            console_log!("{}", g_vec.len());
            for g in g_vec {
                match &g {
                    Geometry::MultiPolygon(mp) => {
                        console_log!("Have multipolygon");
                        for p in &mp.0 {
                            // console_log!("mp : poly..");
                            let s = builder.object(&Geometry::Polygon(p.clone()));

                            if s == "EMPTY" {
                                console_log!("{}", s);
                                console_log!("{:?}", p.clone());
                            }
                            // console_log!("Have res(mp)");
                            let class_name = format!("id-{}", i);
                            let path = get_path_node(&class_name)?;
                            path.set_attribute_ns(None, "d", &s)?;
                            // console_log!("setting attr1");
                            path.set_attribute_ns(None, "class", &class_name)?;
                            // console_log!("setting attr2");
                            path.set_attribute_ns(None, "style", stroke[i % 6])?;
                            // console_log!("setting attr3");
                            svg.append_child(&path)?;
                            // console_log!("svg udpated");
                            i = i + 1
                        }
                    }
                    Geometry::Polygon(p) => {
                        console_log!("polygon");
                        let s = builder.object(&Geometry::Polygon(p.clone()));
                        // match res {
                        // Some(res) => match res {
                        // let s = res;
                        // console_log!("Have res(p) ");

                        let class_name = format!("id-{}", i);
                        let path = get_path_node(&class_name)?;
                        path.set_attribute_ns(None, "d", &s)?;
                        // console_log!("setting  (p) attr1");
                        path.set_attribute_ns(None, "style", stroke[i % 6])?;
                        // console_log!("setting (p) attr2");
                        svg.append_child(&path)?;
                        // console_log!("setting (p) attr3");
                        i = i + 1
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
