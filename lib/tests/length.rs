#[cfg(not(tarpaulin_include))]
mod length {
    extern crate pretty_assertions;

    use core::f64::consts::PI;

    use geo::line_string;
    use geo::polygon;
    use geo::LineString;
    use geo::MultiLineString;
    use geo::MultiPoint;
    use geo::MultiPolygon;
    use geo::Point;
    use geo::Polygon;
    use pretty_assertions::assert_eq;

    use d3_geo_rs::in_delta::in_delta;
    use d3_geo_rs::length::Stream as LengthStream;

    #[test]
    fn point_returns_zero() {
        println!("geoLength(Point) returns zero");
        let length = LengthStream::calc(&Point::new(0f64, 0f64));
        assert_eq!(length, 0f64);
    }

    #[test]
    fn multipoint_returns_zero() {
        println!("geoLength(Point) returns zero");
        let length = LengthStream::calc(&MultiPoint(vec![
            Point::new(0f64, 1f64),
            Point::new(2f64, 3f64),
        ]));
        assert_eq!(length, 0f64);
    }

    #[test]
    fn line_string_great_arc_segments() {
        println!("geoLength(LineString) returns the sum of its great-arc segments");
        let ls: LineString<f64> = line_string![(x: -45f64,y:  0f64 ), ( x:45f64,y:  0f64 )];
        assert!(in_delta(LengthStream::calc(&ls), PI / 2f64, 1e-6));

        let ls: LineString<f64> =
            vec![(-45f64, 0f64), (-30f64, 0f64), (-15f64, 0f64), (0f64, 0f64)].into();
        assert!(in_delta(LengthStream::calc(&ls), PI / 4f64, 1e-6));
    }

    #[test]
    fn multiline_string_returns_the_sum_of_great_arc_segments() {
        println!("geoLength(MultiLineString) returns the sum of its great-arc segments");
        assert!(in_delta(
            LengthStream::calc(&MultiLineString(vec![
                line_string![(x: -45f64, y: 0f64) , (x: -30f64, y: 0f64)],
                line_string![(x: -15f64, y: 0f64) , (x: 0f64, y:0f64 )]
            ])),
            PI / 6f64,
            1e-6
        ));
    }

    #[test]
    fn polygon_length_of_perimeter() {
        println!("geoLength(Polygon) returns the length of its perimeter");
        assert!(in_delta(
            LengthStream::calc(&Polygon::new(
                line_string![
                    ( x:0f64, y:0f64 ),
                    ( x:3f64, y:0f64 ),
                    ( x:3f64, y:3f64 ),
                    ( x:0f64, y:3f64 ),
                    ( x:0f64, y:0f64 )
                ],
                vec![]
            )),
            0.157008f64,
            1e-6f64
        ));
    }

    #[test]
    fn polygon_length_of_perimeter_including_holes() {
        println!("geoLength(Polygon) returns the length of its perimeter, including holes");
        assert!(in_delta(
            LengthStream::calc(&Polygon::new(
                line_string![
                    (x: 0f64, y:0f64 ),
                    (x: 3f64, y:0f64 ),
                    (x: 3f64, y:3f64 ),
                    (x: 0f64, y:3f64 ),
                    (x: 0f64, y:0f64 )
                ],
                vec![line_string![
                    (x: 1f64, y:1f64 ),
                    (x: 2f64, y:1f64 ),
                    (x: 2f64, y:2f64 ),
                    (x: 1f64, y:2f64 ),
                    (x: 1f64, y:1f64 )
                ]]
            )),
            0.209354f64,
            1e-6f64
        ));
    }

    #[test]
    fn multipolygon_returns_to_summed_length_of_perimeters() {
        println!("geoLength(MultiPolygon) returns the summed length of the perimeters");
        assert!(in_delta(
            LengthStream::calc(&MultiPolygon(vec![polygon![
                ( x: 0f64, y:0f64 ),
                ( x: 3f64, y:0f64 ),
                ( x: 3f64, y:3f64 ),
                ( x: 0f64, y:3f64 ),
                ( x: 0f64, y:0f64 )
            ]])),
            0.157008,
            1e-6
        ));
        assert!(in_delta(
            LengthStream::calc(&MultiPolygon(vec![
                polygon![
                    (x:  0f64, y: 0f64 ),
                    (x:  3f64, y: 0f64 ),
                    (x:  3f64, y: 3f64 ),
                    (x:  0f64, y: 3f64 ),
                    (x:  0f64, y: 0f64 )
                ],
                polygon![
                    (x: 1f64, y: 1f64 ),
                    (x: 2f64, y: 1f64 ),
                    (x: 2f64, y: 2f64 ),
                    (x: 1f64, y: 2f64 ),
                    (x: 1f64, y: 1f64 ),
                ]
            ])),
            0.209354,
            1e-6
        ));
    }

    // #[test]
    // fn fc_line_string_the_sum_of_its_features() {
    //     println!("geoLength(FeatureCollection) returns the sum of its features’ lengths");
    //     assert!(in_delta(
    //         LengthStream::calc(&FeatureCollection(vec![FeaturesStruct {
    //             properties: Vec::new(),
    //             geometry: vec![
    //                 Geometry::LineString(line_string![
    //                     (x: -45f64, y: 0f64 ),
    //                     (x: 0f64, y: 0f64 )
    //                 ]),
    //                 Geometry::LineString(line_string![
    //                     ( x:0f64, y: 0f64 ),
    //                     ( x:45f64, y: 0f64 )
    //                 ]),
    //             ],
    //         }])),
    //         PI / 2f64,
    //         1e-6
    //     ));
    // }
}
