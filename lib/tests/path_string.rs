use core::f64::consts::PI;
use core::fmt::Display;
use core::ops::AddAssign;

use geo::line_string;
use geo::point;
use geo::CoordFloat;
use geo::Geometry;
use geo::GeometryCollection;
use geo::MultiPolygon;
use geo::Polygon;
use geo_types::Coord;

use num_traits::FloatConst;
use pretty_assertions::assert_eq;

use d3_geo_rs::circle::generator::Generator as CircleGenerator;
use d3_geo_rs::path::builder::Builder as PathBuilder;
use d3_geo_rs::path::string::String as PathString;
use d3_geo_rs::path::PointRadiusTrait;
use d3_geo_rs::projection::equirectangular::Equirectangular;
use d3_geo_rs::projection::orthographic::Orthographic;
use d3_geo_rs::projection::projector_common::types::ProjectorAntimeridianResampleNoneNoClip;
use d3_geo_rs::projection::Build;
use d3_geo_rs::projection::PrecisionBypass;
use d3_geo_rs::projection::RawBase;
use d3_geo_rs::projection::ScaleSet;
use d3_geo_rs::projection::TranslateSet;
use d3_geo_rs::stream::Streamable;

#[inline]
fn equirectangular<T>(
) -> ProjectorAntimeridianResampleNoneNoClip<PathString<T>, Equirectangular<T>, T>
where
    T: 'static + CoordFloat + Default + FloatConst,
{
    Equirectangular::builder()
        .scale_set(T::from(900f64 / PI).unwrap())
        .precision_bypass()
        .build()
}

#[inline]
fn path<T>(
    projection: ProjectorAntimeridianResampleNoneNoClip<
        PathString<T>,
        Equirectangular<T>,
        T,
    >,

    object: impl Streamable<T = T>,
) -> String
where
    T: 'static + AddAssign + CoordFloat + Display + FloatConst,
{
    let pb = PathBuilder::pathstring();
    pb.build(projection).object(&object)
}

#[test]
fn point_renders_a_point() {
    println!("geoPath.point(…) renders a point");
    let object = Geometry::Point(point!(x: -63_f64, y:18_f64));
    let eq = equirectangular::<f64>();
    assert_eq!(
        path(eq, object),
        "M165,160m0,4.5a4.5,4.5 0 1,1 0,-9a4.5,4.5 0 1,1 0,9z"
    );
}

#[test]
fn point_renders_a_point_of_given_radius() {
    println!("geoPath.point(…) renders a point of a given radius");

    let mut builder = PathBuilder::pathstring();

    builder.point_radius(10_f64);

    let eq = equirectangular();
    let mut path = builder.build(eq);
    let object = Geometry::Point(point!(x: -63_f64, y:18_f64));

    assert_eq!(
        path.object(&object),
        "M165,160m0,10a10,10 0 1,1 0,-20a10,10 0 1,1 0,20z"
    );
}

#[test]
fn renders_multipoint() {
    println!("geoPath(MultiPoint) renders a point");
    let object = Geometry::MultiPoint(
        vec![
            point![x:-63_f64, y:18_f64],
            point![x:-62_f64, y:18_f64],
            point![x:-62_f64, y:17_f64],
        ]
        .into(),
    );
    let eq = equirectangular();
    assert_eq!(path(eq, object),
			"M165,160m0,4.5a4.5,4.5 0 1,1 0,-9a4.5,4.5 0 1,1 0,9zM170,160m0,4.5a4.5,4.5 0 1,1 0,-9a4.5,4.5 0 1,1 0,9zM170,165m0,4.5a4.5,4.5 0 1,1 0,-9a4.5,4.5 0 1,1 0,9z");
}

#[test]
fn renders_a_line_string() {
    let object = Geometry::LineString(line_string![
        (x:-63_f64, y:18_f64), (x:-62_f64, y:18_f64), (x:-62_f64, y:17_f64)
    ]);
    let eq = equirectangular();
    assert_eq!(path(eq, object), "M165,160L170,160L170,165");
}

#[test]
fn renders_a_polygon() {
    let object = Geometry::Polygon(Polygon::new(
        line_string![
            (x:-63_f64, y:18_f64), (x:-62_f64, y:18_f64), (x:-62_f64, y:17_f64)
        ],
        vec![],
    ));
    let eq = equirectangular();
    assert_eq!(path(eq, object), "M165,160L170,160L170,165Z");
}

#[test]
// This has no equivalent in JS testing, looking down the js functions, it is a hole in the test strategy.
// the values for everything after the first z where copied from a modified javascript test.
fn renders_a_multi_polygon() {
    let object = Geometry::MultiPolygon(MultiPolygon(vec![
        Polygon::new(
            line_string![
                (x:-63_f64, y:18_f64), (x:-62_f64, y:18_f64), (x:-62_f64, y:17_f64)
            ],
            vec![],
        ),
        Polygon::new(
            line_string![
                (x:0_f64, y:0_f64), (x:0_f64, y:1_f64), (x:1_f64, y:1_f64), (x:0_f64, y:1_f64),(x:0_f64, y:0_f64)
            ],
            vec![],
        ),
    ]));

    let eq = equirectangular();
    assert_eq!(
        path(eq, object),
        "M165,160L170,160L170,165ZM480,250L480,245L485,245L480,245Z"
    );
}

#[test]
fn render_a_simple_multi_polygon() {
    let mut gc = CircleGenerator::default();
    gc.radius_set(10_f64).precision_set(80_f64);

    let mut p_vec = vec![];

    let lat = 0;
    for long in (0..=40).step_by(40) {
        let poly = Polygon::new(
            gc.clone()
                .center_set(&Coord {
                    x: long as f64,
                    y: lat as f64,
                })
                .circle(),
            vec![],
        );
        p_vec.push(poly);
    }
    let object = Geometry::MultiPolygon(MultiPolygon(p_vec));

    let ortho = Orthographic::<_>::builder()
        .scale_set(240_f64)
        .translate_set(&Coord {
            x: 300_f64,
            y: 300_f64,
        })
        .build();

    let pb = PathBuilder::pathstring();
    let s = pb.build(ortho).object(&object);

    assert_eq!(s, "M258.957583,307.236886L285.746118,260.837781L336.092096,279.162219L326.788535,331.925333L273.211465,331.925333ZM420.485018,307.236886L431.567778,283.936957L441.006226,260.837781L479.573483,279.162219L477.1168,305.578636L472.446542,331.925333L431.404125,331.925333Z");
}

#[test]
fn renders_a_geometry_collection() {
    let object = Geometry::GeometryCollection(GeometryCollection(vec![
        Geometry::Polygon(Polygon::new(
            line_string![
                (x:-63_f64, y:18_f64), (x:-62_f64, y:18_f64), (x:-62_f64, y:17_f64)
            ],
            vec![],
        )),
    ]));
    let eq = equirectangular();
    assert_eq!(path(eq, object), "M165,160L170,160L170,165Z");
}

// Missing Feature, FeatureCollection test.

#[test]
fn line_string_then_point() {
    println!(
            "geoPath(LineString) then geoPath(Point) does not treat the point as part of a line"
        );
    let line_object = Geometry::LineString(line_string![
        (x:-63_f64, y:18_f64), (x:-62_f64, y:18_f64), (x:-62_f64, y:17_f64)
    ]);
    let point_object = Geometry::Point(point!(x: -63_f64, y:18_f64));
    let eq = equirectangular();

    assert_eq!(path(eq.clone(), line_object), "M165,160L170,160L170,165");

    assert_eq!(
        path(eq, point_object),
        "M165,160m0,4.5a4.5,4.5 0 1,1 0,-9a4.5,4.5 0 1,1 0,9z"
    );
}
