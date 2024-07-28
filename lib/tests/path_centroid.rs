
extern crate pretty_assertions;

use core::f64::consts::PI;
use core::ops::AddAssign;

use geo::line_string;
use geo::CoordFloat;
use geo::Geometry;
use geo::GeometryCollection;
use geo::MultiLineString;
use geo::MultiPoint;
use geo::MultiPolygon;
use geo::Point;
use geo::Polygon;
use geo_types::Coord;
use num_traits::AsPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

use d3_geo_rs::in_delta::point as in_delta_point;
use d3_geo_rs::path::centroid::Centroid;
use d3_geo_rs::path::Path;
use d3_geo_rs::projection::builder::Builder as ProjectionBuilderCommon;
use d3_geo_rs::projection::equirectangular::Equirectangular;
use d3_geo_rs::projection::projector_common::types::ProjectorAntimeridianResampleNoneNoClip;
use d3_geo_rs::projection::Build;
use d3_geo_rs::projection::BuilderTrait as ProjectionBuilderTrait;
use d3_geo_rs::projection::PrecisionBypass;
use d3_geo_rs::projection::ScaleSet;
use d3_geo_rs::stream::Streamable;

#[inline]
fn equirectangular<T>(
) -> ProjectorAntimeridianResampleNoneNoClip<Centroid<T>, Equirectangular<T>, T>
where
    T: 'static + AddAssign<T> + CoordFloat + Default + FloatConst,
{
    ProjectionBuilderCommon::new(Equirectangular::default())
        .scale_set(T::from(900f64 / PI).unwrap())
        .precision_bypass()
        .build()
}

#[inline]
fn centroid<T>(
    projection: ProjectorAntimeridianResampleNoneNoClip<
        Centroid<T>,
        Equirectangular<T>,
        T,
    >,

    object: &impl Streamable<T = T>,
) -> Point<T>
where
    T: AddAssign<T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
    let cs = Centroid::default();
    let result = Path::new(cs, projection).centroid(object);
    Point(result)
}

#[test]
fn of_a_point() {
    println!("geoPath.centroid(…) of a point");
    let point = Geometry::Point(Point(Coord { x: 0_f64, y: 0_f64 }));

    let eq = equirectangular();
    assert!(in_delta_point(
        centroid(eq, &point),
        Point::new(480_f64, 250_f64),
        1e-6_f64
    ));
}

#[test]
fn of_a_empty_multipoint() {
    println!("geoPath.centroid(…) of an empty multipoint");
    let mp = Geometry::MultiPoint(MultiPoint(vec![]));

    let eq = equirectangular();
    assert!(in_delta_point(
        centroid(eq, &mp),
        Point::new(f64::nan(), f64::nan()),
        1e-6_f64
    ));
}

#[test]
fn of_a_singleton_multipoint() {
    println!("geoPath.centroid(…) of an singleton  multipoint");
    let mp = Geometry::MultiPoint(MultiPoint(vec![Point::new(0_f64, 0_f64)]));

    let eq = equirectangular();
    assert!(in_delta_point(
        centroid(eq, &mp),
        Point::new(480_f64, 250_f64),
        1e-6_f64
    ));
}

#[test]
fn of_a_two_points() {
    println!("geoPath.centroid(…) of an singleton  multipoint");
    let mp = Geometry::MultiPoint(MultiPoint(vec![
        Point::new(-122_f64, 37_f64),
        Point::new(-74_f64, 40_f64),
    ]));

    let eq = equirectangular();
    assert!(in_delta_point(
        centroid(eq, &mp),
        Point::new(-10_f64, 57.5_f64),
        1e-6_f64
    ));
}

#[test]
fn of_an_empty_linestring() {
    println!("geoPath.centroid(…) of an empty linestring");
    let ls = Geometry::LineString(line_string![]);

    let eq = equirectangular();
    assert!(in_delta_point(
        centroid(eq, &ls),
        Point::new(f64::nan(), f64::nan()),
        1e-6_f64
    ));
}

#[test]
fn of_a_linestring_with_two_points() {
    println!("geoPath.centroid(…) of an empty linestring");
    let ls1 = Geometry::LineString(line_string![
        (x: 100_f64, y:0_f64),
        (x: 0_f64, y:0_f64)
    ]);

    let eq = equirectangular();
    assert!(in_delta_point(
        centroid(eq, &ls1),
        Point::new(730_f64, 250_f64),
        1e-6_f64
    ));

    let ls2 = Geometry::LineString(line_string![
        (x: 0_f64, y:0_f64),
        (x: 100_f64, y:0_f64),
        (x: 101_f64, y: 0_f64)
    ]);

    let eq = equirectangular();
    assert!(in_delta_point(
        centroid(eq, &ls2),
        Point::new(732.5_f64, 250_f64),
        1e-6_f64
    ));
}

#[test]
fn of_a_linestring_with_two_points_one_unique() {
    println!("geoPath.centroid(…) of a linestring with two points, one unique");
    let ls1 = Geometry::LineString(line_string![
        (x: -122_f64, y:37_f64),
        (x: -122_f64, y:37_f64),
    ]);

    let eq = equirectangular();
    assert!(in_delta_point(
        centroid(eq, &ls1),
        Point::new(-130_f64, 65_f64),
        1e-6_f64
    ));

    let ls2 = Geometry::LineString(line_string![
        (x: -74_f64, y: 40_f64),
        (x: -74_f64, y: 40_f64)
    ]);

    let eq = equirectangular();
    assert!(in_delta_point(
        centroid(eq, &ls2),
        Point::new(110_f64, 50_f64),
        1e-6_f64
    ));
}

#[test]
fn of_a_linestring_with_three_points_two_unique() {
    println!(
        "geoPath.centroid(…) of a linestring with three points; two unique"
    );
    let ls = Geometry::LineString(line_string![
        (x: -122_f64, y:37_f64),
        (x: -74_f64, y:40_f64),
        (x: -74_f64, y:40_f64),
    ]);

    let eq = equirectangular();
    assert!(in_delta_point(
        centroid(eq, &ls),
        Point::new(-10_f64, 57.5_f64),
        1e-6_f64
    ));
}

#[test]
fn of_a_linestring_with_three_points() {
    println!("geoPath.centroid(…) of a linestring with three points");
    let ls = Geometry::LineString(line_string![
        (x: -122_f64, y:37_f64),
        (x: -74_f64, y:40_f64),
        (x: -100_f64, y:0_f64),
    ]);

    let eq = equirectangular();

    assert!(in_delta_point(
        centroid(eq, &ls),
        Point::new(17.389135_f64, 103.563545_f64),
        1e-6_f64
    ));
}

#[test]
fn of_a_multiline_string() {
    println!("geoPath.centroid(…) of a multilinestring");
    let mls = Geometry::MultiLineString(MultiLineString(vec![
        line_string![
            (x: 100_f64, y:0_f64),
            (x: 0_f64, y:0_f64),
        ],
        line_string![
            (x: -10_f64, y:0_f64),
            (x: 0_f64, y:0_f64),
        ],
    ]));

    let eq = equirectangular();
    assert!(in_delta_point(
        centroid(eq, &mls),
        Point::new(705_f64, 250_f64),
        1e-6_f64
    ));
}

#[test]
fn of_a_single_ring_polygon() {
    println!("geoPath.centroid(…) of a single-ring polygon");
    let p = Geometry::Polygon(Polygon::new(
        line_string![
            (x: 100_f64, y:0_f64),
            (x: 100_f64, y:1_f64),
            (x: 101_f64, y:1_f64),
            (x: 101_f64, y:0_f64),
            (x: 100_f64, y:0_f64)
        ],
        vec![],
    ));

    let eq = equirectangular();
    assert!(in_delta_point(
        centroid(eq, &p),
        Point::new(982.5_f64, 247.5_f64),
        1e-6_f64
    ));
}

#[test]
fn of_a_zero_area_polygon() {
    println!("geoPath.centroid(…) of a zero-area polygon");
    let p = Geometry::Polygon(Polygon::new(
        line_string![
            (x: 1_f64, y:0_f64),
            (x: 2_f64, y:0_f64),
            (x: 3_f64, y:0_f64),
            (x: 1_f64, y:0_f64),
        ],
        vec![],
    ));

    let eq = equirectangular();
    assert!(in_delta_point(
        centroid(eq, &p),
        Point::new(490_f64, 250_f64),
        1e-6_f64
    ));
}

#[test]
fn of_a_polygon_with_two_rings_one_zero_area() {
    println!(
        "geoPath.centroid(…) of a polygon with two rings, one with zero area"
    );
    let p = Geometry::Polygon(Polygon::new(
        line_string![
            (x: 100_f64, y:0_f64),
            (x: 100_f64, y:1_f64),
            (x: 101_f64, y:1_f64),
            (x: 101_f64, y:0_f64),
            (x: 100_f64, y:0_f64),
        ],
        vec![line_string![
            (x: 100.1_f64, y:0_f64),
            (x: 100.2_f64, y:0_f64),
            (x: 100.3_f64, y:0_f64),
            (x: 101.1_f64, y:0_f64),
        ]],
    ));

    let eq = equirectangular();
    assert!(in_delta_point(
        centroid(eq, &p),
        Point::new(982.5_f64, 247.5_f64),
        1e-6_f64
    ));
}

#[test]
fn of_a_polygon_with_clockwise_exterior_and_anticlockwise_interior() {
    println!(
            "geoPath.centroid(…) of a polygon with clockwise exterior and anticlockwise interior"
        );
    let mut ext_vec = vec![
        (-2_f64, -2_f64),
        (2_f64, -2_f64),
        (2_f64, 2_f64),
        (-2_f64, 2_f64),
        (-2_f64, -2_f64),
    ];
    ext_vec.reverse();
    let polygon = Geometry::Polygon(Polygon::new(
        ext_vec.into(),
        vec![line_string![
            (x: 0_f64, y:-1_f64),
            (x: 1_f64, y:-1_f64),
            (x: 1_f64, y:1_f64),
            (x: 0_f64, y:1_f64),
            (x: 0_f64, y:-1_f64),
        ]],
    ));

    let eq = equirectangular();
    assert!(in_delta_point(
        centroid(eq, &polygon),
        Point::new(479.642857_f64, 250_f64),
        1e-6_f64
    ));
}

#[test]
fn of_an_empty_multipolygon() {
    println!("geoPath.centroid(…) of an empty multipolygon");

    let polygon = Geometry::MultiPolygon(MultiPolygon(vec![]));

    let eq = equirectangular();
    assert!(in_delta_point(
        centroid(eq, &polygon),
        Point::new(f64::nan(), f64::nan()),
        1e-6_f64
    ));
}

#[test]
fn of_a_singleton_multipolygon() {
    println!("geoPath.centroid(…) of a singleton multipolygon");

    let polygon = Geometry::MultiPolygon(MultiPolygon(vec![Polygon::new(
        line_string![
            (x: 100_f64, y: 0_f64),
            (x: 100_f64, y: 1_f64),
            (x: 101_f64, y: 1_f64),
            (x: 101_f64, y: 0_f64),
            (x: 100_f64, y: 0_f64)
        ],
        vec![],
    )]));

    let eq = equirectangular();
    assert!(in_delta_point(
        centroid(eq, &polygon),
        Point::new(982.5_f64, 247.5_f64),
        1e-6_f64
    ));
}

#[test]
fn of_a_multipolygon_with_two_polygons() {
    println!("geoPath.centroid(…) of a multipolygon with two polygons");

    let polygon = Geometry::MultiPolygon(MultiPolygon(vec![
        Polygon::new(
            line_string![
                (x: 100_f64, y:0_f64),
                (x: 100_f64, y:1_f64),
                (x: 101_f64, y: 1_f64),
                (x: 101_f64, y: 0_f64),
                (x: 100_f64, y: 0_f64)
            ],
            vec![],
        ),
        Polygon::new(
            line_string![
                (x: 0_f64, y:0_f64),
                (x: 1_f64, y:0_f64),
                (x: 1_f64, y: -1_f64),
                (x: 0_f64, y: -1_f64),
                (x: 0_f64, y: 0_f64)
            ],
            vec![],
        ),
    ]));

    let eq = equirectangular();
    assert!(in_delta_point(
        centroid(eq, &polygon),
        Point::new(732.5_f64, 250_f64),
        1e-6_f64
    ));
}

#[test]
fn of_a_multipolygon_with_two_polygons_one_zero_area() {
    println!("geoPath.centroid(…) of a multipolygon with two polygons");

    let polygon = Geometry::MultiPolygon(MultiPolygon(vec![
        Polygon::new(
            line_string![
                (x: 100_f64, y:0_f64),
                (x: 100_f64, y:1_f64),
                (x: 101_f64, y: 1_f64),
                (x: 101_f64, y: 0_f64),
                (x: 100_f64, y: 0_f64)
            ],
            vec![],
        ),
        Polygon::new(
            line_string![
                (x: 0_f64, y:0_f64),
                (x: 1_f64, y:0_f64),
                (x: 2_f64, y: 0_f64),
                (x: 0_f64, y: 0_f64)
            ],
            vec![],
        ),
    ]));

    let eq = equirectangular();
    assert!(in_delta_point(
        centroid(eq, &polygon),
        Point::new(982.5_f64, 247.5_f64),
        1e-6_f64
    ));
}

#[test]
fn of_a_geometry_collection_with_a_single_point() {
    println!(
        "geoPath.centroid(…) of a geometry collection with a single point"
    );

    let gc = Geometry::GeometryCollection(GeometryCollection(vec![
        Geometry::Point(Point::new(0_f64, 0_f64)),
    ]));

    let eq = equirectangular();
    assert!(in_delta_point(
        centroid(eq, &gc),
        Point::new(480_f64, 250_f64),
        1e-6_f64
    ));
}

#[test]
fn of_a_geometry_collection_with_a_point_and_a_linestring() {
    println!("geoPath.centroid(…) of a geometry collection with a point and a linestring");

    let gc = Geometry::GeometryCollection(GeometryCollection(vec![
        Geometry::LineString(
            line_string![(x:179_f64, y:0_f64),(x:180_f64, y:0_f64) ],
        ),
        Geometry::Point(Point::new(0_f64, 0_f64)),
    ]));

    let eq = equirectangular();
    assert!(in_delta_point(
        centroid(eq, &gc),
        Point::new(1377.5_f64, 250_f64),
        1e-6_f64
    ));
}

#[test]
fn of_a_geometry_collection_with_a_point_and_a_linestring_and_a_polygon() {
    println!(
            "geoPath.centroid(…) of a geometry collection with a point, linestring and polygon"
        );

    let gc = Geometry::GeometryCollection(GeometryCollection(vec![
        Geometry::Polygon(Polygon::new(
            line_string![
                (x: -180_f64, y:0_f64),
                (x: -180_f64, y:1_f64),
                (x: -179_f64, y: 1_f64),
                (x: -179_f64, y: 0_f64),
                (x: -180_f64, y: 0_f64),
            ],
            vec![],
        )),
        Geometry::LineString(
            line_string![(x:179_f64, y:0_f64),(x:180_f64, y:0_f64) ],
        ),
        Geometry::Point(Point::new(0_f64, 0_f64)),
    ]));

    let eq = equirectangular();
    assert!(in_delta_point(
        centroid(eq, &gc),
        Point::new(-417.5_f64, 247.5_f64),
        1e-6_f64
    ));
}

//   it("geoPath.centroid(…) of a feature collection with a point", () => {
//     assert.deepStrictEqual(testCentroid(equirectangular, {type: "FeatureCollection", features: [{type: "Feature", geometry: {type: "Point", coordinates: [0, 0]}}]}), [480, 250]);
//   });

//   it("geoPath.centroid(…) of a feature collection with a point and a line string", () => {
//     assert.deepStrictEqual(testCentroid(equirectangular, {type: "FeatureCollection", features: [
//       {type: "Feature", geometry: {type: "LineString", coordinates: [[179, 0], [180, 0]]}},
//       {type: "Feature", geometry: {type: "Point", coordinates: [0, 0]}}
//     ]}), [1377.5, 250]);
//   });

//   it("geoPath.centroid(…) of a feature collection with a point, line string and polygon", () => {
//     assert.deepStrictEqual(testCentroid(equirectangular, {type: "FeatureCollection", features: [
//       {type: "Feature", geometry: {type: "Polygon", coordinates: [[[-180, 0], [-180, 1], [-179, 1], [-179, 0], [-180, 0]]]}},
//       {type: "Feature", geometry: {type: "LineString", coordinates: [[179, 0], [180, 0]]}},
//       {type: "Feature", geometry: {type: "Point", coordinates: [0, 0]}}
//     ]}), [-417.5, 247.5]);
//   });

//   it("geoPath.centroid(…) of a sphere", () => {
//     assert.deepStrictEqual(testCentroid(equirectangular, {type: "Sphere"}), [480, 250]);
//   });
