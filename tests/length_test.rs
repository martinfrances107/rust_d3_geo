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
    fn point_returns_zero() {
        println!("geoLength(Point) returns zero");
        let length = LengthStream::calc(DataObject::Point {
            coordinate: Point { x: 0f64, y: 0f64 },
        });
        assert_eq!(length, 0f64);
    }

    #[test]
    fn multipoint_returns_zero() {
        println!("geoLength(Point) returns zero");
        let length = LengthStream::calc(DataObject::MultiPoint {
            coordinates: vec![Point { x: 0f64, y: 1f64 }, Point { x: 2f64, y: 3f64 }],
        });
        assert_eq!(length, 0f64);
    }

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
    fn multiline_string_returns_the_sum_of_great_arc_segments() {
        println!("geoLength(MultiLineString) returns the sum of its great-arc segments");
        assert!(in_delta(
            LengthStream::calc(DataObject::MultiLineString {
                coordinates: vec![
                    vec![Point { x: -45f64, y: 0f64 }, Point { x: -30f64, y: 0f64 }],
                    vec![Point { x: -15f64, y: 0f64 }, Point { x: 0f64, y: 0f64 }]
                ]
            }),
            PI / 6f64,
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

    #[test]
    fn multipolygon_returns_to_summed_length_of_perimeters() {
        println!("geoLength(MultiPolygon) returns the summed length of the perimeters");
        assert!(in_delta(
            LengthStream::calc(DataObject::MultiPolygon {
                coordinates: vec![vec![vec![
                    Point { x: 0f64, y: 0f64 },
                    Point { x: 3f64, y: 0f64 },
                    Point { x: 3f64, y: 3f64 },
                    Point { x: 0f64, y: 3f64 },
                    Point { x: 0f64, y: 0f64 }
                ]]]
            }),
            0.157008,
            1e-6
        ));
        assert!(in_delta(
            LengthStream::calc(DataObject::MultiPolygon {
                coordinates: vec![
                    vec![vec![
                        Point { x: 0f64, y: 0f64 },
                        Point { x: 3f64, y: 0f64 },
                        Point { x: 3f64, y: 3f64 },
                        Point { x: 0f64, y: 3f64 },
                        Point { x: 0f64, y: 0f64 }
                    ]],
                    vec![vec![
                        Point { x: 1f64, y: 1f64 },
                        Point { x: 2f64, y: 1f64 },
                        Point { x: 2f64, y: 2f64 },
                        Point { x: 1f64, y: 2f64 },
                        Point { x: 1f64, y: 1f64 },
                    ]]
                ]
            }),
            0.209354,
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
}
