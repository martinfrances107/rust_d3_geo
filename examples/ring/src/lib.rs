#![allow(clippy::pedantic)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![cfg(not(tarpaulin_include))]

//! # rust d3 geo rings
//!
//! See the README.md.
extern crate web_sys;

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::console_log;
use web_sys::Document;
use web_sys::Element;

use orthographic::draw_orthographic;
use stereographic::draw_stereographic;

mod orthographic;
mod stereographic;

type Result<T> = std::result::Result<T, JsValue>;

fn document() -> Result<Document> {
    let window = web_sys::window().unwrap();
    Ok(window.document().unwrap())
}

/// Entry point.
#[wasm_bindgen]
pub fn run() -> Result<()> {
    draw_orthographic()?;
    draw_stereographic()?;
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
