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
use geo::LineString;
use geo::MultiPolygon;
use geo::Polygon;

use rust_d3_geo::projection::Scale;
use rust_d3_geo::projection::Translate;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Document;
use web_sys::Element;

use rust_d3_geo::circle::generator::Generator as CircleGenerator;

use rust_d3_geo::path::builder::Builder as PathBuilder;
use rust_d3_geo::projection::orthographic::Orthographic;
use rust_d3_geo::projection::Raw;
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
    draw()
}

fn draw() -> Result<()> {
    let svg: SvgsvgElement = get_document()?
        .get_element_by_id("ring_rust")
        .unwrap()
        .dyn_into::<web_sys::SvgsvgElement>()?;

    let width = svg.width().base_val().value()? as f64;
    let height = svg.height().base_val().value()? as f64;

    let ortho_builder = Orthographic::<_, f64>::builder()
        .scale(240_f64)
        .translate(&Coordinate {
            x: width / 2_f64,
            y: height / 2_f64,
        });

    let ortho = ortho_builder.build();

    let cg_outer = CircleGenerator::default().radius(10_f64).precision(10_f64);
    let cg_inner = CircleGenerator::default().radius(5_f64).precision(5_f64);

    let mut p_vec: Vec<Polygon<f64>> = vec![];
    for lat in (-30..=30).step_by(30) {
        for long in (-180..=180).step_by(40) {
            let mut inner = cg_inner
                .clone()
                .center(&Coordinate {
                    x: long as f64,
                    y: lat as f64,
                })
                .circle()
                .exterior()
                .0
                .clone();
            inner.reverse();
            let inner_ring: LineString<f64> = inner.into();

            let poly = Polygon::new(
                cg_outer
                    .clone()
                    .center(&Coordinate {
                        x: long as f64,
                        y: lat as f64,
                    })
                    .circle()
                    .exterior()
                    .clone(),
                vec![inner_ring],
            );

            p_vec.push(poly);
        }
    }

    let object = MultiPolygon(p_vec);

    let mut path = PathBuilder::context_pathstring().build(ortho);
    console_log!("{:?}", object);
    let s = path.object(&object);

    let class_name = format!("s2-id-{}", 0);
    let path = get_path_node(&class_name)?;
    path.set_attribute_ns(None, "d", &s)?;
    svg.append_child(&path)?;

    Ok(())
}

#[cfg(not(tarpaulin_include))]
fn get_path_node(class_name: &str) -> Result<Element> {
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
