#[cfg(not(tarpaulin_include))]
mod centroid {
    extern crate pretty_assertions;
    use geo::line_string;
    use geo::point;
    use geo::polygon;
    use geo::Geometry;
    use geo::GeometryCollection;
    use geo::LineString;
    use geo::MultiLineString;
    use geo::MultiPoint;
    use geo::MultiPolygon;
    use geo::Point;
    use geo::Polygon;
    use geo_types::Coord;

    use d3_geo_rs::centroid::Centroid;
    use d3_geo_rs::circle::generator::Generator as CircleGenerator;
    use d3_geo_rs::data_object::sphere::Sphere;
    use d3_geo_rs::in_delta::point as in_delta_point;

    #[test]
    fn of_a_point_is_itself() {
        println!("the centroid of a point is itself");
        assert!(in_delta_point(
            Centroid::default().calc(&Point::new(0f64, 0f64)),
            Point::new(0f64, 0f64),
            1e-6
        ));
        assert!(in_delta_point(
            Centroid::default().calc(&Point::new(1f64, 1f64)),
            Point::new(1f64, 1f64),
            1e-6
        ));
        assert!(in_delta_point(
            Centroid::default().calc(&Point::new(2f64, 3f64)),
            Point::new(2f64, 3f64),
            1e-6
        ));
        assert!(in_delta_point(
            Centroid::default().calc(&Point::new(-4f64, -5f64)),
            Point::new(-4f64, -5f64),
            1e-6
        ));
    }

    #[test]
    fn centroid_spherical_average() {
        println!(
            "the centroid of a set of points is the (spherical) average of its constituent members"
        );

        assert!(in_delta_point(
            Centroid::default().calc(&GeometryCollection(vec![
                Geometry::Point(Point::new(0f64, 0f64)),
                Geometry::Point(Point::new(1f64, 2f64))
            ])),
            Point::new(0.499847, 1.000038),
            1e-6
        ));

        assert!(in_delta_point(
            Centroid::default().calc(&MultiPoint(vec![
                Point::new(0f64, 0f64),
                Point::new(1f64, 2f64),
            ])),
            point!(
                x: 0.499847f64,
                y: 1.000038f64
            ),
            1e-6
        ));

        assert!(in_delta_point(
            Centroid::default().calc(&MultiPoint(vec![
                Point::new(179f64, 0f64),
                Point::new(-179f64, 0f64),
            ])),
            point!( x: 180f64, y: 0f64 ),
            1e-6
        ));
    }

    #[test]
    fn centroid_of_a_set_of_points_and_their_antipodes() {
        println!(
            "the centroid of a set of points and their antipodes is ambiguous"
        );
        let p1: Point<f64> = Centroid::default().calc(&MultiPoint(vec![
            Point::new(0_f64, 0_f64),
            Point::new(180_f64, 0_f64),
        ]));
        assert!(p1.x().is_nan());
        assert!(p1.y().is_nan());

        let p2 = Centroid::default().calc(&MultiPoint(vec![
            Point::new(0_f64, 0_f64),
            Point::new(90_f64, 0_f64),
            Point::new(180_f64, 0_f64),
            Point::new(-90_f64, 0_f64),
        ]));
        assert!(p2.x().is_nan());
        assert!(p2.y().is_nan());

        let p3 = Centroid::default().calc(&MultiPoint(vec![
            Point::new(0_f64, 0_f64),
            Point::new(0_f64, 90_f64),
            Point::new(180_f64, 0_f64),
            Point::new(0_f64, -90_f64),
        ]));
        assert!(p3.x().is_nan());
        assert!(p3.y().is_nan());
    }

    #[test]
    fn of_the_empty_set_of_points() {
        println!("the centroid of the empty set of points is ambiguous");
        let p: Point<f64> = Centroid::default().calc(&MultiPoint(vec![]));

        assert!(p.x().is_nan());
        assert!(p.y().is_nan());
    }

    #[test]
    fn line_string_is_the_spherical_average() {
        println!("the centroid of a line string is the (spherical) average of its constituent great arc segments");
        assert!(in_delta_point(
            Centroid::default().calc(&line_string![(
                x: 0.0f64,
                y: 0.0f64
            ), (
                x: 1.0f64,
                y: 0.0f64
            )]),
            Point::new(0.5f64, 0f64),
            1e-6
        ));

        assert!(in_delta_point(
            Centroid::default().calc(&line_string![
              ( x: 0.0f64, y: 0.0f64),
              ( x: 0f64, y: 90f64 )
            ]),
            Point::new(0f64, 45f64),
            1e-6
        ));

        assert!(in_delta_point(
            Centroid::default().calc(&line_string![
                ( x: 0f64, y: 0f64 ),
                ( x: 0f64, y: 45f64 ),
                ( x: 0f64, y: 90f64)
            ]),
            Point::new(0f64, 45f64),
            1e-6
        ));

        assert!(in_delta_point(
            Centroid::default().calc(&line_string![
                ( x: -1f64, y: -1f64 ), ( x: 1f64, y: 1f64 )
            ]),
            Point::new(0f64, 0f64),
            1e-6
        ));

        assert!(in_delta_point(
            Centroid::default().calc(&line_string![
                ( x: -60f64, y: -1f64 ),
                ( x: 60f64, y: 1f64 ),
            ]),
            Point::new(0f64, 0f64),
            1e-6
        ));

        assert!(in_delta_point(
            Centroid::default().calc(&line_string![
            ( x: 179f64, y: -1f64 ),
            ( x: -179f64, y: 1f64 ),
            ]),
            Point::new(180f64, 0f64),
            1e-6
        ));

        assert!(in_delta_point(
            Centroid::default().calc(&line_string![
                (x: -179f64, y: 0f64),
                (x: 0f64, y: 0f64),
                (x: 179f64, y: 0f64)
            ]),
            Point::new(0f64, 0f64),
            1e-6
        ));

        assert!(in_delta_point(
            Centroid::default().calc(&line_string![
             ( x: -180f64, y: -90f64 ),
             ( x: 0f64, y: 0f64 ),
             ( x: 0f64, y: 90f64 ),
            ]),
            Point::new(0f64, 0f64),
            1e-6
        ));
    }

    #[test]
    fn line_string_great_arc_segments() {
        println!("the centroid of a great arc from a point to its antipode is ambiguous");

        let p = Centroid::default().calc(&line_string![(
            x: 180.0f64,
            y: 0.0f64
        ), (
            x: 0.0f64,
            y: 0.0f64
        )]);
        assert!(p.x().is_nan());
        assert!(p.y().is_nan());

        let p = Centroid::default().calc(&MultiLineString(vec![
            line_string![(x: 0_f64, y: -90_f64), (x: 0_f64, y: 90_f64)],
        ]));
        assert!(p.x().is_nan());
        assert!(p.y().is_nan());
    }

    #[test]
    fn a_set_of_line_strings_is_the_spherical_average_of_its_great_arc_segments(
    ) {
        println!("the centroid of a set of line strings is the (spherical) average of its constituent great arc segments");
        let mls = MultiLineString(vec![LineString(vec![
            Coord { x: 0_f64, y: 0_f64 },
            Coord { x: 0_f64, y: 2_f64 },
        ])]);

        assert!(in_delta_point(
            Centroid::default().calc(&mls),
            Point::new(0_f64, 1_f64),
            1e-6_f64
        ));
    }

    #[test]
    fn a_line_of_zero_length_is_treated_as_points() {
        println!("a line of zero length is treated as points");
        let ls = LineString(vec![
            Coord { x: 1_f64, y: 1_f64 },
            Coord { x: 1_f64, y: 1_f64 },
        ]);

        assert!(in_delta_point(
            Centroid::default().calc(&ls),
            Point::new(1_f64, 1_f64),
            1e-6_f64
        ));

        let gc = GeometryCollection(vec![
            Geometry::Point(Point::new(0_f64, 0_f64)),
            Geometry::LineString(LineString(vec![
                Coord { x: 1_f64, y: 2_f64 },
                Coord { x: 1_f64, y: 2_f64 },
            ])),
        ]);

        assert!(in_delta_point(
            Centroid::default().calc(&gc),
            Point::new(0.666534_f64, 1.333408_f64),
            1e-6_f64
        ));
    }

    #[test]
    fn an_empty_polygon_with_non_zero_extent_is_treated_as_a_line() {
        println!("an empty polygon with non-zero extent is treated as a line");
        assert!(in_delta_point(
            Centroid::default().calc(&polygon![
            (
                x: 1.0f64,
                y: 1.0f64
            ),
            (
                x: 2.0f64,
                y: 1.0f64
            ),
            (
                x: 3.0f64,
                y: 1.0f64
            ),
            (
                x: 2.0f64,
                y: 1.0f64
            ),
            (
                x: 1.0f64,
                y: 1.0f64
            )
            ]),
            Point::new(2f64, 1.000076f64),
            1e-6
        ));
    }

    #[test]
    fn an_empty_polygon_with_non_zero_extent_is_treated_as_a_point() {
        println!("an empty polygon with zero extent is treated as a point");
        assert!(in_delta_point(
            Centroid::default().calc(&polygon![
            (
                x: 1.0f64,
                y: 1.0f64
            ),
            (
                x: 1.0f64,
                y: 1.0f64
            ),
            (
                x: 1.0f64,
                y: 1.0f64
            ),
            (
                x: 1.0f64,
                y: 1.0f64
            ),
            ]),
            Point::new(1_f64, 1_f64),
            1e-6
        ));

        let gc = GeometryCollection(vec![
            Geometry::Point(Point::new(0_f64, 0_f64)),
            Geometry::Polygon(Polygon::new(
                line_string![(x:1_f64,y: 2_f64), (x:1_f64, y:2_f64), (x:1_f64, y:2_f64), (x:1_f64, y:2_f64)],
                vec![],
            )),
        ]);
        assert!(in_delta_point(
            Centroid::default().calc(&gc),
            Point::new(0.799907, 1.600077),
            1e-6
        ));
    }

    #[test]
    fn of_the_equator_is_ambiguous() {
        println!("the centroid of the equator is ambiguous");
        let c = Centroid::default().calc(&line_string![
        (
            x: 0f64,
            y: 0f64
        ),
        (
            x: 120.0f64,
            y: 0.0f64
        ),
        (
            x: -120f64,
            y: 0f64
        ),
        (
            x: 0.0f64,
            y: 0.0f64
        ),
        ]);
        assert!(c.x().is_nan());
        assert!(c.y().is_nan());
    }

    //     // tape("the centroid of a polygon is the (spherical) average of its surface", function(test) {
    //     //   test.inDelta(d3.geoCentroid({type: "Polygon", coordinates: [[[0, -90], [0, 0], [0, 90], [1, 0], [0, -90]]]}), [0.5, 0], 1e-6);
    //     //   test.inDelta(d3.geoCentroid({type: "Polygon", coordinates: [array.range(-180, 180 + 1 / 2, 1).map(function(x) { return [x, -60]; })]})[1], -90, 1e-6);
    //     //   test.inDelta(d3.geoCentroid({type: "Polygon", coordinates: [[[0, -10], [0, 10], [10, 10], [10, -10], [0, -10]]]}), [5, 0], 1e-6);
    //     //   test.end();
    //     // });

    #[test]
    fn a_set_of_polygons_is_the_spherical_average_of_its_surface() {
        println!("the centroid of a set of polygons is the (spherical) average of its surface");
        let p45 = CircleGenerator::default()
            .radius_set(45_f64)
            .center_set(&Coord {
                x: 90_f64,
                y: 0_f64,
            })
            .circle();
        let p60 = CircleGenerator::default()
            .radius_set(60_f64)
            .center_set(&Coord {
                x: -90_f64,
                y: 0_f64,
            })
            .circle();
        assert!(in_delta_point(
            Centroid::default()
                .calc(&Geometry::MultiPolygon(MultiPolygon(vec![p45, p60]))),
            Point::new(-90_f64, 0_f64),
            1e-6
        ));
    }

    //     // tape("the centroid of a lune is the (spherical) average of its surface", function(test) {
    //     //   test.inDelta(d3.geoCentroid({type: "Polygon", coordinates: [[[0, -90], [0, 0], [0, 90], [1, 0], [0, -90]]]}), [0.5, 0], 1e-6);
    //     //   test.end();
    //     // });

    //     // tape("the centroid of a small circle is its center: 5°", function(test) {
    //     //   test.inDelta(d3.geoCentroid(d3.geoCircle().radius(5).center([30, 45])()), [30, 45], 1e-6);
    //     //   test.end();
    //     // });

    //     // tape("the centroid of a small circle is its center: 135°", function(test) {
    //     //   test.inDelta(d3.geoCentroid(d3.geoCircle().radius(135).center([30, 45])()), [30, 45], 1e-6);
    //     //   test.end();
    //     // });

    //     // tape("the centroid of a small circle is its center: South Pole", function(test) {
    //     //   test.equal(d3.geoCentroid({type: "Polygon", coordinates: [array.range(-180, 180 + 1 / 2, 1).map(function(x) { return [x, -60]; })]})[1], -90);
    //     //   test.end();
    //     // });

    //     // tape("the centroid of a small circle is its center: equator", function(test) {
    //     //   test.inDelta(d3.geoCentroid({type: "Polygon", coordinates: [[[0, -10], [0, 10], [10, 10], [10, -10], [0, -10]]]}), [5, 0], 1e-6);
    //     //   test.end();
    //     // });

    //     // tape("the centroid of a small circle is its center: equator with coincident points", function(test) {
    //     //   test.inDelta(d3.geoCentroid({type: "Polygon", coordinates: [[[0, -10], [0, 10], [0, 10], [10, 10], [10, -10], [0, -10]]]}), [5, 0], 1e-6);
    //     //   test.end();
    //     // });

    //     // tape("the centroid of a small circle is its center: other", function(test) {
    //     //   test.inDelta(d3.geoCentroid({type: "Polygon", coordinates: [[[-180, 0], [-180, 10], [-179, 10], [-179, 0], [-180, 0]]]}), [-179.5, 4.987448], 1e-6);
    //     //   test.end();
    //     // });

    //     // tape("the centroid of a small circle is its center: concentric rings", function(test) {
    //     //   var circle = d3.geoCircle().center([0, 45]),
    //     //       coordinates = circle.radius(60)().coordinates;
    //     //   coordinates.push(circle.radius(45)().coordinates[0].reverse());
    //     //   test.inDelta(d3.geoCentroid({type: "Polygon", coordinates: coordinates}), [0, 45], 1e-6);
    //     //   test.end();
    //     // });

    //     // tape("the centroid of a spherical square on the equator", function(test) {
    //     //   test.inDelta(d3.geoCentroid({type: "Polygon", coordinates: [[[0, -10], [0, 10], [10, 10], [10, -10], [0, -10]]]}), [5, 0], 1e-6);
    //     //   test.end();
    //     // });
    #[test]
    fn of_a_spherical_square_on_the_equator() {
        println!("the centroid of a spherical square on the equator");
        assert!(in_delta_point(
            Centroid::default().calc(&polygon![
            (
                x: 0_f64,
                y: -10_f64
            ),
            (
                x: 0_f64,
                y: 10_f64
            ),
            (
                x: 10_f64,
                y: 10_f64
            ),
            (
                x: 10_f64,
                y: -10_f64
            ),
            (
                x: 0_f64,
                y: -10_f64
            )
            ]),
            Point::new(5_f64, 0_f64),
            1e-6
        ));
    }

    #[test]
    fn of_a_spherical_square_touching_the_antimeridian() {
        println!(
            "the centroid of a spherical square touching the antimeridian"
        );
        assert!(in_delta_point(
            Centroid::default().calc(&polygon![
            (
                x: -180_f64,
                y: 0_f64
            ),
            (
                x: -180_f64,
                y: 10.0f64
            ),
            (
                x: -179_f64,
                y: 10_f64
            ),
            (
                x: -179_f64,
                y: 0_f64
            ),
            (
                x: 180_f64,
                y: 0_f64
            )
            ]),
            Point::new(-179.5_f64, 4.987448_f64),
            1e-6
        ));
    }

    #[test]
    fn concentric_rings() {
        println!("concentric rings");
        let mut cg = CircleGenerator::default();
        cg.center_set(&Coord {
            x: 0_f64,
            y: 45_f64,
        });

        let l60 = cg.clone().radius_set(60_f64).circle().exterior().clone();

        let l45 = cg.radius_set(45_f64).circle().exterior().clone();
        let rev_vec = l45.into_iter().rev().collect();
        let l45_rev = LineString(rev_vec);

        let polygon = Polygon::new(l60, vec![l45_rev]);
        assert!(in_delta_point(
            Centroid::default().calc(&polygon),
            Point::new(0_f64, 45_f64),
            1e-6
        ));
    }

    #[test]
    fn of_a_sphere_is_ambiguous() {
        println!("the centroid of a sphere is ambiguous");
        let point: Point<f64> = Centroid::default().calc(&Sphere::default());
        assert!(point.x().is_nan());
        assert!(point.y().is_nan());
    }
    //     // tape("the centroid of a feature is the centroid of its constituent geometry", function(test) {
    //     //   test.inDelta(d3.geoCentroid({type: "Feature", geometry: {type: "LineString", coordinates: [[1, 1], [1, 1]]}}), [1, 1], 1e-6);
    //     //   test.inDelta(d3.geoCentroid({type: "Feature", geometry: {type: "Point", coordinates: [1, 1]}}), [1, 1], 1e-6);
    //     //   test.inDelta(d3.geoCentroid({type: "Feature", geometry: {type: "Polygon", coordinates: [[[0, -90], [0, 0], [0, 90], [1, 0], [0, -90]]]}}), [0.5, 0], 1e-6);
    //     //   test.end();
    //     // });

    //     // tape("the centroid of a feature collection is the centroid of its constituent geometry", function(test) {
    //     //   test.inDelta(d3.geoCentroid({type: "FeatureCollection", features: [
    //     //     {type: "Feature", geometry: {type: "LineString", coordinates: [[179, 0], [180, 0]]}},
    //     //     {type: "Feature", geometry: {type: "Point", coordinates: [0, 0]}}
    //     //   ]}), [179.5, 0], 1e-6);
    //     //   test.end();
    //     // });

    #[test]
    fn geometry_collection_non_empty_line_string_and_point() {
        println!(
            "the centroid of a non-empty line string and a point only considers the line string"
        );

        let data_object = GeometryCollection(vec![
            Geometry::LineString(
                line_string![(x:179_f64, y:0_f64),(x:180_f64, y:0_f64) ],
            ),
            Geometry::Point(point!(x:0_f64, y: 0_f64)),
        ]);
        let centroid = Centroid::default().calc(&data_object);
        assert!(in_delta_point(centroid, (179.5_f64, 0_f64).into(), 1e-6));
    }

    //     // tape("the centroid of a non-empty polygon, a non-empty line string and a point only considers the polygon", function(test) {
    //     //   test.inDelta(d3.geoCentroid({type: "GeometryCollection", geometries: [
    //     //     {type: "Polygon", coordinates: [[[-180, 0], [-180, 1], [-179, 1], [-179, 0], [-180, 0]]]},
    //     //     {type: "LineString", coordinates: [[179, 0], [180, 0]]},
    //     //     {type: "Point", coordinates: [0, 0]}
    //     //   ]}), [-179.5, 0.500006], 1e-6);
    //     //   test.inDelta(d3.geoCentroid({type: "GeometryCollection", geometries: [
    //     //     {type: "Point", coordinates: [0, 0]},
    //     //     {type: "LineString", coordinates: [[179, 0], [180, 0]]},
    //     //     {type: "Polygon", coordinates: [[[-180, 0], [-180, 1], [-179, 1], [-179, 0], [-180, 0]]]}
    //     //   ]}), [-179.5, 0.500006], 1e-6);
    //     //   test.end();
    //     // });
    #[test]
    fn of_a_non_empty_polygon_a_non_empty_line_string_and_a_point_only_considers_the_polygon(
    ) {
        println!(
            "the centroid of a non-empty polygon, a non-empty line string and a point only considers the polygon"
        );

        let data_object = GeometryCollection(vec![
            Geometry::Polygon(Polygon::new(
                line_string![(x: -180_f64, y: 0_f64), (x: -180_f64, y:1_f64), (x: -179_f64, y: 1_f64), (x: -179_f64, y: 0_f64),
                    (x: -180_f64, y: 0_f64)
                ],
                vec![],
            )),
            Geometry::LineString(
                line_string![(x:179_f64, y:0_f64),(x:180_f64, y:0_f64) ],
            ),
            Geometry::Point(point!(x:0_f64, y: 0_f64)),
        ]);
        let centroid = Centroid::default().calc(&data_object);
        assert!(in_delta_point(
            centroid,
            (-179.5_f64, 0.500006_f64).into(),
            1e-6
        ));

        let data_object = GeometryCollection(vec![
            Geometry::Point(point!(x:0_f64, y: 0_f64)),
            Geometry::LineString(
                line_string![(x:179_f64, y:0_f64),(x:180_f64, y:0_f64) ],
            ),
            Geometry::Polygon(Polygon::new(
                line_string![(x: -180_f64, y: 0_f64), (x: -180_f64, y:1_f64), (x: -179_f64, y: 1_f64), (x: -179_f64, y: 0_f64),
                    (x: -180_f64, y: 0_f64)
                ],
                vec![],
            )),
        ]);
        let centroid = Centroid::default().calc(&data_object);
        assert!(in_delta_point(
            centroid,
            (-179.5_f64, 0.500006_f64).into(),
            1e-6
        ));
    }

    //     // tape("the centroid of the sphere and a point is the point", function(test) {
    //     //   test.deepEqual(d3.geoCentroid({type: "GeometryCollection", geometries: [
    //     //     {type: "Sphere"},
    //     //     {type: "Point", coordinates: [0, 0]}
    //     //   ]}), [0, 0]);
    //     //   test.deepEqual(d3.geoCentroid({type: "GeometryCollection", geometries: [
    //     //     {type: "Point", coordinates: [0, 0]},
    //     //     {type: "Sphere"}
    //     //   ]}), [0, 0]);
    //     //   test.end();
    //     // });

    //     // tape("the centroid of a detailed feature is correct", function(test) {
    //     //   var ny = require("./data/ny.json");
    //     //   test.inDelta(d3.geoCentroid(ny), [-73.93079, 40.69447], 1e-5);
    //     //   test.end();
    //     // });
}
