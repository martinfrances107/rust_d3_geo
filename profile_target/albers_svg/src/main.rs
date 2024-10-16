use std::fs::File;
use std::io::BufReader;
use std::io::LineWriter;
use std::io::Write;

use geo::Geometry;
use geo::GeometryCollection;
use rust_topojson_client::feature::feature_from_name;
use topojson::Topology;

use d3_geo_rs::path::builder::Builder as PathBuilder;
use d3_geo_rs::path::string::String as PathString;
use d3_geo_rs::path::Result as PathResult;

use d3_geo_rs::projection::Projector;
use d3_geo_rs::stream::Stream;
use d3_geo_rs::stream::Streamable;

/// Helper function to extract world geometry from file.
fn world() -> Topology {
    let file = File::open("./world-atlas/world/counties-10m.json")
        .expect("File should open read only.");
    let bufreader = BufReader::new(file);
    serde_json::from_reader(bufreader).expect("File should be parse as JSON.")
}

fn parse_topology() -> Geometry {
    let topology = world();
    feature_from_name(&topology, "counties").expect("Did not extract geometry")
}

static FILL: [&str; 7] = [
    "fill: red",
    "fill: orange",
    "fill: olive",
    "fill: blue",
    "fill: indigo",
    "fill: brown",
    "fill: silver",
];

fn draw(counties: &Geometry) -> Result<Vec<String>, ()> {
    use d3_geo_rs::projection::albers_usa::AlbersUsa;

    let albers_usa = AlbersUsa::<PathString<f64>, f64>::builder().build();

    let pb = PathBuilder::albers_pathstring();

    let mut path = pb.build(albers_usa);

    let mut c_index = 0;
    let mut paths: Vec<String> = vec![];
    if let Geometry::GeometryCollection(GeometryCollection(g_vec)) = &counties {
        for g in g_vec {
            match &g {
                Geometry::MultiPolygon(mp) => {
                    for p in mp.0.iter() {
                        // TODO: this object() call is identical to the 3 lines below
                        // Can I restore the object call?
                        let mut stream_in =
                            path.projector.stream(&path.context);
                        let object = Geometry::Polygon(p.clone());
                        object.to_stream(&mut stream_in);

                        for s in stream_in.endpoint().result() {
                            if !s.is_empty() {
                                paths.push(format!(
                                    "<path d = \"{s}\" style=\"{}\"></path>",
                                    FILL[c_index % 7]
                                ));
                            }
                        }
                    }
                }
                Geometry::Polygon(p) => {
                    let mut stream_in = path.projector.stream(&path.context);
                    let object = Geometry::Polygon(p.clone());
                    object.to_stream(&mut stream_in);

                    for s in stream_in.endpoint().result().iter() {
                        paths.push(format!(
                            "<path d = \"{s}\" style=\"{}\"></path>",
                            FILL[c_index % 7]
                        ));
                    }
                }
                _ => {}
            }
            c_index += 1;
        }
    }

    Ok(paths)
}

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
    match draw(&countries) {
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
