#![cfg(not(tarpaulin_include))]

use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;
use std::io::LineWriter;

use geo::CoordFloat;
use geo::Geometry;
use geo_types::Coord;
use num_traits::FloatConst;
use rust_topojson_client::feature::feature_from_name;
use topojson::Topology;

use d3_geo_rs::graticule::generate_mls;
use d3_geo_rs::path::builder::Builder as PathBuilder;
use d3_geo_rs::projection::orthographic::Orthographic;
use d3_geo_rs::projection::Build;
use d3_geo_rs::projection::RawBase as ProjectionRawBase;
use d3_geo_rs::projection::RotateSet;

#[macro_use]
extern crate lazy_static;

#[cfg(not(tarpaulin_include))]
lazy_static! {
    static ref SCHEME_CATEGORY10: [String; 10] = [
        String::from("#1f77b4"),
        String::from("#ff7f0e"),
        String::from("#2ca02c"),
        String::from("#d62728"),
        String::from("#9467bd"),
        String::from("#8c564b"),
        String::from("#e377c2"),
        String::from("#7f7f7f"),
        String::from("#bcbd22"),
        String::from("#17becf"),
    ];
}

///  Helper function to extract world geometry from file.
fn world<T>() -> Topology
where
    T: CoordFloat + Debug + FloatConst,
{
    let file = File::open("./world-atlas/world/50m.json").expect("File should open read only.");
    let topology: Topology = serde_json::from_reader(file).expect("File should be parse as JSON.");

    // if let Some(g) = feature_from_name(&topology, "land") {
    //     g
    // } else {
    //     panic!("failed to file and decode from file.");
    // }
    topology
}

#[cfg(not(tarpaulin_include))]
fn draw() -> Result<Vec<String>, ()> {
    use d3_geo_rs::projection::{ScaleSet, TranslateSet};
    use geo::GeometryCollection;

    let width = 1000_f64;
    let height = 1000_f64;

    let topology = world::<f64>();
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

    let mut builder = PathBuilder::context_pathstring().build(ortho);
    let mut i = 0;

    let mut paths = vec![];
    if let Geometry::GeometryCollection(GeometryCollection(g_vec)) = &countries {
        for g in g_vec {
            match &g {
                Geometry::MultiPolygon(mp) => {
                    i += 1;
                    for p in &mp.0 {
                        let s = builder.object(&Geometry::Polygon(p.clone()));
                        if !s.is_empty() {
                            paths.push(format!(
                                "<path d = \"{s}\" class=\"id-{i}\" style=\"{}\"></path>",
                                fill[i % 7]
                            ));
                            i += 1
                        }
                    }
                }
                Geometry::Polygon(p) => {
                    let s = builder.object(&Geometry::Polygon(p.clone()));
                    if !s.is_empty() {
                        paths.push(format!(
                            "<path d = \"{s}\" class=\"id-{i}\" style=\"{}\"></path>",
                            fill[i % 7]
                        ));
                        i += 1
                    }
                }

                _ => {
                    // console_log!("Not polygon, Not Multipolygon.");
                }
            }
        }
    }

    // // Graticule
    // let graticule = generate_mls::<f64>();

    // let graticule_d = builder.object(&graticule);
    // // let class_name = "graticule";
    // // let path = path_node(class_name)?;
    // // path.set_attribute_ns(None, "d", &graticule_d)?;
    // // path.set_attribute_ns(None, "style", "#ccc")?;
    // // svg.append_child(&path)?;
    // paths.push(format!(
    //     "<path d = \"{graticule_d}\" class=\"graticule\" style=\"#ccc\"/>"
    // ));
    Ok(paths)
}

#[cfg(not(tarpaulin_include))]
fn main() -> std::io::Result<()> {
    let file = File::create("profile_output.html")?;
    let mut file = LineWriter::new(file);

    file.write_all(b"
    <!DOCTYPE html>
    <html lang=\"en\">
    <head>
    <title>Profile Target</title>
    <meta charset=\"utf-8\">
    <meta name=\"description\" content=\"Complex output used for profiling.\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">
    </head>
    <body>
    <h1>Project: rust_d3_geo_voronoi</h1>
    <p>
     A Complex SVG used for profiling.
    </p>
    <?xml version=\"1.0\" standalone=\"no\"?><!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">
    <svg version=\"1.1\"
      width=\"1280\"
      height=\"1280\"
      viewBox=\"0 0 1200 1280\"
      xmlns=\"http://www.w3.org/2000/svg\"
    >
    ")?;

    // file.write_all(draw().as_bytes())?;
    match draw() {
        Ok(paths) => {
            for path in paths {
                file.write_all(path.as_bytes())?;
            }
            file.write_all(b"</svg></body></html>")?;

            file.flush()?;

            Ok(())
        }
        Err(_) => Ok(()),
    }
}
