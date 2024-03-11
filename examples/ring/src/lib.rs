#![deny(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::perf)]
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

use orthographic::draw as draw_orthographic;
use stereographic::draw as draw_stereographic;

mod orthographic;
mod stereographic;

type Result<T> = core::result::Result<T, JsValue>;

fn document() -> Result<Document> {
    let window = web_sys::window().ok_or("no window")?;
    Ok(window.document().ok_or("no document")?)
}

/// Entry point.
///
/// # Errors
///
/// When a drawing calls fail.
///
#[wasm_bindgen]
pub fn run() -> Result<()> {
    draw_orthographic()?;
    draw_stereographic()?;
    Ok(())
}

/// # Errors
///
/// When the creation of the SVG element fails.
#[cfg(not(tarpaulin_include))]
fn path_node(class_name: &str) -> Result<Element> {
    let document = document()?;

    let class_list = document.get_elements_by_class_name(class_name);

    assert!(class_list.length() < 2);
    let ret = class_list.item(0).map_or_else(
        || {
            document
                .create_element_ns(Some("http://www.w3.org/2000/svg"), "path").unwrap_or_else(|_| {
                        console_log!("failed to create node.");
                        panic!("failed");
                    })
        },
        |element| element,
    );
    Ok(ret)
}
