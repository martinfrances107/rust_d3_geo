#![cfg(not(tarpaulin_include))]

use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;
use std::io::LineWriter;

use d3_geo_rs::graticule::builder::Builder as GraticuleBuilder;

use d3_geo_rs::path::builder::Builder as PathBuilder;
use d3_geo_rs::path::string::String as PathString;
use d3_geo_rs::projection::Build;
use geo::CoordFloat;
use geo::Geometry;
use geo_types::Coord;
use num_traits::FloatConst;
use rust_topojson_client::feature::feature_from_name;
use topojson::Topology;

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
    serde_json::from_reader(file).expect("File should be parse as JSON.")
}

fn parse_topology() -> Geometry {
    let topology = world::<f64>();
    feature_from_name(&topology, "countries").expect("Did not extract geometry")
}

#[cfg(not(tarpaulin_include))]
fn draw(countries: Geometry) -> Result<Vec<String>, ()> {
    use d3_geo_rs::{
        graticule::{generate, generate_mls},
        projection::{albers::albers, builder::template::PCNC, ScaleSet, TranslateSet},
    };
    use geo::LineString;

    let width = 1200_f64;
    let height = 1200_f64;

    let ortho = albers()
        .scale_set(width / 1.5_f64 / std::f64::consts::PI)
        .translate_set(&Coord {
            x: width / 2_f64,
            y: height / 2_f64,
        })
        // .rotate2_set(&[270_f64, 0_f64])
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

    // let builder: PathBuilder<_, _, PathString<f64>, PCNC<PathString<f64>, f64>, _, _, _, _, f64> =
    let builder = PathBuilder::context_pathstring();
    let mut projector = builder.build(ortho);
    let mut i = 0;

    let mut paths = vec![];
    // if let Geometry::GeometryCollection(GeometryCollection(g_vec)) = &countries {
    //     for g in g_vec {
    //         match &g {
    //             Geometry::MultiPolygon(mp) => {
    //                 i += 1;
    //                 for p in &mp.0 {
    //                     let s = builder.object(&Geometry::Polygon(p.clone()));
    //                     if !s.is_empty() {
    //                         paths.push(format!(
    //                             "<path d = \"{s}\" class=\"id-{i}\" style=\"{}\"></path>",
    //                             fill[i % 7]
    //                         ));
    //                         i += 1
    //                     }
    //                 }
    //             }
    //             Geometry::Polygon(p) => {
    //                 let s = builder.object(&Geometry::Polygon(p.clone()));
    //                 if !s.is_empty() {
    //                     paths.push(format!(
    //                         "<path d = \"{s}\" class=\"id-{i}\" style=\"{}\"></path>",
    //                         fill[i % 7]
    //                     ));
    //                     i += 1
    //                 }
    //             }

    //             _ => {}
    //         }
    //     }
    // }

    // Graticule
    // let graticule = generate_mls();
    let mls = GraticuleBuilder::<f64>::default()
        .extent_minor_set([[-180_f64, 50_f64 - 1e-6], [180_f64, 60_f64 + 1e-6]])
        .lines()
        .collect::<Vec<LineString<f64>>>();
    // let graticule = Geometry::MultiLineString(mls);
    for (i, ls) in mls.iter().enumerate() {
        let d = projector.object(ls);
        paths.push(format!(
            "<path d = \"{}\" class=\"graticule{}\" style=\"#ccc\"/></path>",
            d, i
        ));
    }

    // let graticule_d = builder.object(&graticule);
    // paths.push(format!(
    //     "<path d = \"{graticule_d}\" class=\"graticule\" style=\"#ccc\"/></path>"
    // ));

    // Render points.
    // let phi_range = 0..360;
    // let lambda_range = -90..90;
    // let n_poles = phi_range
    //     .clone()
    //     .step_by(10)
    //     .map(|l| Coord {
    //         x: l as f64,
    //         y: 90_f64,
    //     })
    //     .collect::<Vec<Coord>>();

    // let s_poles = phi_range
    //     .clone()
    //     .step_by(10)
    //     .map(|l| Coord {
    //         x: l as f64,
    //         y: -90_f64,
    //     })
    //     .collect::<Vec<Coord>>();

    // let point_grid = phi_range
    //     .step_by(10)
    //     .map(|l| {
    //         lambda_range
    //             .clone()
    //             .step_by(10)
    //             .map(|phi| {
    //                 dbg!();
    //                 Coord {
    //                     x: l as f64,
    //                     y: phi as f64,
    //                 }
    //             })
    //             .collect::<Vec<Coord>>()
    //     })
    //     .collect::<Vec<Vec<Coord>>>();

    // for p in n_poles {
    //     let point_d = builder.object(&Geometry::Point(p.into()));
    //     paths.push(format!(
    //         "<path d = \"{point_d}\" class=\"graticule\" style=\"#111\"/></path>"
    //     ));
    // }
    // for p in s_poles {
    //     let point_d = builder.object(&Geometry::Point(p.into()));
    //     paths.push(format!(
    //         "<path d = \"{point_d}\" class=\"graticule\" style=\"#111\"/></path>"
    //     ));
    // }

    // for line in point_grid {
    //     for p in line {
    //         let point_d = builder.object(&Geometry::Point(p.into()));
    //         paths.push(format!(
    //             "<path d = \"{point_d}\" class=\"graticule\" style=\"#111\"/></path>"
    //         ));
    //     }
    // }

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
    <h1>Project: rust_d3_geo</h1>
    <p>
     Show problem with albers.
    </p>
    <?xml version=\"1.0\" standalone=\"no\"?><!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">
    <svg version=\"1.1\"
      width=\"1280\"
      height=\"1280\"
      viewBox=\"0 0 1200 1280\"
      xmlns=\"http://www.w3.org/2000/svg\"
    >
    <style>
    path {
      fill: None;
      stroke: black;
      stroke-width: 1px;
    }
     </style>
    ")?;

    // file.write_all(draw().as_bytes())?;
    let countries = parse_topology();
    match draw(countries) {
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
