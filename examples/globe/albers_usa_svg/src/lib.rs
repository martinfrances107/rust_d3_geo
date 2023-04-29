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

use d3_geo_rs::path::Result;
use d3_geo_rs::projection::projector_albers_usa::multidrain::Multidrain;
use d3_geo_rs::projection::projector_albers_usa::multidrain::Unpopulated;
use d3_geo_rs::projection::projector_albers_usa::multiplex::Multiplex;
use d3_geo_rs::projection::projector_albers_usa::AlbersUsaMultiplex;
use d3_geo_rs::projection::Projector;
use d3_geo_rs::stream::Stream;
use d3_geo_rs::stream::Streamable;
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
fn document() -> Option<Document> {
    let window = web_sys::window().unwrap();
    window.document()
}

#[cfg(not(tarpaulin_include))]
fn path_node(class_name: &str) -> Element {
    let document = document().unwrap();
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
    ret
}

// type PA = ProjectorAlbersUsa<Multiplex<AlbersUsa<PathString<f64>, f64>, _, f64>, PathString<f64>>;

// type MyBuilder = PathBuilder<PathString<f64>, P_A_USA<3>, f64>;

// use d3_geo_rs::path::builder::StringMultidrian;
/// Entry point.
#[wasm_bindgen]
pub async fn start() {
    let document = document().unwrap();
    let window = web_sys::window().expect("Failed to get window");

    // Get data from world map.
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init("/world-atlas/world/50m.json", &opts)
        .expect("requets failed.");
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .expect("await failed");
    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json().expect("resp failed"))
        .await
        .expect("json failed");

    let topology =
        JsValueSerdeExt::into_serde::<Topology>(&json).expect("Did not get a valid Topology");

    // Grab canvas.
    let svg: SvgsvgElement = document
        .get_element_by_id("s")
        .unwrap()
        .dyn_into::<web_sys::SvgsvgElement>()
        .expect("svg failed");

    let width = svg.width().base_val().value().expect("width failed") as f64;
    let height = svg.height().base_val().value().expect("height failed") as f64;

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

    let pb = PathBuilder::albers_pathstring();
    let mut path = pb.build(projector);

    let mut i = 0;

    match &countries {
        Geometry::GeometryCollection(GeometryCollection(g_vec)) => {
            for g in g_vec {
                match &g {
                    Geometry::MultiPolygon(mp) => {
                        i += 1;
                        for (j, p) in mp.0.iter().enumerate() {
                            // let s = path.object(&Geometry::Polygon(p.clone()));

                            let mut stream_in = path.projection.stream(&path.context_stream);
                            let object = Geometry::Polygon(p.clone());
                            object.to_stream(&mut stream_in);
                            // let s = stream_in.endpoint().result();
                            for (k, s) in stream_in.endpoint().result().iter().enumerate() {
                                let class_name = format!("id-{i}-{j}-{k}");
                                let path = path_node(&class_name);
                                path.set_attribute_ns(None, "d", s).expect("none 2");
                                path.set_attribute_ns(None, "class", &class_name)
                                    .expect("class failed");
                                path.set_attribute_ns(None, "style", fill[i % 7])
                                    .expect("none failed");
                                svg.append_child(&path).expect("append failed.");
                            }
                            i += 1
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
}
