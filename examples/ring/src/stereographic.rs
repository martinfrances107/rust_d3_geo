extern crate rand;
extern crate web_sys;

use geo::LineString;
use geo::MultiPolygon;
use geo::Polygon;
use geo_types::Coord;
use wasm_bindgen::JsCast;
use web_sys::SvgsvgElement;

use d3_geo_rs::circle::generator::Generator as CircleGenerator;
use d3_geo_rs::path::builder::Builder as PathBuilder;
use d3_geo_rs::projection::stereographic::Stereographic;
use d3_geo_rs::projection::Build;
use d3_geo_rs::projection::RawBase;
use d3_geo_rs::projection::TranslateSet;

use super::document;
use super::path_node;
use super::Result;

pub fn draw_sterographic() -> Result<()> {
    let svg: SvgsvgElement = document()?
        .get_element_by_id("ring_stereographic_rust")
        .unwrap()
        .dyn_into::<web_sys::SvgsvgElement>()?;

    let width = svg.width().base_val().value()? as f64;
    let height = svg.height().base_val().value()? as f64;

    let stereographic = Stereographic::<_, f64>::builder()
        .translate_set(&Coord {
            x: width / 2_f64,
            y: height / 2_f64,
        })
        .build();

    let mut cg_outer = CircleGenerator::default();
    cg_outer.radius_set(10_f64).precision_set(10_f64);

    let mut cg_inner = CircleGenerator::default();
    cg_inner.radius_set(5_f64).precision_set(5_f64);

    let mut p_vec: Vec<Polygon<f64>> = vec![];
    for lat in (-30..=30).step_by(30) {
        for long in (-180..=180).step_by(40) {
            let mut inner = cg_inner
                .clone()
                .center_set(&Coord {
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
                    .center_set(&Coord {
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

    let mut path = PathBuilder::context_pathstring().build(stereographic);
    let s = path.object(&object);

    let class_name = format!("s2-id-{}", 0);
    let path = path_node(&class_name)?;
    path.set_attribute_ns(None, "d", &s)?;
    svg.append_child(&path)?;

    Ok(())
}
