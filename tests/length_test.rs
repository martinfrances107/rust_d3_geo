mod length_test {
    #[cfg(test)]
    extern crate pretty_assertions;
    use delaunator::Point;
    use rust_d3_geo::data_object::DataObject;
    use rust_d3_geo::data_object::FeatureGeometry;
    use rust_d3_geo::data_object::FeaturesStruct;
    use rust_d3_geo::in_delta::in_delta;
    use rust_d3_geo::length::LengthStream;
    use std::f64::consts::PI;

    #[test]
    fn line_string_great_arc_segements() {
        println!("geoLength(LineString) returns the sum of its great-arc segments");
        assert!(in_delta(
            LengthStream::calc(DataObject::LineString {
                coordinates: vec![Point { x: -45f64, y: 0f64 }, Point { x: 45f64, y: 0f64 }]
            }),
            PI / 2f64,
            1e-6
        ));

        assert!(in_delta(
            LengthStream::calc(DataObject::LineString {
                coordinates: vec![
                    Point { x: -45f64, y: 0f64 },
                    Point { x: -30f64, y: 0f64 },
                    Point { x: -15f64, y: 0f64 },
                    Point { x: 0f64, y: 0f64 }
                ]
            }),
            PI / 4f64,
            1e-6
        ));
    }

    #[test]
    fn fc_line_string_the_sum_of_its_features() {
        println!("geoLength(FeatureCollection) returns the sum of its features’ lengths");
        assert!(in_delta(
            LengthStream::calc(DataObject::FeatureCollection {
                features: vec![FeaturesStruct {
                    properties: Vec::new(),
                    geometry: vec![
                        FeatureGeometry::LineString {
                            coordinates: vec![
                                Point { x: -45f64, y: 0f64 },
                                Point { x: 0f64, y: 0f64 }
                            ]
                        },
                        FeatureGeometry::LineString {
                            coordinates: vec![
                                Point { x: 0f64, y: 0f64 },
                                Point { x: 45f64, y: 0f64 }
                            ]
                        },
                    ],
                }]
            }),
            PI / 2f64,
            1e-6
        ));
    }

    #[test]
    fn polygon_length_of_perimeter() {
        println!("geoLength(Polygon) returns the length of its perimeter");
        assert!(in_delta(
            LengthStream::calc(DataObject::Polygon {
                coordinates: vec![vec![
                    Point { x: 0f64, y: 0f64 },
                    Point { x: 3f64, y: 0f64 },
                    Point { x: 3f64, y: 3f64 },
                    Point { x: 0f64, y: 3f64 },
                    Point { x: 0f64, y: 0f64 }
                ]]
            }),
            0.157008f64,
            1e-6f64
        ));
    }

    #[test]
    fn polygon_length_of_perimeter_including_holes() {
        println!("geoLength(Polygon) returns the length of its perimeter, including holes");
        assert!(in_delta(
            LengthStream::calc(DataObject::Polygon {
                coordinates: vec![
                    vec![
                        Point { x: 0f64, y: 0f64 },
                        Point { x: 3f64, y: 0f64 },
                        Point { x: 3f64, y: 3f64 },
                        Point { x: 0f64, y: 3f64 },
                        Point { x: 0f64, y: 0f64 }
                    ],
                    vec![
                        Point { x: 1f64, y: 1f64 },
                        Point { x: 2f64, y: 1f64 },
                        Point { x: 2f64, y: 2f64 },
                        Point { x: 1f64, y: 2f64 },
                        Point { x: 1f64, y: 1f64 }
                    ]
                ]
            }),
            0.209354f64,
            1e-6f64
        ));
    }

    // tape("geoLength(FeatureCollection) returns the sum of its features’ lengths", function(test) {
    //   test.inDelta(d3.geoLength({
    //     type: "FeatureCollection", features: [
    //       {type: "Feature", geometry: {type: "LineString", coordinates: [[-45, 0], [0, 0]]}},
    //       {type: "Feature", geometry: {type: "LineString", coordinates: [[0, 0], [45, 0]]}}
    //     ]
    //   }), Math.PI / 2, 1e-6);
    //   test.end();
    // });
}

// var tape = require("tape"),
//     d3 = require("../");

// require("./inDelta");

// tape("geoLength(Point) returns zero", function(test) {
//   test.inDelta(d3.geoLength({type: "Point", coordinates: [0, 0]}), 0, 1e-6);
//   test.end();
// });

// tape("geoLength(MultiPoint) returns zero", function(test) {
//   test.inDelta(d3.geoLength({type: "MultiPoint", coordinates: [[0, 1], [2, 3]]}), 0, 1e-6);
//   test.end();
// });

// tape("geoLength(LineString) returns the sum of its great-arc segments", function(test) {
//   test.inDelta(d3.geoLength({type: "LineString", coordinates: [[-45, 0], [45, 0]]}), Math.PI / 2, 1e-6);
//   test.inDelta(d3.geoLength({type: "LineString", coordinates: [[-45, 0], [-30, 0], [-15, 0], [0, 0]]}), Math.PI / 4, 1e-6);
//   test.end();
// });

// tape("geoLength(MultiLineString) returns the sum of its great-arc segments", function(test) {
//   test.inDelta(d3.geoLength({type: "MultiLineString", coordinates: [[[-45, 0], [-30, 0]], [[-15, 0], [0, 0]]]}), Math.PI / 6, 1e-6);
//   test.end();
// });

// tape("geoLength(Polygon) returns the length of its perimeter", function(test) {
//   test.inDelta(d3.geoLength({type: "Polygon", coordinates: [[[0, 0], [3, 0], [3, 3], [0, 3], [0, 0]]]}), 0.157008, 1e-6);
//   test.end();
// });

// tape("geoLength(Polygon) returns the length of its perimeter, including holes", function(test) {
//   test.inDelta(d3.geoLength({type: "Polygon", coordinates: [[[0, 0], [3, 0], [3, 3], [0, 3], [0, 0]], [[1, 1], [2, 1], [2, 2], [1, 2], [1, 1]]]}), 0.209354, 1e-6);
//   test.end();
// });

// tape("geoLength(MultiPolygon) returns the summed length of the perimeters", function(test) {
//   test.inDelta(d3.geoLength({type: "MultiPolygon", coordinates: [[[[0, 0], [3, 0], [3, 3], [0, 3], [0, 0]]]]}), 0.157008, 1e-6);
//   test.inDelta(d3.geoLength({type: "MultiPolygon", coordinates: [[[[0, 0], [3, 0], [3, 3], [0, 3], [0, 0]]], [[[1, 1], [2, 1], [2, 2], [1, 2], [1, 1]]]]}), 0.209354, 1e-6);
//   test.end();
// });

// tape("geoLength(FeatureCollection) returns the sum of its features’ lengths", function(test) {
//   test.inDelta(d3.geoLength({
//     type: "FeatureCollection", features: [
//       {type: "Feature", geometry: {type: "LineString", coordinates: [[-45, 0], [0, 0]]}},
//       {type: "Feature", geometry: {type: "LineString", coordinates: [[0, 0], [45, 0]]}}
//     ]
//   }), Math.PI / 2, 1e-6);
//   test.end();
// });

// tape("geoLength(GeometryCollection) returns the sum of its geometries’ lengths", function(test) {
//   test.inDelta(d3.geoLength({
//     type: "GeometryCollection", geometries: [
//       {type: "GeometryCollection", geometries: [{type: "LineString", coordinates: [[-45, 0], [0, 0]]}]},
//       {type: "LineString", coordinates: [[0, 0], [45, 0]]}
//     ]
//   }), Math.PI / 2, 1e-6);
//   test.end();
// });
